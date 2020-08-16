use crate::model::note::{Note, NoteId};

use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use url::Url;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct UserId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<Url>,
    pub avatar_url: Option<Url>,
    pub avatar_blurhash: Option<Value>,
    #[serde(default)]
    pub banner_url: Option<Url>,
    #[serde(default)]
    pub banner_blurhash: Option<Value>,
    pub emojis: Option<Value>,
    pub host: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub birthday: Option<String>,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default = "default_zero")]
    pub followers_count: u64,
    #[serde(default = "default_zero")]
    pub following_count: u64,
    #[serde(default = "default_zero")]
    pub notes_count: u64,
    #[serde(default = "default_false")]
    pub is_bot: bool,
    #[serde(default)]
    pub pinned_note_ids: Vec<NoteId>,
    #[serde(default)]
    pub pinned_notes: Vec<Note>,
    #[serde(default = "default_false")]
    pub is_cat: bool,
    #[serde(default = "default_false")]
    pub is_admin: bool,
    #[serde(default = "default_false")]
    pub is_moderator: bool,
    #[serde(default = "default_false")]
    pub is_locked: bool,
    #[serde(default = "default_false")]
    pub has_unread_specified_notes: bool,
    #[serde(default = "default_false")]
    pub has_unread_mentions: bool,
}

fn default_false() -> bool {
    false
}

fn default_zero() -> u64 {
    0
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct FollowRequestId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowRequest {
    pub id: FollowRequestId,
    pub followee: User,
    pub follower: User,
}
