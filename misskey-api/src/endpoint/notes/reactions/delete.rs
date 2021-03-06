use crate::model::{id::Id, note::Note};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: Id<Note>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "notes/reactions/delete";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        use crate::model::note::Reaction;

        let client = TestClient::new();
        let note = client.admin.create_note(Some("test"), None, None).await;

        client
            .user
            .test(crate::endpoint::notes::reactions::create::Request {
                note_id: note.id.clone(),
                reaction: Reaction("👍".to_string()),
            })
            .await;

        client.user.test(Request { note_id: note.id }).await;
    }
}
