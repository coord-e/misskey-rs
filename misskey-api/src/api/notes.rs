use crate::model::note::{Note, NoteId};

use misskey_core::ApiRequest;
use serde::Serialize;

pub mod children;
pub mod conversation;
pub mod create;
pub mod delete;
pub mod favorites;
pub mod featured;
pub mod global_timeline;
pub mod hybrid_timeline;
pub mod local_timeline;
pub mod mentions;
pub mod polls;
pub mod reactions;
pub mod renotes;
pub mod replies;
pub mod search;
pub mod search_by_tag;
pub mod show;
pub mod state;
pub mod timeline;
pub mod unrenote;
pub mod user_list_timeline;
pub mod watching;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub local: bool,
    pub reply: bool,
    pub renote: bool,
    pub with_files: bool,
    pub poll: bool,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<NoteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<NoteId>,
}

impl ApiRequest for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::TestClient;

    #[tokio::test]
    async fn test_list() {
        let mut client = TestClient::new();
        client
            .test(Request {
                local: false,
                reply: false,
                renote: false,
                with_files: false,
                poll: false,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn test_list_with_limit() {
        let mut client = TestClient::new();
        client
            .test(Request {
                local: false,
                reply: false,
                renote: false,
                with_files: false,
                poll: false,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn test_list_paginate() {
        let mut client = TestClient::new();

        let note = client
            .test(crate::api::notes::create::Request {
                visibility: None,
                visible_user_ids: Vec::new(),
                text: Some("test".to_string()),
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
            })
            .await
            .created_note;

        client
            .test(Request {
                local: false,
                reply: false,
                renote: false,
                with_files: false,
                poll: false,
                limit: None,
                since_id: Some(note.id.clone()),
                until_id: Some(note.id.clone()),
            })
            .await;
    }
}
