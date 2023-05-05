use crate::model::{drive::DriveFile, emoji::Emoji, id::Id, note::Note, user::User};

use misskey_core::{Client, Request};
use misskey_http::HttpClient;
use misskey_test::env;
use misskey_websocket::WebSocketClient;
use ulid_crate::Ulid;
use url::Url;

pub mod http;
pub mod websocket;

pub use http::{HttpClientExt, TestClient};

#[async_trait::async_trait]
pub trait ClientExt {
    async fn test<R: Request + Send>(&self, req: R) -> R::Response;
    async fn create_user(&self) -> (User, HttpClient);
    async fn create_streaming_user(&self) -> (User, WebSocketClient);
    async fn create_http_and_ws_client(&self) -> (User, HttpClient, WebSocketClient);
    async fn me(&self) -> User;
    async fn create_note(
        &self,
        text: Option<&str>,
        renote_id: Option<Id<Note>>,
        reply_id: Option<Id<Note>>,
    ) -> Note;
    // `drive/files/upload-from-url` does not return `DriveFile` since 12.48.0
    // so we need this
    async fn upload_from_url(&self, url: Url) -> DriveFile;
    async fn avatar_url(&self) -> Url;
    async fn add_emoji_from_url(&self, url: Url) -> Id<Emoji>;
}

#[async_trait::async_trait]
impl<T: Client + Send + Sync> ClientExt for T {
    async fn test<R: Request + Send>(&self, req: R) -> R::Response {
        self.request(req).await.unwrap().unwrap()
    }

    async fn me(&self) -> User {
        self.test(crate::endpoint::i::Request {}).await
    }

    async fn create_user(&self) -> (User, HttpClient) {
        let ulid = Ulid::new().to_string();
        let res = self
            .test(crate::endpoint::admin::accounts::create::Request {
                username: ulid[..20].to_owned(),
                password: "test".to_string(),
            })
            .await;

        (
            res.user,
            HttpClient::with_token(env::api_url(), res.token).unwrap(),
        )
    }

    async fn create_streaming_user(&self) -> (User, WebSocketClient) {
        let ulid = Ulid::new().to_string();
        let res = self
            .test(crate::endpoint::admin::accounts::create::Request {
                username: ulid[..20].to_owned(),
                password: "test".to_string(),
            })
            .await;

        (
            res.user,
            WebSocketClient::builder(env::websocket_url())
                .token(res.token)
                .connect()
                .await
                .unwrap(),
        )
    }

    async fn create_http_and_ws_client(&self) -> (User, HttpClient, WebSocketClient) {
        let ulid = Ulid::new().to_string();
        let res = self
            .test(crate::endpoint::admin::accounts::create::Request {
                username: ulid[..20].to_owned(),
                password: "test".to_string(),
            })
            .await;

        (
            res.user,
            HttpClient::with_token(env::api_url(), res.token.clone()).unwrap(),
            WebSocketClient::builder(env::websocket_url())
                .token(res.token)
                .connect()
                .await
                .unwrap(),
        )
    }

    async fn create_note(
        &self,
        text: Option<&str>,
        renote_id: Option<Id<Note>>,
        reply_id: Option<Id<Note>>,
    ) -> Note {
        let mut request = crate::endpoint::notes::create::Request::builder().build();
        request.text = text.map(Into::into);
        request.renote_id = renote_id;
        request.reply_id = reply_id;

        self.test(request).await.created_note
    }

    async fn avatar_url(&self) -> Url {
        let me = self.me().await;
        if let Some(url) = me.avatar_url {
            url
        } else {
            let path = format!("/avatar/{}", me.id);
            env::api_url().join(&path).unwrap()
        }
    }

    // TODO: better impl
    async fn upload_from_url(&self, url: Url) -> DriveFile {
        let random = ulid_crate::Ulid::new().to_string();
        let folder = self
            .test(crate::endpoint::drive::folders::create::Request {
                name: Some(random),
                parent_id: None,
            })
            .await;

        self.test(crate::endpoint::drive::files::upload_from_url::Request {
            #[cfg(feature = "12-48-0")]
            comment: None,
            #[cfg(feature = "12-48-0")]
            marker: None,
            url,
            folder_id: Some(folder.id),
            is_sensitive: None,
            force: Some(true),
        })
        .await;

        loop {
            let files = self
                .test(
                    crate::endpoint::drive::files::Request::builder()
                        .folder_id(folder.id)
                        .limit(1)
                        .build(),
                )
                .await;
            if let Some(file) = files.into_iter().next() {
                break file;
            }
        }
    }

    #[cfg(feature = "12-9-0")]
    async fn add_emoji_from_url(&self, url: Url) -> Id<Emoji> {
        let file = self.upload_from_url(url).await;
        self.test(crate::endpoint::admin::emoji::add::Request { file_id: file.id })
            .await
            .id
    }

    #[cfg(not(feature = "12-9-0"))]
    async fn add_emoji_from_url(&self, url: Url) -> Id<Emoji> {
        let ulid = Ulid::new().to_string();
        self.test(crate::endpoint::admin::emoji::add::Request {
            name: ulid,
            url,
            category: None,
            aliases: None,
        })
        .await
        .id
    }
}
