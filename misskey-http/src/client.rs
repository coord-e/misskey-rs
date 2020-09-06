use std::path::Path;

use crate::error::{Error, Result};

use common_multipart_rfc7578::client::{multipart, Error as MultipartError};
use futures::compat::Compat01As03;
use futures::future::BoxFuture;
use futures::io::AsyncReadExt;
use isahc::http;
#[cfg(feature = "inspect-contents")]
use log::debug;
use mime::Mime;
use misskey_core::model::ApiResult;
use misskey_core::{Client, Request, UploadFileRequest};
use serde::Serialize;
use serde_json::value::{self, Value};
use url::Url;

pub mod builder;

pub struct HttpClient {
    url: Url,
    token: Option<String>,
    client: isahc::HttpClient,
}

impl HttpClient {
    pub fn new(url: Url, token: Option<String>) -> Result<Self> {
        Ok(HttpClient {
            url,
            token,
            client: isahc::HttpClient::new()?,
        })
    }

    fn set_api_key<R: Request>(
        &self,
        request: R,
    ) -> std::result::Result<impl Serialize, serde_json::Error> {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum ValueOrRequest<R> {
            Value(Value),
            Request(R),
        }

        if let Some(token) = &self.token {
            let mut value = value::to_value(request)?;

            let obj = value.as_object_mut().expect("Request must be an object");
            if obj
                .insert("i".to_string(), Value::String(token.to_owned()))
                .is_some()
            {
                panic!("Request must not have 'i' key");
            }

            Ok(ValueOrRequest::Value(value))
        } else {
            Ok(ValueOrRequest::Request(request))
        }
    }

    pub async fn request_with_file<R: UploadFileRequest>(
        &self,
        request: R,
        type_: Mime,
        file_name: String,
        path: impl AsRef<Path>,
    ) -> Result<ApiResult<R::Response>> {
        let url = self
            .url
            .join(R::ENDPOINT)
            .expect("Request::ENDPOINT must be a fragment of valid URL");

        // limit the use of `R` value to the outside of `async`
        // in order not to require `Send` on `R`
        let value = self.set_api_key(request).and_then(value::to_value);

        async move {
            let value = value?;

            #[cfg(feature = "inspect-contents")]
            debug!(
                "sending request to {} with {} ({}) file: {}",
                url,
                path.as_ref().display(),
                type_,
                value
            );

            let mut form = multipart::Form::default();

            // TODO: Can't we just take `AsyncRead` or `Read` and use it directly?
            let read = std::fs::File::open(path)?;
            form.add_reader_file_with_mime("file", read, file_name, type_);

            let obj = value.as_object().expect("Request must be an object");
            for (k, v) in obj {
                let v = v
                    .as_str()
                    .expect("UploadFileRequest must be an object that all values are string");
                form.add_text(k.to_owned(), v.to_owned());
            }

            let content_type = form.content_type();

            use futures::stream::TryStreamExt;
            let stream = Compat01As03::new(multipart::Body::from(form)).map_err(|e| match e {
                MultipartError::HeaderWrite(e) => e,
                MultipartError::BoundaryWrite(e) => e,
                MultipartError::ContentRead(e) => e,
            });
            let body = isahc::Body::from_reader(async_dup::Mutex::new(stream.into_async_read()));

            use isahc::http::header::CONTENT_TYPE;
            let response = self
                .client
                .send_async(
                    // TODO: uncomfortable conversion from `Url` to `Uri`
                    http::Request::post(url.into_string())
                        .header(CONTENT_TYPE, content_type)
                        .body(body)
                        .unwrap(),
                )
                .await?;

            response_to_result::<R>(response).await
        }
        .await
    }
}

impl Client for HttpClient {
    type Error = Error;

