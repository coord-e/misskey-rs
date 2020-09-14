use crate::api::Request;
use crate::model::ApiResult;

use futures_core::future::BoxFuture;

/// Abstraction over API clients.
pub trait Client {
    /// The error type produced by the client when an error occurs.
    type Error: std::error::Error;

    /// Dispatch an API request.
    ///
    /// Takes [`Request`] and returns a future that waits for the [`Response`][`Request::Response`].
    fn request<'a, R>(
        &'a self,
        request: R,
    ) -> BoxFuture<'a, Result<ApiResult<R::Response>, Self::Error>>
    where
        R: Request + 'a;
}
