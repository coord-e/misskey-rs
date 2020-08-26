use crate::model::drive::DriveFile;
use crate::test::env;

use mime::Mime;
use misskey_core::model::ApiResult;
use misskey_core::{Client, Request, RequestWithFile};
use misskey_http::HttpClient;

pub struct TestClient {
    pub admin: HttpClient,
    pub user: HttpClient,
}

impl TestClient {
    pub fn new() -> Self {
        TestClient {
            admin: HttpClient::new(
                env::TEST_API_URL.clone(),
                Some(env::TEST_ADMIN_TOKEN.clone()),
            ),
            user: HttpClient::new(
                env::TEST_API_URL.clone(),
                Some(env::TEST_USER_TOKEN.clone()),
            ),
        }
    }
}

#[async_trait::async_trait]
impl Client for TestClient {
    type Error = <HttpClient as Client>::Error;
    async fn request<R: Request + Send>(
        &mut self,
        request: R,
    ) -> Result<ApiResult<R::Response>, Self::Error> {
        self.user.request(request).await
    }
}

#[async_trait::async_trait]
pub trait HttpClientExt {
    async fn test_with_file<R, B>(
        &mut self,
        req: R,
        mime: Mime,
        file_name: &str,
        content: B,
    ) -> R::Response
    where
        R: RequestWithFile + Send,
        B: AsRef<[u8]> + Send + Sync;
    async fn create_text_file(&mut self, file_name: &str, content: &str) -> DriveFile;
}

#[async_trait::async_trait]
impl HttpClientExt for HttpClient {
    async fn test_with_file<R, B>(
        &mut self,
        req: R,
        mime: Mime,
        file_name: &str,
        content: B,
    ) -> R::Response
    where
        R: RequestWithFile + Send,
        B: AsRef<[u8]> + Send + Sync,
    {
        self.request_with_file(req, mime, file_name.to_string(), content.as_ref().to_vec())
            .await
            .unwrap()
            .unwrap()
    }

    async fn create_text_file(&mut self, file_name: &str, content: &str) -> DriveFile {
        self.test_with_file(
            crate::api::drive::files::create::Request {
                folder_id: None,
                name: Some(file_name.to_string()),
                is_sensitive: None,
                force: Some(true),
            },
            mime::TEXT_PLAIN,
            file_name,
            content,
        )
        .await
    }
}

#[async_trait::async_trait]
impl HttpClientExt for TestClient {
    async fn test_with_file<R, B>(
        &mut self,
        req: R,
        mime: Mime,
        file_name: &str,
        content: B,
    ) -> R::Response
    where
        R: RequestWithFile + Send,
        B: AsRef<[u8]> + Send + Sync,
    {
        self.user
            .test_with_file(req, mime, file_name, content)
            .await
    }

    async fn create_text_file(&mut self, file_name: &str, content: &str) -> DriveFile {
        self.user.create_text_file(file_name, content).await
    }
}
