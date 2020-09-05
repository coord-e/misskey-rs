use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Request: Serialize {
    type Response: DeserializeOwned;
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

pub trait UploadFileRequest: Request {}

impl<R: ?Sized> UploadFileRequest for &'_ R where R: UploadFileRequest {}
impl<R: ?Sized> UploadFileRequest for &'_ mut R where R: UploadFileRequest {}
impl<R: ?Sized> UploadFileRequest for Box<R> where R: UploadFileRequest {}
