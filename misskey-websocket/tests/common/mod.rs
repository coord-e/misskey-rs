use std::env;

use misskey_api::api;
use misskey_api::model::{
    note::{Note, NoteId},
    user::User,
};
use misskey_core::model::ApiResult;
use misskey_core::{Client, Request};
use misskey_websocket::{WebSocketClient, WebSocketClientBuilder};
use url::Url;

// TODO: Share code with misskey-api

pub struct TestClient {
    pub admin: WebSocketClient,
    pub user: WebSocketClient,
}

impl TestClient {
    pub async fn new() -> Self {
        let url = env::var("TEST_WEBSOCKET_URL").unwrap();
        let url = Url::parse(&url).unwrap();
        let admin_token = env::var("TEST_ADMIN_TOKEN").unwrap();
        let user_token = env::var("TEST_USER_TOKEN").unwrap();

        let admin = WebSocketClientBuilder::new(url.clone())
            .token(admin_token)
            .connect()
            .await
            .unwrap();
        let user = WebSocketClientBuilder::new(url.clone())
            .token(user_token)
            .connect()
            .await
            .unwrap();

        TestClient { admin, user }
    }
}

#[async_trait::async_trait]
impl Client for TestClient {
    type Error = <WebSocketClient as Client>::Error;
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
    async fn me(&mut self) -> User;
    async fn create_note(
        &mut self,
        text: Option<&str>,
        renote_id: Option<NoteId>,
        reply_id: Option<NoteId>,
    ) -> Note;
}

#[async_trait::async_trait]
impl<T: Client + Send> ClientExt for T {
    async fn test<R: Request + Send>(&mut self, req: R) -> R::Response {
        self.request(req).await.unwrap().unwrap()
    }

    async fn me(&mut self) -> User {
        self.test(api::i::Request {}).await
    }

    async fn create_note(
        &mut self,
        text: Option<&str>,
        renote_id: Option<NoteId>,
        reply_id: Option<NoteId>,
    ) -> Note {
        self.test(api::notes::create::Request {
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
