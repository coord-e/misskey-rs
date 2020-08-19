use crate::model::note::NoteId;

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
    pub choice: u64,
}

impl ApiRequest for Request {
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
                visible_user_ids: Vec::new(),
                text: Some("poll".to_string()),
                cw: None,
                via_mobile: false,
                local_only: false,
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                file_ids: Vec::new(),
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
