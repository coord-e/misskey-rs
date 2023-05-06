use std::result::Result as StdResult;
use std::sync::Arc;
use std::time::Duration;

use crate::broker::{ReconnectCondition, ReconnectConfig};
use crate::client::WebSocketClient;
use crate::error::{Error, Result};

use async_tungstenite::tungstenite::http::{
    self,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use url::Url;

#[derive(Debug, Clone)]
struct WebSocketClientBuilderInner {
    url: Url,
    additional_headers: HeaderMap,
    reconnect: ReconnectConfig,
}

/// Builder for [`WebSocketClient`].
#[derive(Debug, Clone)]
pub struct WebSocketClientBuilder {
    inner: Result<WebSocketClientBuilderInner>,
}

trait ResultExt<T, E> {
    fn err_into<U>(self) -> StdResult<T, U>
    where
        E: Into<U>;
    fn and_then_mut<F>(&mut self, op: F)
    where
        F: FnOnce(&mut T) -> StdResult<(), E>;
}

impl<T, E> ResultExt<T, E> for StdResult<T, E> {
    fn err_into<U>(self) -> StdResult<T, U>
    where
        E: Into<U>,
    {
        self.map_err(Into::into)
    }

    fn and_then_mut<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T) -> StdResult<(), E>,
    {
        if let Ok(x) = self {
            if let Err(e) = f(x) {
                *self = Err(e);
            }
        }
    }
}

impl WebSocketClientBuilder {
    /// Creates a new builder instance with `url`.
    ///
    /// All configurations are set to default.
    pub fn new<T>(url: T) -> Self
    where
        T: TryInto<Url>,
        T::Error: Into<Error>,
    {
        let inner = url
            .try_into()
            .map_err(Into::into)
            .map(|url| WebSocketClientBuilderInner {
                url,
                additional_headers: HeaderMap::new(),
                reconnect: ReconnectConfig::default(),
            });

        WebSocketClientBuilder { inner }
    }

    /// Creates a new builder instance with the given host name `host`.
    ///
    /// This method configures the builder with a URL of the form `wss://{host}/streaming`.
    /// All other configurations are set to default.
    pub fn with_host<S>(host: S) -> Self
    where
        S: AsRef<str>,
    {
        let url = format!("wss://{}/streaming", host.as_ref());
        WebSocketClientBuilder::new(url.as_str())
    }

    /// Sets an additional header for the connection request.
    pub fn header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: TryInto<HeaderName>,
        V: TryInto<HeaderValue>,
        K::Error: Into<http::Error>,
        V::Error: Into<http::Error>,
    {
        self.inner.and_then_mut(|inner| {
            let result = key.try_into().err_into().and_then(|key| {
                let value = value.try_into().err_into()?;
                Ok((key, value))
            });
            match result {
                Ok((key, value)) => {
                    inner.additional_headers.insert(key, value);
                    Ok(())
                }
                Err(e) => Err(Error::WebSocket(Arc::new(e.into()))),
            }
        });
        self
    }

    /// Sets an API token.
    ///
    /// This method appends the given token as the `i` query parameter to the URL.
    pub fn token<S>(&mut self, token: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.query("i", token)
    }

    /// Specifies additional query parameters for the URL.
    pub fn query<S1, S2>(&mut self, key: S1, value: S2) -> &mut Self
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        self.inner.and_then_mut(|inner| {
            inner
                .url
                .query_pairs_mut()
                .append_pair(key.as_ref(), value.as_ref());
            Ok(())
        });
        self
    }

    /// Sets whether or not to enable automatic reconnection.
    ///
    /// Automatic reconnection is enabled by default (as per [`Default`][default] implementation for
    /// [`ReconnectConfig`]), and you can disable it with `.auto_reconnect(false)`.
    ///
    /// [default]: std::default::Default
    pub fn auto_reconnect(&mut self, enable: bool) -> &mut Self {
        if enable {
            self.reconnect_condition(ReconnectCondition::unexpected_reset())
        } else {
            self.reconnect_condition(ReconnectCondition::never())
        }
    }

    /// Sets an interval duration of automatic reconnection in seconds.
    pub fn reconnect_secs(&mut self, secs: u64) -> &mut Self {
        self.inner.and_then_mut(|inner| {
            inner.reconnect.interval = Duration::from_secs(secs);
            Ok(())
        });
        self
    }

    /// Sets an interval duration of automatic reconnection.
    pub fn reconnect_interval(&mut self, interval: Duration) -> &mut Self {
        self.inner.and_then_mut(|inner| {
            inner.reconnect.interval = interval;
            Ok(())
        });
        self
    }

    /// Specifies the condition for reconnecting.
    pub fn reconnect_condition(&mut self, condition: ReconnectCondition) -> &mut Self {
        self.inner.and_then_mut(|inner| {
            inner.reconnect.condition = condition;
            Ok(())
        });
        self
    }

    /// Specifies whether to re-send messages that may have failed to be sent when reconnecting.
    pub fn reconnect_retry_send(&mut self, enable: bool) -> &mut Self {
        self.inner.and_then_mut(|inner| {
            inner.reconnect.retry_send = enable;
            Ok(())
        });
        self
    }

    /// Finish this builder instance and connect to Misskey using this configuration.
    pub async fn connect(&self) -> Result<WebSocketClient> {
        let WebSocketClientBuilderInner {
            url,
            additional_headers,
            reconnect,
        } = match self.inner.clone() {
            Err(e) => return Err(e),
            Ok(inner) => inner,
        };

        WebSocketClient::connect_with_headers_and_config(url, additional_headers, reconnect).await
    }
}
