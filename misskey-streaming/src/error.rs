use err_derive::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "websocket error: {}", _0)]
    WebSocket(#[error(source)] tokio_tungstenite::tungstenite::Error),
    #[error(display = "websocket unexpected message: {}", _0)]
    UnexpectedMessage(tokio_tungstenite::tungstenite::Message),
    #[error(display = "JSON error: {}", _0)]
    Json(#[error(source)] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
