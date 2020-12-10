use crate::model::{clip::Clip, id::Id, note::Note};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: Id<Note>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Clip>;
    const ENDPOINT: &'static str = "notes/clips";
}

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
        let note = client.user.create_note(Some("test"), None, None).await;

        client
            .test(crate::endpoint::clips::add_note::Request {
                clip_id: clip.id,
                note_id: note.id,
            })
            .await;

        client.test(Request { note_id: note.id }).await;
    }
}
