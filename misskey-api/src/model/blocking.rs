use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Blocking {
    pub id: Id<Blocking>,
    pub created_at: DateTime<Utc>,
    pub blockee_id: Id<User>,
    pub blockee: User,
}

impl_entity!(Blocking);
