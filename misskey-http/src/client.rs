use crate::error::{Error, Result};

use mime::Mime;
use misskey_core::model::ApiResult;
use misskey_core::{Client, Request, RequestWithFile};
use reqwest::header::HeaderMap;
use serde_json::value::{self, Value};
use url::Url;

pub mod builder;

pub struct HttpClient {
    url: Url,
    token: Option<String>,
    client: reqwest::Client,
    additional_headers: HeaderMap,
}

impl HttpClient {
    pub fn new(url: Url, token: Option<String>) -> Self {
        HttpClient {
            url,
            token,
            client: reqwest::Client::new(),
            additional_headers: HeaderMap::new(),
        }
    }

    pub async fn request_with_file<R: RequestWithFile + Send>(
        &mut self,
        request: R,
        type_: Mime,
        file_name: String,
        data: Vec<u8>,
    ) -> Result<ApiResult<R::Response>> {
        let url = self
            .url
            .join(R::ENDPOINT)
            .expect("Request::ENDPOINT must be a fragment of valid URL");

        let mut form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(data)
                .file_name(file_name)
                .mime_str(type_.as_ref())
                .unwrap(),
        );

        let value = if let Some(token) = &self.token {
            to_json_with_api_key(request, token)?
        } else {
            value::to_value(request)?
        };

        let obj = value.as_object().expect("Request must be an object");
        for (k, v) in obj {
            let v = v
                .as_str()
                .expect("RequestWithFile must be an object that all values are string");
            form = form.text(k.to_owned(), v.to_owned());
        }

        use reqwest::header::CONTENT_TYPE;
        let response = self
            .client
            .post(url)
            .multipart(form)
            .header(CONTENT_TYPE, "multipart/form-data")
            .headers(self.additional_headers.clone())
            .send()
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

        use reqwest::header::CONTENT_TYPE;
        let response = self
            .client
            .post(url)
            .body(body)
            .header(CONTENT_TYPE, "application/json")
            .headers(self.additional_headers.clone())
            .send()
            .await?;

        response_to_result::<R>(response).await
    }
}

async fn response_to_result<R: Request>(
    response: reqwest::Response,
) -> Result<ApiResult<R::Response>> {
    let status = response.status();
    let bytes = response.bytes().await?;
    let json_bytes = if bytes.is_empty() {
        b"null"
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
