use crate::model::{
    note::{Note, NoteId},
    user::User,
};

use misskey_core::model::ApiResult;
use misskey_core::{ApiRequest, Client};
use misskey_http::HttpClient;
use url::Url;
use uuid::Uuid;

pub struct TestClient {
    pub admin: HttpClient,
    pub user: HttpClient,
}

impl TestClient {
    pub fn new() -> Self {
        let url = std::env::var("TEST_API_URL").unwrap();
        let url = Url::parse(&url).unwrap();
        let admin_token = std::env::var("TEST_ADMIN_TOKEN").unwrap();
        let user_token = std::env::var("TEST_USER_TOKEN").unwrap();

        TestClient {
            admin: HttpClient::new(url.clone(), Some(admin_token)),
            user: HttpClient::new(url.clone(), Some(user_token)),
        }
    }
}

#[async_trait::async_trait]
impl Client for TestClient {
    type Error = <HttpClient as Client>::Error;
    async fn request<R: ApiRequest + Send>(
        &mut self,
        request: R,
    ) -> Result<ApiResult<R::Response>, Self::Error> {
        self.user.request(request).await
    }
}

#[async_trait::async_trait]
pub trait ClientExt {
    async fn test<R: ApiRequest + Send>(&mut self, req: R) -> R::Response;
    async fn me(&mut self) -> User;
    async fn create_test_account(&mut self) -> User;
    async fn create_note(
        &mut self,
        text: Option<&'static str>,
        renote_id: Option<NoteId>,
        reply_id: Option<NoteId>,
    ) -> Note;
}

#[async_trait::async_trait]
impl<T: Client + Send> ClientExt for T {
    async fn test<R: ApiRequest + Send>(&mut self, req: R) -> R::Response {
        self.request(req).await.unwrap().unwrap()
    }

    async fn me(&mut self) -> User {
        self.test(crate::api::i::Request {}).await
    }

    async fn create_test_account(&mut self) -> User {
        let uuid = Uuid::new_v4().to_simple().to_string();
        self.test(crate::api::admin::accounts::create::Request {
            username: uuid[..20].to_owned(),
            password: "test".to_string(),
        })
        .await
        .user
    }

    async fn create_note(
        &mut self,
        text: Option<&'static str>,
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
}
