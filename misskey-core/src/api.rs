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

/// [`Request`] that can be paginated via `since_id` and `until_id`.
pub trait PaginationRequest: Request {
    /// The paginated item type.
    type Item;

    /// Sets the `since_id` field of the request.
    fn set_since(&mut self, item: &Self::Item);
    /// Sets the `until_id` field of the request.
    fn set_until(&mut self, item: &Self::Item);
}

impl<R: ?Sized> PaginationRequest for &'_ mut R
where
    R: PaginationRequest,
{
    type Item = R::Item;

    fn set_since(&mut self, item: &Self::Item) {
        R::set_since(self, item)
    }
    fn set_until(&mut self, item: &Self::Item) {
        R::set_until(self, item)
    }
}

impl<R: ?Sized> PaginationRequest for Box<R>
where
    R: PaginationRequest,
{
    type Item = R::Item;

    fn set_since(&mut self, item: &Self::Item) {
        R::set_since(self, item)
    }
    fn set_until(&mut self, item: &Self::Item) {
        R::set_until(self, item)
    }
}

/// [`Request`] that can be paginated via `offset`.
pub trait OffsetPaginationRequest: Request {
    /// The paginated item type.
    type Item;

    /// Sets the `offset` field of the request.
    fn set_offset(&mut self, offset: u64);
}

impl<R: ?Sized> OffsetPaginationRequest for &'_ mut R
where
    R: OffsetPaginationRequest,
{
    type Item = R::Item;

    fn set_offset(&mut self, offset: u64) {
        R::set_offset(self, offset)
    }
}

impl<R: ?Sized> OffsetPaginationRequest for Box<R>
where
    R: OffsetPaginationRequest,
{
    type Item = R::Item;

    fn set_offset(&mut self, offset: u64) {
        R::set_offset(self, offset)
    }
}

/// [`Request`] that requires a file to upload.
pub trait UploadFileRequest: Request {}

impl<R: ?Sized> UploadFileRequest for &'_ R where R: UploadFileRequest {}
impl<R: ?Sized> UploadFileRequest for &'_ mut R where R: UploadFileRequest {}
impl<R: ?Sized> UploadFileRequest for Box<R> where R: UploadFileRequest {}
