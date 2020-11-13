use crate::model::{drive::DriveFile, id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGroup {
    pub id: Id<UserGroup>,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub owner_id: Id<User>,
    #[serde(default)]
    pub user_ids: Vec<Id<User>>,
}

impl_entity!(UserGroup);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessagingMessage {
    pub id: Id<MessagingMessage>,
    pub created_at: DateTime<Utc>,
    pub user_id: Id<User>,
    pub user: User,
    pub text: Option<String>,
    #[serde(default)]
    pub file_id: Option<Id<DriveFile>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<DriveFile>,
    pub recipient_id: Option<Id<User>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<User>,
    pub group_id: Option<Id<UserGroup>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<UserGroup>,
    #[serde(default = "default_false")]
    pub is_read: bool,
    #[serde(default)]
    pub reads: Vec<Id<User>>,
}

fn default_false() -> bool {
    false
}

impl_entity!(MessagingMessage);
