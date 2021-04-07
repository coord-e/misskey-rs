use std::fmt::{self, Debug, Display};

use misskey_core::model::ApiError;

/// Possible errors from the high-level API.
pub enum Error<E> {
    /// Errors from underlying client, namely `E`.
    Client(E),
    /// Errors from Misskey API.
    Api(ApiError),
    /// IO Errors from some high-level API.
    Io(std::io::Error),
}

impl<E: std::error::Error> std::error::Error for Error<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Client(err) => err.source(),
            Error::Api(err) => Some(err),
            Error::Io(err) => err.source(),
        }
    }
}

impl<E: std::error::Error> std::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Client(err) => Display::fmt(err, f),
            Error::Api(_) => write!(f, "Misskey API returned an error"),
            Error::Io(err) => Display::fmt(err, f),
        }
    }
}

impl<E: std::error::Error> Debug for Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Client(err) => f.debug_tuple("Client").field(&err).finish(),
            Error::Api(err) => f.debug_tuple("Api").field(&err).finish(),
            Error::Io(err) => f.debug_tuple("Io").field(&err).finish(),
        }
    }
}

impl<E> From<ApiError> for Error<E> {
    fn from(err: ApiError) -> Self {
        Error::Api(err)
    }
}

impl<E> From<std::io::Error> for Error<E> {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

#[cfg(feature = "http-client")]
impl From<misskey_http::Error> for Error<misskey_http::Error> {
    fn from(err: misskey_http::Error) -> Self {
        Error::Client(err)
    }
}

#[cfg(feature = "websocket-client")]
impl From<misskey_websocket::Error> for Error<misskey_websocket::Error> {
    fn from(err: misskey_websocket::Error) -> Self {
        Error::Client(err)
    }
}
