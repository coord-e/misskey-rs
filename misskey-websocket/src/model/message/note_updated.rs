use chrono::{DateTime, Utc};
use misskey_api::model::{
    note::{NoteId, Reaction},
    user::UserId,
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct NoteUpdatedMessage {
    pub id: NoteId,
    #[serde(flatten)]
    pub event: NoteUpdateEvent,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum NoteUpdateEvent {
    #[serde(rename_all = "camelCase")]
    Reacted { reaction: Reaction, user_id: UserId },
    #[serde(rename_all = "camelCase")]
    Unreacted { reaction: Reaction, user_id: UserId },
    #[serde(rename_all = "camelCase")]
    Deleted { deleted_at: DateTime<Utc> },
    #[serde(rename_all = "camelCase")]
    PollVoted { choice: u64, user_id: UserId },
}
