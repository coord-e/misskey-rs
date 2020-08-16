use err_derive::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "network error: {}", _0)]
    Network(#[error(source)] reqwest::Error),
    #[error(display = "JSON error: {}", _0)]
    Json(#[error(source)] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
