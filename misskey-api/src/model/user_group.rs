use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGroup {
    pub id: Id<UserGroup>,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub owner_id: Id<User>,
    pub user_ids: Vec<Id<User>>,
}

impl_entity!(UserGroup);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupInvitation {
    pub id: Id<UserGroupInvitation>,
    pub group: UserGroup,
}

impl_entity!(UserGroupInvitation);
