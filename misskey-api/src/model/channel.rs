use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: Id<Channel>,
    pub created_at: DateTime<Utc>,
    pub last_noted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: Option<String>,
    pub banner_url: Option<Url>,
    pub notes_count: u64,
    pub users_count: u64,
    pub user_id: Id<User>,
    #[serde(default)]
    pub is_following: Option<bool>,
    #[serde(default)]
    pub has_unread_note: Option<bool>,
}

impl_entity!(Channel);
