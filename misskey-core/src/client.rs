use crate::model::ApiResult;
use crate::request::Request;

#[async_trait::async_trait]
pub trait Client {
    type Error: std::error::Error;

    async fn request<R: Request + Send>(
        &mut self,
        request: R,
    ) -> Result<ApiResult<R::Response>, Self::Error>;
}
