use crate::model::drive::DriveFile;
use crate::test::env;

use futures::future::BoxFuture;
use mime::Mime;
use misskey_core::model::ApiResult;
use misskey_core::{Client, Request, UploadFileRequest};
use misskey_http::HttpClient;
use uuid::Uuid;

pub struct TestClient {
    pub admin: HttpClient,
    pub user: HttpClient,
}

impl TestClient {
    pub fn new() -> Self {
        env::init_logger();

        TestClient {
            admin: HttpClient::new(
                env::TEST_API_URL.clone(),
                Some(env::TEST_ADMIN_TOKEN.clone()),
            )
            .unwrap(),
            user: HttpClient::new(
                env::TEST_API_URL.clone(),
                Some(env::TEST_USER_TOKEN.clone()),
            )
            .unwrap(),
        }
    }
}

impl Client for TestClient {
    type Error = <HttpClient as Client>::Error;
    fn request<'a, R>(
        &'a mut self,
        request: R,
    ) -> BoxFuture<'a, Result<ApiResult<R::Response>, Self::Error>>
    where
        R: Request + 'a,
    {
        self.user.request(request)
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
        R: UploadFileRequest + Send,
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
        R: UploadFileRequest + Send,
        B: AsRef<[u8]> + Send + Sync,
    {
        let tmp_name = Uuid::new_v4().to_simple().to_string();
        let path = std::env::temp_dir().join(tmp_name);
        {
            use tokio::{fs::File, io::AsyncWriteExt};
            let mut file = File::create(&path).await.unwrap();
            file.write_all(content.as_ref()).await.unwrap();
            file.sync_all().await.unwrap();
        }

        self.request_with_file(req, mime, file_name.to_string(), &path)
            .await
            .unwrap()
            .unwrap()
    }

    async fn create_text_file(&mut self, file_name: &str, content: &str) -> DriveFile {
        self.test_with_file(
            crate::endpoint::drive::files::create::Request {
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
        R: UploadFileRequest + Send,
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
