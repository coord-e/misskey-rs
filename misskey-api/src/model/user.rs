#[cfg(feature = "13-2-4")]
use std::collections::HashMap;
#[cfg(feature = "12-48-0")]
use std::collections::HashSet;
use std::fmt::{self, Display};

#[cfg(feature = "12-48-0")]
use crate::model::notification::NotificationType;
use crate::model::{id::Id, note::Note, page::Page};

#[cfg(feature = "13-1-0")]
use chrono::serde::ts_milliseconds;
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

#[cfg(not(feature = "13-0-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
// packed `Emoji` for `User`
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserEmoji {
    pub name: String,
    pub url: Url,
    #[cfg(not(feature = "12-75-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-75-0"))))]
    pub host: Option<String>,
    #[cfg(not(feature = "12-75-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-75-0"))))]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Copy, Hash)]
#[serde(rename_all = "camelCase")]
pub enum UserEmailNotificationType {
    Follow,
    ReceiveFollowRequest,
    Mention,
    Reply,
    Quote,
    GroupInvited,
}

impl Display for UserEmailNotificationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserEmailNotificationType::Follow => f.write_str("follow"),
            UserEmailNotificationType::ReceiveFollowRequest => f.write_str("receiveFollowRequest"),
            UserEmailNotificationType::Mention => f.write_str("mention"),
            UserEmailNotificationType::Reply => f.write_str("reply"),
            UserEmailNotificationType::Quote => f.write_str("quote"),
            UserEmailNotificationType::GroupInvited => f.write_str("groupInvited"),
        }
    }
}

#[derive(Debug, Error, Clone)]
#[error("invalid email notification type")]
pub struct ParseUserEmailNotificationType {
    _priv: (),
}

impl std::str::FromStr for UserEmailNotificationType {
    type Err = ParseUserEmailNotificationType;

    fn from_str(s: &str) -> Result<UserEmailNotificationType, Self::Err> {
        match s {
            "follow" | "Follow" => Ok(UserEmailNotificationType::Follow),
            "receiveFollowRequest" | "ReceiveFollowRequest" => {
                Ok(UserEmailNotificationType::ReceiveFollowRequest)
            }
            "mention" | "Mention" => Ok(UserEmailNotificationType::Mention),
            "reply" | "Reply" => Ok(UserEmailNotificationType::Reply),
            "quote" | "Quote" => Ok(UserEmailNotificationType::Quote),
            "groupInvited" | "GroupInvited" => Ok(UserEmailNotificationType::GroupInvited),
            _ => Err(ParseUserEmailNotificationType { _priv: () }),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Copy, Hash)]
#[serde(rename_all = "camelCase")]
pub enum OnlineStatus {
    Unknown,
    Online,
    Active,
    Offline,
}

impl Display for OnlineStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OnlineStatus::Unknown => f.write_str("unknown"),
            OnlineStatus::Online => f.write_str("online"),
            OnlineStatus::Active => f.write_str("active"),
            OnlineStatus::Offline => f.write_str("offline"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Copy, Hash)]
#[serde(rename_all = "camelCase")]
pub enum FfVisibility {
    Public,
    Followers,
    Private,
}

impl Display for FfVisibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FfVisibility::Public => f.write_str("public"),
            FfVisibility::Followers => f.write_str("followers"),
            FfVisibility::Private => f.write_str("private"),
        }
    }
}

#[derive(Debug, Error, Clone)]
#[error("invalid ff visibility")]
pub struct ParseFfVisibilityError {
    _priv: (),
}

impl std::str::FromStr for FfVisibility {
    type Err = ParseFfVisibilityError;

    fn from_str(s: &str) -> Result<FfVisibility, Self::Err> {
        match s {
            "public" | "Public" => Ok(FfVisibility::Public),
            "followers" | "Followers" => Ok(FfVisibility::Followers),
            "private" | "Private" => Ok(FfVisibility::Private),
            _ => Err(ParseFfVisibilityError { _priv: () }),
        }
    }
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
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub emojis: Option<Vec<UserEmoji>>,
    #[cfg(feature = "13-2-4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-2-4")))]
    #[serde(default)]
    pub emojis: Option<HashMap<String, Url>>,
    pub host: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub birthday: Option<String>,
    #[cfg(feature = "12-70-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-70-0")))]
    #[serde(default)]
    pub lang: Option<String>,
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
    #[cfg(feature = "12-104-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-104-0")))]
    #[serde(default = "default_false")]
    pub show_timeline_replies: bool,
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
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    #[serde(default)]
    pub muting_notification_types: Option<HashSet<NotificationType>>,
    #[cfg(feature = "12-70-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-70-0")))]
    #[serde(default)]
    pub email_notification_types: Option<HashSet<UserEmailNotificationType>>,
    #[cfg(feature = "12-77-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-77-0")))]
    #[serde(default)]
    pub online_status: Option<OnlineStatus>,
    #[cfg(feature = "12-77-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-77-0")))]
    #[serde(default)]
    pub hide_online_status: Option<bool>,
    #[cfg(feature = "12-96-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-96-0")))]
    #[serde(default)]
    pub ff_visibility: Option<FfVisibility>,
    #[cfg(feature = "12-99-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-99-0")))]
    #[serde(default)]
    pub muted_instances: Option<Vec<String>>,
    #[cfg(feature = "13-1-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-1-0")))]
    #[serde(default)]
    pub achievements: Option<Vec<Achievement>>,
    #[cfg(feature = "13-1-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-1-0")))]
    #[serde(default)]
    pub logged_in_dates: Option<u64>,
    #[cfg(feature = "13-4-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-4-0")))]
    pub badge_roles: Option<Vec<BadgeRole>>,
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

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum AdminUserSortKey {
    Follower,
    CreatedAt,
    UpdatedAt,
    LastActiveDate,
}

impl Display for AdminUserSortKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AdminUserSortKey::Follower => f.write_str("follower"),
            AdminUserSortKey::CreatedAt => f.write_str("createdAt"),
            AdminUserSortKey::UpdatedAt => f.write_str("updatedAt"),
            AdminUserSortKey::LastActiveDate => f.write_str("lastActiveDate"),
        }
    }
}

#[derive(Debug, Error, Clone)]
#[error("invalid sort key")]
pub struct ParseAdminUserSortKeyError {
    _priv: (),
}

impl std::str::FromStr for AdminUserSortKey {
    type Err = ParseAdminUserSortKeyError;

    fn from_str(s: &str) -> Result<AdminUserSortKey, Self::Err> {
        match s {
            "follower" | "Follower" => Ok(AdminUserSortKey::Follower),
            "createdAt" | "CreatedAt" => Ok(AdminUserSortKey::CreatedAt),
            "updatedAt" | "UpdatedAt" => Ok(AdminUserSortKey::UpdatedAt),
            "lastActiveDate" | "LastActiveDate" => Ok(AdminUserSortKey::LastActiveDate),
            _ => Err(ParseAdminUserSortKeyError { _priv: () }),
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

#[cfg(all(feature = "12-111-0", not(feature = "13-3-0")))]
#[cfg_attr(docsrs, doc(all(feature = "12-111-0", not(feature = "13-3-0"))))]
pub type IntegrationValue = serde_json::Value;

#[cfg(feature = "13-1-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-1-0")))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
    pub name: String,
    #[serde(with = "ts_milliseconds")]
    pub unlocked_at: DateTime<Utc>,
}

#[cfg(feature = "13-4-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-4-0")))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BadgeRole {
    pub name: String,
    pub icon_url: String,
}
