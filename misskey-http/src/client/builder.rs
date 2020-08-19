use crate::client::HttpClient;

use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use url::Url;

pub struct HttpClientBuilder {
    url: Url,
    token: Option<String>,
    additional_headers: HeaderMap,
}

impl HttpClientBuilder {
    pub fn new(url: Url) -> Self {
        HttpClientBuilder {
            url,
            token: None,
            additional_headers: HeaderMap::new(),
        }
    }

    pub fn header<'a, K: IntoHeaderName>(&'a mut self, key: K, value: HeaderValue) -> &'a mut Self {
        self.additional_headers.insert(key, value);
        self
    }

    pub fn token<'a, S: Into<String>>(&'a mut self, token: S) -> &'a mut Self {
        self.token = Some(token.into());
        self
    }

    pub fn build(&self) -> HttpClient {
        HttpClient {
            url: self.url.clone(),
            token: self.token.clone(),
            additional_headers: self.additional_headers.clone(),
            client: reqwest::Client::new(),
        }
    }
}
