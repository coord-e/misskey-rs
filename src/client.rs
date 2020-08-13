use crate::api::{self, ApiRequest};
use crate::error::Result;
use crate::model::note::{Note, Visibility};

pub mod http;

#[async_trait::async_trait]
pub trait Client {
    async fn request<R: ApiRequest + Send>(&mut self, request: R) -> Result<R::Response>;
}

#[async_trait::async_trait]
pub trait ClientExt {
    async fn list_notes(&mut self) -> Result<Vec<Note>>;
    async fn create_note(&mut self, text: String) -> Result<Note>;
}

#[async_trait::async_trait]
impl<T: Client + Send> ClientExt for T {
    async fn list_notes(&mut self) -> Result<Vec<Note>> {
        let request = api::notes::Request {
            local: false,
            reply: false,
            renote: false,
            with_files: false,
            poll: false,
            limit: None,
            since_id: None,
            until_id: None,
        };
        self.request(request).await
    }

    async fn create_note(&mut self, text: String) -> Result<Note> {
        let request = api::notes::create::Request {
            visibility: Some(Visibility::Public),
            visible_user_ids: Vec::new(),
            text: Some(text),
            cw: None,
            via_mobile: false,
            local_only: false,
            no_extract_mentions: false,
            no_extract_hashtags: false,
            no_extract_emojis: false,
            file_ids: Vec::new(),
            reply_id: None,
            renote_id: None,
            poll: None,
        };
        let response = self.request(request).await?;
        Ok(response.created_note)
    }
}
