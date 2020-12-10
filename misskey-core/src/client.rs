use crate::api::{Request, UploadFileRequest};
use crate::model::ApiResult;

use futures_core::future::BoxFuture;
use mime::Mime;

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

/// Abstraction over API clients that can upload files.
pub trait UploadFileClient: Client {
    /// Dispatches an API request with file.
    ///
    /// Takes the file to be attatched and [`UploadFileRequest`], then returns a future that waits for the [`Request::Response`].
    fn request_with_file<R, T>(
        &self,
        request: R,
        type_: Mime,
        file_name: String,
        content: T,
    ) -> BoxFuture<Result<ApiResult<R::Response>, Self::Error>>
    where
        R: UploadFileRequest,
        T: std::io::Read + Send + Sync + 'static;
}

impl<C: ?Sized> UploadFileClient for &C
where
    C: UploadFileClient,
{
    fn request_with_file<R, T>(
        &self,
        request: R,
        type_: Mime,
        file_name: String,
        content: T,
    ) -> BoxFuture<Result<ApiResult<R::Response>, Self::Error>>
    where
        R: UploadFileRequest,
        T: std::io::Read + Send + Sync + 'static,
    {
        C::request_with_file(self, request, type_, file_name, content)
    }
}

impl<C: ?Sized> UploadFileClient for &mut C
where
    C: UploadFileClient,
{
    fn request_with_file<R, T>(
        &self,
        request: R,
        type_: Mime,
        file_name: String,
        content: T,
    ) -> BoxFuture<Result<ApiResult<R::Response>, Self::Error>>
    where
        R: UploadFileRequest,
        T: std::io::Read + Send + Sync + 'static,
    {
        C::request_with_file(self, request, type_, file_name, content)
    }
}

impl<C: ?Sized> UploadFileClient for Box<C>
where
    C: UploadFileClient,
{
    fn request_with_file<R, T>(
        &self,
        request: R,
        type_: Mime,
        file_name: String,
        content: T,
    ) -> BoxFuture<Result<ApiResult<R::Response>, Self::Error>>
    where
        R: UploadFileRequest,
        T: std::io::Read + Send + Sync + 'static,
    {
        C::request_with_file(self, request, type_, file_name, content)
    }
}
