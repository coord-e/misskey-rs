use crate::model::note::NoteId;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
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
        let mut client = TestClient::new();

        let poll = crate::api::notes::create::PollRequest {
            choices: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            multiple: None,
            expires_at: None,
        };
        let note = client
            .test(crate::api::notes::create::Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("poll".to_string()),
                cw: None,
                via_mobile: None,
                local_only: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: Some(poll),
            })
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
