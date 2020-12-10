use crate::model::{clip::Clip, id::Id, note::Note};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub clip_id: Id<Clip>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<Note>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<Note>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "clips/notes";
}

impl_pagination!(Request, Note);

#[cfg(feature = "12-57-0")]
#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_simple() {
        let client = TestClient::new();
        let clip = client
            .user
            .test(crate::endpoint::clips::create::Request {
                name: "testclip".to_string(),
                is_public: None,
                description: None,
            })
            .await;
        client
            .test(Request {
                clip_id: clip.id,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        let clip = client
            .user
            .test(crate::endpoint::clips::create::Request {
                name: "testclip".to_string(),
                is_public: None,
                description: None,
            })
            .await;
        let note = client.user.create_note(Some("test"), None, None).await;

        client
            .test(crate::endpoint::clips::add_note::Request {
                clip_id: clip.id,
                note_id: note.id,
            })
            .await;

        client
            .test(Request {
                clip_id: clip.id,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let clip = client
            .user
            .test(crate::endpoint::clips::create::Request {
                name: "testclip".to_string(),
                is_public: None,
                description: None,
            })
            .await;
        let note = client.user.create_note(Some("test"), None, None).await;

        client
            .test(crate::endpoint::clips::add_note::Request {
                clip_id: clip.id,
                note_id: note.id,
            })
            .await;

        client
            .test(Request {
                clip_id: clip.id,
                limit: None,
                since_id: Some(note.id),
                until_id: Some(note.id),
            })
            .await;
    }
}
