use derive_more::{Display, Error, From};

#[derive(Debug, Display, From, Error)]
pub enum Error {
    #[display(fmt = "network error: {}", _0)]
    Network(#[error(source)] reqwest::Error),
    #[display(fmt = "JSON error: {}", _0)]
    Json(#[error(source)] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
