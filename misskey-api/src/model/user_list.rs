use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserList {
    pub id: Id<UserList>,
    pub created_at: DateTime<Utc>,
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<Id<User>>>,
}

impl_entity!(UserList);
