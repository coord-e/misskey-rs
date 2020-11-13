use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbuseUserReport {
    pub id: Id<AbuseUserReport>,
    pub created_at: DateTime<Utc>,
    pub comment: String,
    pub reporter_id: Id<User>,
    pub reporter: User,
    pub user_id: Id<User>,
    pub user: User,
}

impl_entity!(AbuseUserReport);
