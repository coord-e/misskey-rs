use crate::model::user::UserId;

use chrono::{DateTime, Utc};
use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct ChannelId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: ChannelId,
    pub created_at: DateTime<Utc>,
    pub last_noted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: Option<String>,
    pub banner_id: Option<Url>,
    pub notes_count: u64,
    pub users_count: u64,
    pub user_id: UserId,
    #[serde(default)]
    pub is_following: Option<bool>,
    #[serde(default)]
    pub has_unread_note: Option<bool>,
}
