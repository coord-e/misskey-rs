use crate::api::Request;
use crate::model::ApiResult;

use futures_core::future::BoxFuture;

pub trait Client {
    type Error: std::error::Error;

    fn request<'a, R>(
        &'a mut self,
        request: R,
    ) -> BoxFuture<'a, Result<ApiResult<R::Response>, Self::Error>>
    where
        R: Request + 'a;
}
