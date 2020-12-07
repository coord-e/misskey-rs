use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Muting {
    pub id: Id<Muting>,
    pub created_at: DateTime<Utc>,
    pub mutee_id: Id<User>,
    pub mutee: User,
}

impl_entity!(Muting);
