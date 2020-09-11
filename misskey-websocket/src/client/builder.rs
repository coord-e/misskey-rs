use std::time::Duration;

use crate::broker::{ReconnectCondition, ReconnectConfig};
use crate::client::WebSocketClient;
use crate::error::Result;

use url::Url;

#[derive(Debug, Clone)]
pub struct WebSocketClientBuilder {
    url: Url,
    token: Option<String>,
    reconnect: Option<ReconnectConfig>,
}

impl WebSocketClientBuilder {
    pub fn query<S1, S2>(&mut self, key: S1, value: S2) -> &mut Self
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        self.url
            .query_pairs_mut()
            .append_pair(key.as_ref(), value.as_ref());
        self
    }

    pub fn new(url: Url) -> Self {
        WebSocketClientBuilder {
            url,
            token: None,
            reconnect: None,
        }
    }

    pub fn token<S: Into<String>>(&mut self, token: S) -> &mut Self {
        self.token = Some(token.into());
        self
    }

    pub fn auto_reconnect(&mut self) -> &mut Self {
        self.reconnect = Some(ReconnectConfig::default());
        self
    }

    pub fn reconnect_secs(&mut self, secs: u64) -> &mut Self {
        self.reconnect
            .get_or_insert_with(ReconnectConfig::default)
            .interval = Duration::from_secs(secs);
        self
    }

    pub fn reconnect_interval(&mut self, interval: Duration) -> &mut Self {
        self.reconnect
            .get_or_insert_with(ReconnectConfig::default)
            .interval = interval;
        self
    }

    pub fn reconnect_condition(&mut self, condition: ReconnectCondition) -> &mut Self {
        self.reconnect
            .get_or_insert_with(ReconnectConfig::default)
            .condition = condition;
        self
    }

    pub fn reconnect_retry_send(&mut self, enable: bool) -> &mut Self {
        self.reconnect
            .get_or_insert_with(ReconnectConfig::default)
            .retry_send = enable;
        self
    }

    pub async fn connect(&self) -> Result<WebSocketClient> {
        let mut url = self.url.clone();

        if let Some(token) = &self.token {
            url.query_pairs_mut().append_pair("i", token);
        }

        WebSocketClient::connect(url, self.reconnect.clone()).await
    }
}
