use crate::model::note::{Note, NoteId};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
}

impl ApiRequest for Request {
    type Response = Note;
    const ENDPOINT: &'static str = "notes/show";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let note = client.create_note(Some("test"), None, None).await;

        client.test(Request { note_id: note.id }).await;
    }
}