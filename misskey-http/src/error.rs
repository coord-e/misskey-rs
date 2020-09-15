use thiserror::Error;

/// Possible errors from HTTP client.
#[derive(Debug, Error)]
pub enum Error {
    /// Errors from underlying [isahc](https://docs.rs/isahc) library.
    #[error("network error: {0}")]
    Network(#[from] isahc::Error),
    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// JSON encode/decode error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
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
