use crate::model::{
    id::Id,
    note::{Note, Reaction},
    note_reaction::NoteReaction,
};

use serde::Serialize;
use typed_builder::TypedBuilder;

pub mod create;
pub mod delete;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub note_id: Id<Note>,
    #[serde(rename = "type")]
    #[builder(default, setter(strip_option, into))]
    pub type_: Option<Reaction>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub offset: Option<u64>,
}

impl misskey_core::Request for Request {
    type Response = Vec<NoteReaction>;
    const ENDPOINT: &'static str = "notes/reactions";
}

impl_offset_pagination!(Request, NoteReaction);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_simple() {
        let client = TestClient::new();
        let note = client.user.create_note(Some("hello"), None, None).await;
        client
            .user
            .test(Request {
                note_id: note.id,
                type_: None,
                limit: None,
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_type() {
        let client = TestClient::new();
        let note = client.user.create_note(Some("hello"), None, None).await;
        client
            .admin
            .test(crate::endpoint::notes::reactions::create::Request {
                note_id: note.id,
                reaction: "👍".into(),
            })
            .await;

        client
            .user
            .test(Request {
                note_id: note.id,
                type_: Some("👍".into()),
                limit: None,
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        let note = client.user.create_note(Some("hello"), None, None).await;
        client
            .admin
            .test(crate::endpoint::notes::reactions::create::Request {
                note_id: note.id,
                reaction: "👍".into(),
            })
            .await;

        client
            .user
            .test(Request {
                note_id: note.id,
                type_: None,
                limit: Some(100),
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_offset() {
        let client = TestClient::new();
        let note = client.user.create_note(Some("hello"), None, None).await;
        client
            .admin
            .test(crate::endpoint::notes::reactions::create::Request {
                note_id: note.id,
                reaction: "👍".into(),
            })
            .await;

        client
            .user
            .test(Request {
                note_id: note.id,
                type_: None,
                limit: None,
                offset: Some(1),
            })
            .await;
    }
}
