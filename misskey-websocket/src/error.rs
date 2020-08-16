use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::sync::Arc;

use derivative::Derivative;
use derive_more::{Deref, Display, Error, From};
use tokio_tungstenite::tungstenite;

#[derive(Debug, Display, Error, Clone)]
pub enum Error {
    #[display(fmt = "websocket error: {}", _0)]
    WebSocket(#[error(source)] ArcError<tungstenite::Error>),
    #[display(fmt = "websocket unexpected message: {}", _0)]
    UnexpectedMessage(#[error(not(source))] tokio_tungstenite::tungstenite::Message),
    #[display(fmt = "JSON error: {}", _0)]
    Json(#[error(source)] ArcError<serde_json::Error>),
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
