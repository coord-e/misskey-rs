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
    fn request<R: Request>(
        &self,
        request: R,
    ) -> BoxFuture<Result<ApiResult<R::Response>, Self::Error>>;
}

impl<C: Client + ?Sized> Client for &C {
    type Error = C::Error;

    fn request<R: Request>(
        &self,
        request: R,
    ) -> BoxFuture<Result<ApiResult<R::Response>, Self::Error>> {
        C::request(self, request)
    }
}

impl<C: Client + ?Sized> Client for &mut C {
    type Error = C::Error;

    fn request<R: Request>(
        &self,
        request: R,
    ) -> BoxFuture<Result<ApiResult<R::Response>, Self::Error>> {
        C::request(self, request)
    }
}

impl<C: Client + ?Sized> Client for Box<C> {
    type Error = C::Error;

    fn request<R: Request>(
        &self,
        request: R,
    ) -> BoxFuture<Result<ApiResult<R::Response>, Self::Error>> {
        C::request(self, request)
    }
}
