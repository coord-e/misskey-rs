use crate::model::{
    emoji::EmojiId,
    note::{Note, NoteId},
    user::User,
};

use misskey_core::{Client, Request};
use misskey_http::HttpClient;
use url::Url;
use uuid::Uuid;

mod env;
pub mod http;
pub mod websocket;

pub use http::{HttpClientExt, TestClient};

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
        self.test(crate::endpoint::i::Request {}).await
    }

    async fn create_user(&mut self) -> (User, HttpClient) {
        let uuid = Uuid::new_v4().to_simple().to_string();
        let res = self
            .test(crate::endpoint::admin::accounts::create::Request {
                username: uuid[..20].to_owned(),
                password: "test".to_string(),
            })
            .await;

        (
            res.user,
            HttpClient::new(env::TEST_API_URL.clone(), Some(res.token)),
        )
    }

    async fn create_note(
        &mut self,
        text: Option<&str>,
        renote_id: Option<NoteId>,
        reply_id: Option<NoteId>,
    ) -> Note {
        self.test(crate::endpoint::notes::create::Request {
            visibility: None,
            visible_user_ids: None,
            text: text.map(|x| x.to_string()),
            cw: None,
            via_mobile: None,
            local_only: None,
            no_extract_mentions: None,
            no_extract_hashtags: None,
            no_extract_emojis: None,
            file_ids: None,
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
            env::TEST_API_URL.join(&path).unwrap()
        }
    }

    #[cfg(feature = "12-9-0")]
    async fn add_emoji_from_url(&mut self, url: Url) -> EmojiId {
        let file = self
            .test(crate::endpoint::drive::files::upload_from_url::Request {
                url,
                folder_id: None,
                is_sensitive: None,
                force: None,
            })
            .await;

        self.test(crate::endpoint::admin::emoji::add::Request { file_id: file.id })
            .await
            .id
    }

    #[cfg(not(feature = "12-9-0"))]
    async fn add_emoji_from_url(&mut self, url: Url) -> EmojiId {
        let uuid = Uuid::new_v4().to_simple().to_string();
        self.test(crate::endpoint::admin::emoji::add::Request {
            name: uuid,
            url,
            category: None,
            aliases: None,
        })
        .await
        .id
    }
}
