use crate::api::ApiRequest;

use super::{Client, ClientBuilder};

use err_derive::Error;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use serde_json::value::{self, Value};
use url::Url;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "network error: {}", _0)]
    Network(#[error(source)] reqwest::Error),
    #[error(display = "JSON error: {}", _0)]
    Json(#[error(source)] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct HttpClientBuilder {
    url: Url,
    token: Option<String>,
    additional_headers: HeaderMap,
}

#[async_trait::async_trait]
impl ClientBuilder for HttpClientBuilder {
    type Client = HttpClient;

    fn new(url: Url) -> Self {
        HttpClientBuilder {
            url,
            token: None,
            additional_headers: HeaderMap::new(),
        }
    }

    fn token<'a, S: Into<String>>(&'a mut self, token: S) -> &'a mut Self {
        self.token = Some(token.into());
        self
    }

    async fn build(&self) -> Result<HttpClient> {
        Ok(HttpClient {
            url: self.url.clone(),
            token: self.token.clone(),
            additional_headers: self.additional_headers.clone(),
            client: reqwest::Client::new(),
        })
    }
}

impl HttpClientBuilder {
    pub fn header<'a, K: IntoHeaderName>(&'a mut self, key: K, value: HeaderValue) -> &'a mut Self {
        self.additional_headers.insert(key, value);
        self
    }
}

pub struct HttpClient {
    url: Url,
    token: Option<String>,
    client: reqwest::Client,
    additional_headers: HeaderMap,
}

#[async_trait::async_trait]
impl Client for HttpClient {
    type Error = Error;

    async fn request<R: ApiRequest + Send>(&mut self, request: R) -> Result<R::Response> {
        let url = self
            .url
            .join(R::ENDPOINT)
            .expect("ApiRequest::ENDPOINT must be a fragment of valid URL");

        let body = if let Some(token) = &self.token {
            serde_json::to_vec(&to_json_with_api_key(request, token)?)?
        } else {
            serde_json::to_vec(&request)?
        };

        use reqwest::header::CONTENT_TYPE;
        let response_bytes = self
            .client
            .post(url)
            .body(body)
            .header(CONTENT_TYPE, "application/json")
            .headers(self.additional_headers.clone())
            .send()
            .await?
            .bytes()
            .await?;

        let json_bytes = if response_bytes.is_empty() {
            b"null"
        } else {
            response_bytes.as_ref()
        };
        let response = serde_json::from_slice(json_bytes)?;
        Ok(response)
    }
}

pub fn to_json_with_api_key<T: ApiRequest>(data: T, api_key: &str) -> Result<Value> {
    let mut value = value::to_value(data)?;

    let obj = value.as_object_mut().expect("ApiRequest must be an object");
    if obj
        .insert("i".to_string(), Value::String(api_key.to_string()))
        .is_some()
    {
        panic!("ApiRequest must not have 'i' key");
    }

    Ok(value)
}
