use crate::api::{ApiRequest, ApiResult};

#[async_trait::async_trait]
pub trait Client {
    type Error: std::error::Error;

    async fn request<R: ApiRequest + Send>(
        &mut self,
        request: R,
    ) -> Result<ApiResult<R::Response>, Self::Error>;
}
