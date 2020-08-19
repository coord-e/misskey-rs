use crate::model::{
    file::FileId,
    note::{Note, NoteId, Visibility},
    user::UserId,
};

use chrono::{DateTime, Utc};
use misskey_core::ApiRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PollRequest {
    pub choices: Vec<String>,
    pub multiple: bool,
    pub expires_at: Option<DateTime<Utc>>,
    // pub expired_after: Option<DateTime<Utc>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub visible_user_ids: Vec<UserId>,
    pub text: Option<String>,
    pub cw: Option<String>,
    pub via_mobile: bool,
    pub local_only: bool,
    pub no_extract_mentions: bool,
    pub no_extract_hashtags: bool,
    pub no_extract_emojis: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_ids: Vec<FileId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_id: Option<NoteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renote_id: Option<NoteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<PollRequest>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub created_note: Note,
}

impl ApiRequest for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "notes/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::TestClient;

    #[tokio::test]
    async fn test_text() {
        let mut client = TestClient::new();
        client
            .test(Request {
                visibility: None,
                visible_user_ids: Vec::new(),
                text: Some("some text".to_string()),
                cw: None,
                via_mobile: false,
                local_only: false,
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                file_ids: Vec::new(),
                reply_id: None,
                renote_id: None,
                poll: None,
            })
            .await;
    }

    #[tokio::test]
    async fn test_cw() {
        let mut client = TestClient::new();
        client
            .test(Request {
                visibility: None,
                visible_user_ids: Vec::new(),
                text: Some("!".to_string()),
                cw: Some("nsfw".to_string()),
                via_mobile: false,
                local_only: false,
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                file_ids: Vec::new(),
                reply_id: None,
                renote_id: None,
                poll: None,
            })
            .await;
    }

    #[tokio::test]
    async fn test_visibilty() {
        use crate::model::note::Visibility;

        let mut client = TestClient::new();
        client
            .test(Request {
                visibility: Some(Visibility::Home),
                visible_user_ids: Vec::new(),
                text: Some("hello home".to_string()),
                cw: None,
                via_mobile: false,
                local_only: false,
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                file_ids: Vec::new(),
                reply_id: None,
                renote_id: None,
                poll: None,
            })
            .await;
        client
            .test(Request {
                visibility: Some(Visibility::Public),
                visible_user_ids: Vec::new(),
                text: Some("hello public".to_string()),
                cw: None,
                via_mobile: false,
                local_only: false,
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                file_ids: Vec::new(),
                reply_id: None,
                renote_id: None,
                poll: None,
            })
            .await;
        client
            .test(Request {
                visibility: Some(Visibility::Followers),
                visible_user_ids: Vec::new(),
                text: Some("hello followers".to_string()),
                cw: None,
                via_mobile: false,
                local_only: false,
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                file_ids: Vec::new(),
                reply_id: None,
                renote_id: None,
                poll: None,
            })
            .await;
        client
            .test(Request {
                visibility: Some(Visibility::Specified),
                visible_user_ids: Vec::new(), // TODO: specify some users
                text: Some("hello specific person".to_string()),
                cw: None,
                via_mobile: false,
                local_only: false,
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                file_ids: Vec::new(),
                reply_id: None,
                renote_id: None,
                poll: None,
            })
            .await;
    }

    #[tokio::test]
    async fn test_renote() {
        let mut client = TestClient::new();
        let note = client
            .test(Request {
                visibility: None,
                visible_user_ids: Vec::new(),
                text: Some("renote".to_string()),
                cw: None,
                via_mobile: false,
                local_only: false,
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                file_ids: Vec::new(),
                reply_id: None,
                renote_id: None,
                poll: None,
            })
            .await
            .created_note;
        client
            .test(Request {
                visibility: None,
                visible_user_ids: Vec::new(),
                text: None,
                cw: None,
                via_mobile: false,
                local_only: false,
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                file_ids: Vec::new(),
                reply_id: None,
                renote_id: Some(note.id),
                poll: None,
            })
            .await;
    }

    #[tokio::test]
    async fn test_reply() {
        let mut client = TestClient::new();
        let note = client
            .test(Request {
                visibility: None,
                visible_user_ids: Vec::new(),
                text: Some("reply".to_string()),
                cw: None,
                via_mobile: false,
                local_only: false,
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                file_ids: Vec::new(),
                reply_id: None,
                renote_id: None,
                poll: None,
            })
            .await
            .created_note;
        client
            .test(Request {
                visibility: None,
                visible_user_ids: Vec::new(),
                text: Some("hey".to_string()),
                cw: None,
                via_mobile: false,
                local_only: false,
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                file_ids: Vec::new(),
                reply_id: Some(note.id),
                renote_id: None,
                poll: None,
            })
            .await;
    }
}
