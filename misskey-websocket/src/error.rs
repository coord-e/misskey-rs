use std::convert::Infallible;
use std::sync::Arc;

use async_tungstenite::tungstenite;
use thiserror::Error;

/// Possible errors from WebSocket client.
#[derive(Debug, Error, Clone)]
pub enum Error {
    /// Errors from underlying [tungstenite](https://docs.rs/tungstenite) library.
    #[error("websocket error: {0}")]
    WebSocket(#[source] Arc<tungstenite::Error>),
    /// Received unexpected message from server.
    #[error("websocket unexpected message: {0}")]
    UnexpectedMessage(tungstenite::Message),
    /// Invalid URL.
    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),
    /// JSON encode/decode error.
    #[error("JSON error: {0}")]
    Json(#[source] Arc<serde_json::Error>),
}

impl From<Infallible> for Error {
    fn from(x: Infallible) -> Error {
        match x {}
    }
}

impl From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Error {
        Error::WebSocket(Arc::new(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(Arc::new(err))
    }
}

/// Specialized Result type for operations on [`WebSocketClient`][`crate::WebSocketClient`].
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Error>();
    }

    #[test]
    fn test_sync() {
        fn assert_send<T: Sync>() {}
        assert_send::<Error>();
    }
}
