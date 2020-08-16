use crate::client::WebSocketClient;
use crate::error::Result;

use misskey::ClientBuilder;
use url::Url;

pub struct WebSocketClientBuilder {
    url: Url,
    token: Option<String>,
}

#[async_trait::async_trait]
impl ClientBuilder for WebSocketClientBuilder {
    type Client = WebSocketClient;

    fn new(url: Url) -> Self {
        WebSocketClientBuilder { url, token: None }
    }

    fn token<'a, S: Into<String>>(&'a mut self, token: S) -> &'a mut Self {
        self.token = Some(token.into());
        self
    }

    async fn build(&self) -> Result<WebSocketClient> {
        let mut url = self.url.clone();

        if let Some(token) = &self.token {
            url.query_pairs_mut().append_pair("i", token);
        }

        WebSocketClient::connect(url).await
    }
}

impl WebSocketClientBuilder {
    pub fn query<'a, S1, S2>(&'a mut self, key: S1, value: S2) -> &'a mut Self
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        self.url
            .query_pairs_mut()
            .append_pair(key.as_ref(), value.as_ref());
        self
    }
}
