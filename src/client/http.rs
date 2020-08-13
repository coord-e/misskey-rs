use crate::api::ApiRequest;
use crate::error::Result;

use super::{auth, Client};

use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use url::Url;

pub struct HttpClient {
    url: Url,
    api_key: String,
    client: reqwest::Client,
    additional_headers: HeaderMap,
}

impl HttpClient {
    pub fn new(url: Url, api_key: String) -> HttpClient {
        let client = reqwest::Client::new();

        HttpClient {
            url,
            api_key,
            client,
            additional_headers: HeaderMap::new(),
        }
    }

    pub fn add_header<K: IntoHeaderName>(&mut self, key: K, value: HeaderValue) {
        self.additional_headers.insert(key, value);
    }
}

#[async_trait::async_trait]
impl Client for HttpClient {
    async fn request<R: ApiRequest + Send>(&mut self, request: R) -> Result<R::Response> {
        let url = self
            .url
            .join(R::ENDPOINT)
            .expect("ApiRequest::ENDPOINT must be a fragment of valid URL");

        let body = auth::to_json_with_api_key(request, &self.api_key)?.to_string();

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
