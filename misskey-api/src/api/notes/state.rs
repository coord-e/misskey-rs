use crate::model::note::NoteId;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub is_favorited: bool,
    pub is_watching: bool,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "notes/state";
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
