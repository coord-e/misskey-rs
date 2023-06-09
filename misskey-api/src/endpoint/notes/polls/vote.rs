use crate::model::{id::Id, note::Note};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: Id<Note>,
    pub choice: u64,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "notes/polls/vote";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();

        let poll = crate::endpoint::notes::create::PollRequest {
            choices: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            multiple: None,
            expires_at: None,
            expired_after: None,
        };
        let note = client
            .test(
                crate::endpoint::notes::create::Request::builder()
                    .text("poll")
                    .poll(poll)
                    .build(),
            )
            .await
            .created_note;

        client
            .test(Request {
                note_id: note.id,
                choice: 1,
            })
            .await;
    }
}
