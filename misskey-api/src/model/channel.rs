#[cfg(feature = "13-11-0")]
use crate::model::note::Note;
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
    #[cfg(feature = "13-11-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-11-0")))]
    pub pinned_note_ids: Vec<Id<Note>>,
    pub notes_count: u64,
    pub users_count: u64,
    pub user_id: Id<User>,
    #[serde(default)]
    pub is_following: Option<bool>,
    #[cfg(feature = "13-11-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-11-0")))]
    #[serde(default)]
    pub is_favorited: Option<bool>,
    #[serde(default)]
    pub has_unread_note: Option<bool>,
    #[cfg(feature = "13-11-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-11-0")))]
    #[serde(default)]
    pub pinned_notes: Option<Vec<Note>>,
}

impl_entity!(Channel);
