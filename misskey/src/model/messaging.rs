use crate::model::{
    file::{DriveFile, FileId},
    user::{User, UserId},
};

use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct UserGroupId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGroup {
    pub id: UserGroupId,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub owner_id: UserId,
    #[serde(default)]
    pub user_ids: Vec<UserId>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct MessagingMessageId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessagingMessage {
    pub id: MessagingMessageId,
    pub created_at: DateTime<Utc>,
    pub user_id: UserId,
    pub user: User,
    pub text: Option<String>,
    #[serde(default)]
    pub file_id: Option<FileId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<DriveFile>,
    pub recipient_id: Option<UserId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<User>,
    pub group_id: Option<UserGroupId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<UserGroup>,
    #[serde(default = "default_false")]
    pub is_read: bool,
    #[serde(default)]
    pub reads: Vec<UserId>,
}

fn default_false() -> bool {
    false
}
