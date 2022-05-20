use std::convert::TryInto;
use std::result::Result as StdResult;

use crate::client::HttpClient;
use crate::error::{Error, Result};

use isahc::http::{
    self,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use url::Url;

#[derive(Debug)]
struct HttpClientBuilderInner {
    url: Url,
    token: Option<String>,
    additional_headers: HeaderMap,
}

/// Builder for [`HttpClient`].
#[derive(Debug)]
pub struct HttpClientBuilder {
    inner: Result<HttpClientBuilderInner>,
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

impl HttpClientBuilder {
    /// Creates a new builder instance with `url`.
    ///
    /// All configurations are set to default.
    pub fn new<T>(url: T) -> Self
    where
        T: TryInto<Url>,
        T::Error: Into<Error>,
    {
        let inner = url.try_into().err_into().map(|url| HttpClientBuilderInner {
            url,
            token: None,
            additional_headers: HeaderMap::new(),
        });
        HttpClientBuilder { inner }
    }

    /// Creates a new builder instance with the given host name `host`.
    ///
    /// This method configures the builder with a URL of the form `https://{host}/api/`.
    /// All other configurations are set to default.
    pub fn with_host<S>(host: S) -> Self
    where
        S: AsRef<str>,
    {
        let url = format!("https://{}/api/", host.as_ref());
        HttpClientBuilder::new(url.as_str())
    }

    /// Sets an additional header for all requests.
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
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
                Err(e) => Err(Error::Network(e.into())),
            }
        });
        self
    }

    /// Sets an API token.
    pub fn token<S>(mut self, token: S) -> Self
    where
        S: Into<String>,
    {
        self.inner.and_then_mut(|inner| {
            inner.token = Some(token.into());
            Ok(())
        });
        self
    }

    /// Finish this builder instance and build [`HttpClient`].
    pub fn build(self) -> Result<HttpClient> {
        self.inner.and_then(|inner| {
            Ok(HttpClient {
                url: inner.url,
                token: inner.token,
                client: isahc::HttpClientBuilder::new()
                    .default_headers(&inner.additional_headers)
                    .build()?,
            })
        })
    }
}
