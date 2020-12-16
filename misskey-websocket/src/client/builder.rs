use std::time::Duration;

use crate::broker::{ReconnectCondition, ReconnectConfig};
use crate::client::WebSocketClient;
use crate::error::Result;

use url::Url;

/// Builder for [`WebSocketClient`].
#[derive(Debug, Clone)]
pub struct WebSocketClientBuilder {
    url: Url,
    token: Option<String>,
    reconnect: Option<ReconnectConfig>,
}

impl WebSocketClientBuilder {
    /// Specifies additional query parameters for the URL.
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

    /// Creates a new builder instance with `url`.
    /// All configurations are set to default.
    ///
    /// This function is identical to [`WebSocketClient::builder`].
    pub fn new(url: Url) -> Self {
        WebSocketClientBuilder {
            url,
            token: None,
            reconnect: None,
        }
    }

    /// Sets an API token.
    pub fn token<S: Into<String>>(&mut self, token: S) -> &mut Self {
        self.token = Some(token.into());
        self
    }

    /// Enables automatic reconnection.
    pub fn auto_reconnect(&mut self) -> &mut Self {
        self.reconnect = Some(ReconnectConfig::default());
        self
    }

    /// Sets an interval duration of automatic reconnection in seconds.
    pub fn reconnect_secs(&mut self, secs: u64) -> &mut Self {
        self.reconnect
            .get_or_insert_with(ReconnectConfig::default)
            .interval = Duration::from_secs(secs);
        self
    }

    /// Sets an interval duration of automatic reconnection.
    pub fn reconnect_interval(&mut self, interval: Duration) -> &mut Self {
        self.reconnect
            .get_or_insert_with(ReconnectConfig::default)
            .interval = interval;
        self
    }

    /// Specifies the condition for reconnecting.
    pub fn reconnect_condition(&mut self, condition: ReconnectCondition) -> &mut Self {
        self.reconnect
            .get_or_insert_with(ReconnectConfig::default)
            .condition = condition;
        self
    }

    /// Specifies whether to re-send messages that may have failed to be sent when reconnecting.
    pub fn reconnect_retry_send(&mut self, enable: bool) -> &mut Self {
        self.reconnect
            .get_or_insert_with(ReconnectConfig::default)
            .retry_send = enable;
        self
    }

    /// Finish this builder instance and connect to Misskey using this configuration.
    pub async fn connect(&self) -> Result<WebSocketClient> {
        let mut url = self.url.clone();

        if let Some(token) = &self.token {
            url.query_pairs_mut().append_pair("i", token);
        }

        if let Some(config) = &self.reconnect {
            WebSocketClient::connect_with_config(url, config.clone()).await
        } else {
            WebSocketClient::connect(url).await
        }
    }
}
