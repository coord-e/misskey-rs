use crate::model::user::{User, UserId};

use chrono::{DateTime, Utc};
use derive_more::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug)]
#[serde(transparent)]
pub struct MutingId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Muting {
    pub id: MutingId,
    pub created_at: DateTime<Utc>,
    pub mutee_id: UserId,
    pub mutee: User,
}