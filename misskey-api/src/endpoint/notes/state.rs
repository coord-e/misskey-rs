use crate::model::{id::Id, note::Note};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: Id<Note>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub is_favorited: bool,
    pub is_watching: bool,
    #[cfg(feature = "12-95-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-95-0")))]
    pub is_muted_thread: bool,
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
        let client = TestClient::new();
        let note = client.create_note(Some("test"), None, None).await;

        client.test(Request { note_id: note.id }).await;
    }
}
