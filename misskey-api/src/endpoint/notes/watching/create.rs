use crate::model::note::NoteId;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "notes/watching/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let note = client.admin.create_note(Some("test"), None, None).await;
        client.user.test(Request { note_id: note.id }).await;
    }
}
