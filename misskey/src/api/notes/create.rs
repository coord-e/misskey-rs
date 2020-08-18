use crate::api::ApiRequest;
use crate::model::{
    file::FileId,
    note::{Note, NoteId, Visibility},
    user::UserId,
};

use chrono::{DateTime, Utc};
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
