use std::fmt::{self, Display};

use crate::model::{id::Id, note::Note, page::Page};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserField {
    pub name: String,
    pub value: String,
}

// packed `Emoji` for `User`
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserEmoji {
    pub name: String,
    pub url: Url,
    pub host: Option<String>,
    pub aliases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInstance {
    pub name: Option<String>,
    pub software_name: Option<String>,
    pub software_version: Option<String>,
    pub icon_url: Option<String>,
    pub favicon_url: Option<String>,
    pub theme_color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Id<User>,
    pub username: String,
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<Url>,
    pub avatar_url: Option<Url>,
    #[cfg(feature = "12-42-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-42-0")))]
    #[serde(default)]
    pub avatar_blurhash: Option<String>,
    #[cfg(not(feature = "12-42-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-42-0"))))]
    pub avatar_color: Option<String>,
    #[serde(default)]
    pub banner_url: Option<Url>,
    #[cfg(feature = "12-42-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-42-0")))]
    #[serde(default)]
    pub banner_blurhash: Option<String>,
    #[cfg(not(feature = "12-42-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-42-0"))))]
    pub banner_color: Option<String>,
    pub emojis: Option<Vec<UserEmoji>>,
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
    #[serde(default)]
    pub followers_count: Option<u64>,
    #[serde(default)]
    pub following_count: Option<u64>,
    #[serde(default)]
    pub notes_count: Option<u64>,
    #[serde(default = "default_false")]
    pub is_bot: bool,
    #[serde(default)]
    pub pinned_note_ids: Option<Vec<Id<Note>>>,
    #[serde(default)]
    pub pinned_notes: Option<Vec<Note>>,
    #[serde(default)]
    pub pinned_page_id: Option<Id<Page>>,
    #[serde(default)]
    pub pinned_page: Option<Page>,
    #[serde(default = "default_false")]
    pub is_cat: bool,
    #[serde(default = "default_false")]
    pub is_admin: bool,
    #[serde(default = "default_false")]
    pub is_moderator: bool,
    #[serde(default)]
    pub is_locked: Option<bool>,
    #[serde(default)]
    pub is_silenced: Option<bool>,
    #[serde(default)]
    pub is_suspended: Option<bool>,
    #[cfg(feature = "12-63-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-63-0")))]
    #[serde(default)]
    pub is_explorable: Option<bool>,
    #[serde(default)]
    pub has_unread_specified_notes: Option<bool>,
    #[serde(default)]
    pub has_unread_mentions: Option<bool>,
    #[serde(default)]
    pub has_unread_channel: Option<bool>,
    #[serde(default)]
    pub two_factor_enabled: Option<bool>,
    #[serde(default)]
    pub use_password_less_login: Option<bool>,
    #[serde(default)]
    pub security_keys: Option<bool>,
    #[serde(default)]
    pub fields: Option<Vec<UserField>>,
    #[cfg(feature = "12-51-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-51-0")))]
    #[serde(default)]
    pub instance: Option<UserInstance>,
    #[cfg(feature = "12-60-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-60-0")))]
    #[serde(default)]
    pub no_crawle: Option<bool>,
}

fn default_false() -> bool {
    false
}

impl_entity!(User);

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum UserSortKey {
    Follower,
    CreatedAt,
    UpdatedAt,
}

impl Display for UserSortKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserSortKey::Follower => f.write_str("follower"),
            UserSortKey::CreatedAt => f.write_str("createdAt"),
            UserSortKey::UpdatedAt => f.write_str("updatedAt"),
        }
    }
}

#[derive(Debug, Error, Clone)]
#[error("invalid sort key")]
pub struct ParseUserSortKeyError {
    _priv: (),
}

impl std::str::FromStr for UserSortKey {
    type Err = ParseUserSortKeyError;

    fn from_str(s: &str) -> Result<UserSortKey, Self::Err> {
        match s {
            "follower" | "Follower" => Ok(UserSortKey::Follower),
            "createdAt" | "CreatedAt" => Ok(UserSortKey::CreatedAt),
            "updatedAt" | "UpdatedAt" => Ok(UserSortKey::UpdatedAt),
            _ => Err(ParseUserSortKeyError { _priv: () }),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum UserOrigin {
    Local,
    Remote,
    Combined,
}

#[derive(Debug, Error, Clone)]
#[error("invalid user origin")]
pub struct ParseUserOriginError {
    _priv: (),
}

impl std::str::FromStr for UserOrigin {
    type Err = ParseUserOriginError;

    fn from_str(s: &str) -> Result<UserOrigin, Self::Err> {
        match s {
            "local" | "Local" => Ok(UserOrigin::Local),
            "remote" | "Remote" => Ok(UserOrigin::Remote),
            "combined" | "Combined" => Ok(UserOrigin::Combined),
            _ => Err(ParseUserOriginError { _priv: () }),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRelation {
    pub id: Id<User>,
    pub is_following: bool,
    pub has_pending_follow_request_from_you: bool,
    pub has_pending_follow_request_to_you: bool,
    pub is_followed: bool,
    pub is_blocking: bool,
    pub is_blocked: bool,
    pub is_muted: bool,
}
