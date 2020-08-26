use crate::model::{note::NoteId, note::Reaction, user::UserId};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct Request {
    pub id: NoteId,
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

impl misskey_core::streaming::SubscriptionRequest for Request {
    type Item = NoteUpdateEvent;
    const TYPE: &'static str = "subNote";
}

impl misskey_core::streaming::SubscriptionItem for NoteUpdateEvent {
    const TYPE: &'static str = "noteUpdated";
    const UNSUBSCRIBE_REQUEST_TYPE: &'static str = "unsubNote";
}
