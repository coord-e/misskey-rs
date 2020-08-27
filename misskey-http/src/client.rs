use std::path::Path;

use crate::error::{Error, Result};

use common_multipart_rfc7578::client::{multipart, Error as MultipartError};
use futures::compat::Compat01As03;
use futures::io::AsyncReadExt;
use isahc::http;
#[cfg(feature = "inspect-contents")]
use log::debug;
use mime::Mime;
use misskey_core::model::ApiResult;
use misskey_core::{Client, Request, RequestWithFile};
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

    pub async fn request_with_file<R: RequestWithFile + Send>(
        &mut self,
        request: R,
        type_: Mime,
        file_name: String,
        path: impl AsRef<Path>,
    ) -> Result<ApiResult<R::Response>> {
        let url = self
            .url
            .join(R::ENDPOINT)
            .expect("Request::ENDPOINT must be a fragment of valid URL");

        let value = if let Some(token) = &self.token {
            to_json_with_api_key(request, token)?
        } else {
            value::to_value(request)?
        };

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
                .expect("RequestWithFile must be an object that all values are string");
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
}

#[async_trait::async_trait]
impl Client for HttpClient {
    type Error = Error;

    async fn request<R: Request + Send>(&mut self, request: R) -> Result<ApiResult<R::Response>> {
        let url = self
            .url
            .join(R::ENDPOINT)
            .expect("Request::ENDPOINT must be a fragment of valid URL");

        let body = if let Some(token) = &self.token {
            serde_json::to_vec(&to_json_with_api_key(request, token)?)?
        } else {
            serde_json::to_vec(&request)?
        };

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

fn to_json_with_api_key<T: Request>(data: T, api_key: &str) -> Result<Value> {
    let mut value = value::to_value(data)?;

    let obj = value.as_object_mut().expect("Request must be an object");
    if obj
        .insert("i".to_string(), Value::String(api_key.to_string()))
        .is_some()
    {
        panic!("Request must not have 'i' key");
    }

    Ok(value)
}