    fn request<'a, R>(&'a self, request: R) -> BoxFuture<'a, Result<ApiResult<R::Response>>>
    where
        R: Request + 'a,
    {
        let url = self
            .url
            .join(R::ENDPOINT)
            .expect("Request::ENDPOINT must be a fragment of valid URL");

        // limit the use of `R` value to the outside of `async`
        // in order not to require `Send` on `R`
        let body = self
            .set_api_key(request)
            .and_then(|b| serde_json::to_vec(&b));

        Box::pin(async move {
            let body = body?;

            #[cfg(feature = "inspect-contents")]
            debug!(
                "sending request to {}: {}",
                url,
                String::from_utf8_lossy(&body)
            );

            use isahc::http::header::CONTENT_TYPE;
            let response = self
                .client
                .send_async(
                    // TODO: uncomfortable conversion from `Url` to `Uri`
                    http::Request::post(url.to_string())
                        .header(CONTENT_TYPE, "application/json")
                        .body(body)
                        .unwrap(),
                )
                .await?;

            response_to_result::<R>(response).await
        })
    }
}

async fn response_to_result<R: Request>(
    response: http::Response<isahc::Body>,
) -> Result<ApiResult<R::Response>> {
    let status = response.status();
    let mut bytes = Vec::new();
    response.into_body().read_to_end(&mut bytes).await?;

    #[cfg(feature = "inspect-contents")]
    debug!(
        "got response ({}) from {}: {}",
        status,
        R::ENDPOINT,
        String::from_utf8_lossy(&bytes)
    );

    let json_bytes = if bytes.is_empty() {
        b"null".as_ref()
    } else {
        bytes.as_ref()
    };

    if status.is_success() {
        // Limit response to `ApiResult::Ok` branch to get informative error message
        // when our model does not match the response.
        Ok(ApiResult::Ok(serde_json::from_slice(json_bytes)?))
    } else {
        Ok(serde_json::from_slice(json_bytes)?)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use super::HttpClient;

    use misskey_core::Client;
    use url::Url;
    use uuid::Uuid;

    static INIT_LOGGER: Once = Once::new();

    fn test_client() -> HttpClient {
        INIT_LOGGER.call_once(env_logger::init);

        let url = std::env::var("TEST_API_URL").unwrap();
        let token = std::env::var("TEST_USER_TOKEN").unwrap();
        HttpClient::new(Url::parse(&url).unwrap(), Some(token)).unwrap()
    }

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<HttpClient>();
    }

    #[test]
    fn test_sync() {
        fn assert_send<T: Sync>() {}
        assert_send::<HttpClient>();
    }

    #[tokio::test]
    async fn tokio_request() {
        let client = test_client();
        client
            .request(
                misskey_api::endpoint::notes::create::Request::builder()
                    .text("hi")
                    .build(),
            )
            .await
            .unwrap()
            .unwrap();
    }

    #[async_std::test]
    async fn async_std_request() {
        let client = test_client();
        client
            .request(
                misskey_api::endpoint::notes::create::Request::builder()
                    .text("hi")
                    .build(),
            )
            .await
            .unwrap()
            .unwrap();
    }

    fn write_to_temp_file(data: impl AsRef<[u8]>) -> std::path::PathBuf {
        let tmp_name = Uuid::new_v4().to_simple().to_string();
        let path = std::env::temp_dir().join(tmp_name);
        {
            use std::{fs::File, io::Write};
            let mut file = File::create(&path).unwrap();
            file.write_all(data.as_ref()).unwrap();
            file.sync_all().unwrap();
        }
        path
    }

    #[tokio::test]
    async fn tokio_request_with_file() {
        let client = test_client();
        let path = write_to_temp_file("test");

        client
            .request_with_file(
                misskey_api::endpoint::drive::files::create::Request::default(),
                mime::TEXT_PLAIN,
                "test.txt".to_string(),
                path,
            )
            .await
            .unwrap()
            .unwrap();
    }

    #[async_std::test]
    async fn async_std_request_with_file() {
        let client = test_client();
        let path = write_to_temp_file("test");

        client
            .request_with_file(
                misskey_api::endpoint::drive::files::create::Request::default(),
                mime::TEXT_PLAIN,
                "test.txt".to_string(),
                path,
            )
            .await
            .unwrap()
            .unwrap();
    }
}
