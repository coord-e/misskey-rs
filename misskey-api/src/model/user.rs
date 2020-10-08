use crate::model::note::{Note, NoteId};

use chrono::{DateTime, Utc};
use derive_more::{Display, Error, FromStr};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct UserId(pub String);

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
pub struct User {
    pub id: UserId,
    pub username: String,
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<Url>,
    pub avatar_url: Option<Url>,
    pub avatar_blurhash: Option<String>,
    #[serde(default)]
    pub banner_url: Option<Url>,
    #[serde(default)]
    pub banner_blurhash: Option<String>,
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
    #[serde(default = "default_false")]
    pub has_unread_channel: bool,
    #[serde(default)]
    pub fields: Vec<UserField>,
}

fn default_false() -> bool {
    false
}

fn default_zero() -> u64 {
    0
}

impl_entity!(User, UserId);

#[derive(PartialEq, Eq, Clone, Debug, Copy, Display)]
pub enum UserSort {
    #[display(fmt = "follower")]
    Follower,
    #[display(fmt = "createdAt")]
    CreatedAt,
    #[display(fmt = "updatedAt")]
    UpdatedAt,
}

#[derive(Debug, Display, Error, Clone)]
#[display(fmt = "invalid sort key")]
pub struct ParseUserSortError;

impl std::str::FromStr for UserSort {
    type Err = ParseUserSortError;

    fn from_str(s: &str) -> Result<UserSort, Self::Err> {
        match s {
            "follower" | "Follower" => Ok(UserSort::Follower),
            "createdAt" | "CreatedAt" => Ok(UserSort::CreatedAt),
            "updatedAt" | "UpdatedAt" => Ok(UserSort::UpdatedAt),
            _ => Err(ParseUserSortError),
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

#[derive(Debug, Display, Error, Clone)]
#[display(fmt = "invalid user origin")]
pub struct ParseUserOriginError;

impl std::str::FromStr for UserOrigin {
    type Err = ParseUserOriginError;

    fn from_str(s: &str) -> Result<UserOrigin, Self::Err> {
        match s {
            "local" | "Local" => Ok(UserOrigin::Local),
            "remote" | "Remote" => Ok(UserOrigin::Remote),
            "combined" | "Combined" => Ok(UserOrigin::Combined),
            _ => Err(ParseUserOriginError),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRelation {
    pub id: UserId,
    pub is_following: bool,
    pub has_pending_follow_request_from_you: bool,
    pub has_pending_follow_request_to_you: bool,
    pub is_followed: bool,
    pub is_blocking: bool,
    pub is_blocked: bool,
    pub is_muted: bool,
}
