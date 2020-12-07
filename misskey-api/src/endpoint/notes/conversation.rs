use crate::model::{id::Id, note::Note};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub note_id: Id<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub offset: Option<u64>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes/conversation";
}

impl_offset_pagination!(Request, Note);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let note = client.create_note(Some("test"), None, None).await;
        client
            .test(Request {
                note_id: note.id,
                offset: None,
                limit: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        let note = client.create_note(Some("test"), None, None).await;
        client
            .test(Request {
                note_id: note.id,
                limit: Some(100),
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_offset() {
        let client = TestClient::new();
        let note = client.create_note(Some("test"), None, None).await;
        client
            .test(Request {
                note_id: note.id,
                limit: None,
                offset: Some(5),
            })
            .await;
    }
}
