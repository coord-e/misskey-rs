use std::error::Error;

use crate::api::{self, ApiRequest};
use crate::model::note::{Note, NoteId, ReactionType, Visibility};

use url::Url;

pub mod http;
pub use http::HttpClient;

#[async_trait::async_trait]
pub trait ClientBuilder {
    type Client: Client;

    fn new(url: Url) -> Self;
    fn token<'a, S: Into<String>>(&'a mut self, token: S) -> &'a mut Self;
    async fn build(&self) -> Result<Self::Client, <Self::Client as Client>::Error>;
}

#[async_trait::async_trait]
pub trait Client {
    type Error: Error;

    async fn request<R: ApiRequest + Send>(
        &mut self,
        request: R,
    ) -> Result<R::Response, Self::Error>;
}

#[async_trait::async_trait]
pub trait ClientExt: Client {
    async fn list_notes(&mut self) -> Result<Vec<Note>, Self::Error>;
    async fn create_note(&mut self, text: String) -> Result<Note, Self::Error>;
    async fn create_reaction(
        &mut self,
        note_id: NoteId,
        reaction: ReactionType,
    ) -> Result<(), Self::Error>;
}

#[async_trait::async_trait]
impl<T: Client + Send> ClientExt for T {
    async fn list_notes(&mut self) -> Result<Vec<Note>, T::Error> {
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

    async fn create_note(&mut self, text: String) -> Result<Note, T::Error> {
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

    async fn create_reaction(
        &mut self,
        note_id: NoteId,
        reaction: ReactionType,
    ) -> Result<(), T::Error> {
        let request = api::notes::reactions::create::Request { note_id, reaction };
        self.request(request).await
    }
}
