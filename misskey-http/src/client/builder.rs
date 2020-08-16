use crate::client::HttpClient;
use crate::error::Result;

use misskey::client::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use url::Url;

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
