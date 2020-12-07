use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserList {
    pub id: Id<UserList>,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub user_ids: Vec<Id<User>>,
}

impl_entity!(UserList);
