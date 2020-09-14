use derive_more::{Display, Error, From};

/// Possible errors from HTTP client.
#[derive(Debug, Display, From, Error)]
pub enum Error {
    /// Errors from underlying [isahc](https://docs.rs/isahc) library.
    #[display(fmt = "network error: {}", _0)]
    Network(#[error(source)] isahc::Error),
    /// IO error.
    #[display(fmt = "IO error: {}", _0)]
    Io(#[error(source)] std::io::Error),
    /// JSON encode/decode error.
    #[display(fmt = "JSON error: {}", _0)]
    Json(#[error(source)] serde_json::Error),
}

/// Specialized Result type for operations on [`HttpClient`][`crate::HttpClient`].
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
