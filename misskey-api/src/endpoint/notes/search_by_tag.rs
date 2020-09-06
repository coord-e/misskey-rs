use crate::model::note::{Note, NoteId, Tag};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tag: Option<Tag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub query: Option<Vec<Vec<Tag>>>,
    #[builder(default, setter(strip_option))]
    pub reply: Option<bool>,
    #[builder(default, setter(strip_option))]
    pub renote: Option<bool>,
    #[builder(default, setter(strip_option))]
    pub poll: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub with_files: Option<bool>,
    /// 1 .. 100 (1 .. 30 in ~12.19.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<NoteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<NoteId>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes/search-by-tag";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_with_tag() {
        use crate::model::note::Tag;

        let client = TestClient::new();
        client
            .test(Request {
                tag: Some(Tag("tag".to_string())),
                query: None,
                reply: None,
                renote: None,
                poll: None,
                with_files: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_query() {
        use crate::model::note::Tag;

        let client = TestClient::new();
        client
            .test(Request {
                tag: None,
                query: Some(vec![
                    vec![Tag("tag1".to_string()), Tag("tag2".to_string())],
                    vec![Tag("tag3".to_string())],
                ]),
                reply: None,
                renote: None,
                poll: None,
                with_files: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        use crate::model::note::Tag;

        let client = TestClient::new();
        client
            .test(Request {
                tag: Some(Tag("tag".to_string())),
                query: None,
                reply: Some(true),
                renote: Some(true),
                poll: Some(true),
                with_files: Some(true),
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        use crate::model::note::Tag;

        let client = TestClient::new();
        client
            .test(Request {
                tag: Some(Tag("tag".to_string())),
                query: None,
                reply: None,
                renote: None,
                poll: None,
                with_files: None,
                limit: Some(30),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        use crate::model::note::Tag;

        let client = TestClient::new();
        let note = client.create_note(Some("#tag"), None, None).await;

        client
            .test(Request {
                tag: Some(Tag("tag".to_string())),
                query: None,
                reply: None,
                renote: None,
                poll: None,
                with_files: None,
                limit: None,
                since_id: Some(note.id.clone()),
                until_id: Some(note.id.clone()),
            })
            .await;
    }
}
