use crate::model::{note::NoteId, user::User};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
}

impl misskey_core::Request for Request {
    type Response = User;
    const ENDPOINT: &'static str = "i/pin";
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