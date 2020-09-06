use crate::model::note::NoteId;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "notes/unrenote";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let note1 = client.create_note(Some("test"), None, None).await;
        let note2 = client.create_note(None, Some(note1.id), None).await;
        client.test(Request { note_id: note2.id }).await;
    }
}
