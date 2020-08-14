use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::sync::Arc;

use derivative::Derivative;
use derive_more::{Deref, Display, From};
use err_derive::Error;
use tokio_tungstenite::tungstenite;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error(display = "websocket error: {}", _0)]
    WebSocket(#[error(source, no_from)] ArcError<tungstenite::Error>),
    #[error(display = "websocket unexpected message: {}", _0)]
    UnexpectedMessage(tokio_tungstenite::tungstenite::Message),
    #[error(display = "JSON error: {}", _0)]
    Json(#[error(source, no_from)] ArcError<serde_json::Error>),
}

#[derive(Derivative, Display, From, Deref)]
#[derivative(Debug = "transparent", Clone(bound = ""))]
#[deref(forward)]
#[from(forward)]
pub struct ArcError<E>(pub Arc<E>);

impl<E: StdError> StdError for ArcError<E> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0.source()
    }
}

impl From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Error {
        Error::WebSocket(err.into())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err.into())
    }
}

pub type Result<T> = StdResult<T, Error>;
