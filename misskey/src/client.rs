use crate::api::{ApiRequest, ApiResult};

use url::Url;

#[async_trait::async_trait]
pub trait ClientBuilder {
    type Client: Client;

    fn new(url: Url) -> Self;
    fn token<'a, S: Into<String>>(&'a mut self, token: S) -> &'a mut Self;
    async fn build(&self) -> Result<Self::Client, <Self::Client as Client>::Error>;
}

#[async_trait::async_trait]
pub trait Client {
    type Error: std::error::Error;

    async fn request<R: ApiRequest + Send>(
        &mut self,
        request: R,
    ) -> Result<ApiResult<R::Response>, Self::Error>;
}
