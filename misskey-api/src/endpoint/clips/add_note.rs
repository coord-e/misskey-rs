use crate::model::{clip::Clip, id::Id, note::Note};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub clip_id: Id<Clip>,
    pub note_id: Id<Note>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "clips/add-note";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let note = client.user.create_note(Some("test"), None, None).await;
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
                note_id: note.id,
            })
            .await;
    }
}
