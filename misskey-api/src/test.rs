use crate::model::{
    note::{Note, NoteId},
    user::User,
};

use misskey_core::model::ApiResult;
use misskey_core::{ApiRequest, Client};
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
    async fn create_user(&mut self) -> (User, HttpClient);
    async fn me(&mut self) -> User;
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
