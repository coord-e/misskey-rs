use std::io::Cursor;

use crate::model::drive::DriveFile;

use mime::Mime;
use misskey_core::{UploadFileClient, UploadFileRequest};
use misskey_http::HttpClient;
use misskey_test::env;

pub struct TestClient {
    pub admin: HttpClient,
    pub user: HttpClient,
}

impl TestClient {
    pub fn new() -> Self {
        misskey_test::init_logger();

        TestClient {
            admin: HttpClient::with_token(env::api_url(), env::admin_token()).unwrap(),
            user: HttpClient::with_token(env::api_url(), env::user_token()).unwrap(),
        }
    }
}

impl std::ops::Deref for TestClient {
    type Target = HttpClient;
    fn deref(&self) -> &HttpClient {
        &self.user
    }
}

#[async_trait::async_trait]
pub trait HttpClientExt {
    async fn test_with_file<R, B>(
        &self,
        req: R,
        mime: Mime,
        file_name: &str,
        content: B,
    ) -> R::Response
    where
        R: UploadFileRequest + Send,
        B: AsRef<[u8]> + Send + Sync;
    async fn create_text_file(&self, file_name: &str, content: &str) -> DriveFile;
}

#[async_trait::async_trait]
impl HttpClientExt for HttpClient {
    async fn test_with_file<R, B>(
        &self,
        req: R,
        mime: Mime,
        file_name: &str,
        content: B,
    ) -> R::Response
    where
        R: UploadFileRequest + Send,
        B: AsRef<[u8]> + Send + Sync,
    {
        self.request_with_file(
            req,
            mime,
            file_name.to_string(),
            Cursor::new(content.as_ref().to_vec()),
        )
        .await
        .unwrap()
        .unwrap()
    }

    async fn create_text_file(&self, file_name: &str, content: &str) -> DriveFile {
        self.test_with_file(
            crate::endpoint::drive::files::create::Request {
                folder_id: None,
                name: Some(file_name.to_string()),
                #[cfg(feature = "12-102-0")]
                comment: None,
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
