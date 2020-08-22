use crate::model::{
    drive::DriveFile,
    emoji::EmojiId,
    note::{Note, NoteId},
    user::User,
};

use mime::Mime;
use misskey_core::model::ApiResult;
use misskey_core::{Client, Request, RequestWithFile};
use misskey_http::HttpClient;
use url::Url;
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref TEST_API_URL: Url = {
        let url = std::env::var("TEST_API_URL").unwrap();
        Url::parse(&url).unwrap()
    };
}

pub struct TestClient {
    pub admin: HttpClient,
    pub user: HttpClient,
}

impl TestClient {
    pub fn new() -> Self {
        let admin_token = std::env::var("TEST_ADMIN_TOKEN").unwrap();
        let user_token = std::env::var("TEST_USER_TOKEN").unwrap();

        TestClient {
            admin: HttpClient::new(TEST_API_URL.clone(), Some(admin_token)),
            user: HttpClient::new(TEST_API_URL.clone(), Some(user_token)),
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
pub trait ClientExt {
    async fn test<R: Request + Send>(&mut self, req: R) -> R::Response;
    async fn create_user(&mut self) -> (User, HttpClient);
    async fn me(&mut self) -> User;
    async fn create_note(
        &mut self,
        text: Option<&str>,
        renote_id: Option<NoteId>,
        reply_id: Option<NoteId>,
    ) -> Note;
    async fn avatar_url(&mut self) -> Url;
    async fn add_emoji_from_url(&mut self, url: Url) -> EmojiId;
}

#[async_trait::async_trait]
impl<T: Client + Send> ClientExt for T {
    async fn test<R: Request + Send>(&mut self, req: R) -> R::Response {
        self.request(req).await.unwrap().unwrap()
    }

    async fn me(&mut self) -> User {
        self.test(crate::api::i::Request {}).await
    }

    async fn create_user(&mut self) -> (User, HttpClient) {
        let uuid = Uuid::new_v4().to_simple().to_string();
        let res = self
            .test(crate::api::admin::accounts::create::Request {
                username: uuid[..20].to_owned(),
                password: "test".to_string(),
            })
            .await;

        (
            res.user,
            HttpClient::new(TEST_API_URL.clone(), Some(res.token)),
        )
    }

    async fn create_note(
        &mut self,
        text: Option<&str>,
        renote_id: Option<NoteId>,
        reply_id: Option<NoteId>,
    ) -> Note {
        self.test(crate::api::notes::create::Request {
            visibility: None,
            visible_user_ids: Vec::new(),
            text: text.map(|x| x.to_string()),
            cw: None,
            via_mobile: false,
            local_only: false,
            no_extract_mentions: false,
            no_extract_hashtags: false,
            no_extract_emojis: false,
            file_ids: Vec::new(),
            reply_id,
            renote_id,
            poll: None,
        })
        .await
        .created_note
    }

    async fn avatar_url(&mut self) -> Url {
        let me = self.me().await;
        if let Some(url) = me.avatar_url {
            url
        } else {
            let path = format!("/avatar/{}", me.id);
            TEST_API_URL.join(&path).unwrap()
        }
    }

    #[cfg(feature = "12-9-0")]
    async fn add_emoji_from_url(&mut self, url: Url) -> EmojiId {
        let file = self
            .test(crate::api::drive::files::upload_from_url::Request {
                url,
                folder_id: None,
                is_sensitive: None,
                force: None,
            })
            .await;

        self.test(crate::api::admin::emoji::add::Request { file_id: file.id })
            .await
            .id
    }

    #[cfg(not(feature = "12-9-0"))]
    async fn add_emoji_from_url(&mut self, url: Url) -> EmojiId {
        let uuid = Uuid::new_v4().to_simple().to_string();
        self.test(crate::api::admin::emoji::add::Request {
            name: uuid,
            url,
            category: None,
            aliases: None,
        })
        .await
        .id
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
