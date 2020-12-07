use crate::model::{id::Id, note::Note, user::User};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: Id<Note>,
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
        let client = TestClient::new();
        let note = client.create_note(Some("test"), None, None).await;
        client.test(Request { note_id: note.id }).await;
    }
}
