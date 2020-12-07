use crate::model::{drive::DriveFile, id::Id, user::User, user_group::UserGroup};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub file: Option<DriveFile>,
    pub recipient_id: Option<Id<User>>,
    #[serde(default)]
    pub recipient: Option<User>,
    pub group_id: Option<Id<UserGroup>>,
    #[serde(default)]
    pub group: Option<UserGroup>,
    #[serde(default)]
    pub is_read: Option<bool>,
    #[serde(default)]
    pub reads: Option<Vec<Id<User>>>,
}

impl_entity!(MessagingMessage);
