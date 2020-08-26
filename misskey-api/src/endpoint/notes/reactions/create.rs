use crate::model::note::{NoteId, Reaction};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
    pub reaction: Reaction,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "notes/reactions/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        use crate::model::note::Reaction;

        let mut client = TestClient::new();
        let note = client.admin.create_note(Some("test"), None, None).await;

        client
            .user
            .test(Request {
                note_id: note.id,
                reaction: Reaction("👍".to_string()),
            })
            .await;
    }
}
