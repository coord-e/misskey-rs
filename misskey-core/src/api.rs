use serde::de::DeserializeOwned;
use serde::Serialize;

/// API request.
///
/// Request type is [`Serialize`] with associated response type [`Response`][`Request::Response`] and endpoint name [`ENDPOINT`][`Request::ENDPOINT`].
pub trait Request: Serialize {
    /// Response type of this request.
    type Response: DeserializeOwned;
    /// The name of the corresponding endpoint.
    const ENDPOINT: &'static str;
}

impl<R: ?Sized> Request for &'_ R
where
    R: Request,
{
    type Response = R::Response;
    const ENDPOINT: &'static str = R::ENDPOINT;
}

impl<R: ?Sized> Request for &'_ mut R
where
    R: Request,
{
    type Response = R::Response;
    const ENDPOINT: &'static str = R::ENDPOINT;
}

impl<R: ?Sized> Request for Box<R>
where
    R: Request,
{
    type Response = R::Response;
    const ENDPOINT: &'static str = R::ENDPOINT;
}

/// [`Request`] that requires a file to upload.
pub trait UploadFileRequest: Request {}

impl<R: ?Sized> UploadFileRequest for &'_ R where R: UploadFileRequest {}
impl<R: ?Sized> UploadFileRequest for &'_ mut R where R: UploadFileRequest {}
impl<R: ?Sized> UploadFileRequest for Box<R> where R: UploadFileRequest {}
