use crate::model::user::{User, UserId};

use chrono::{DateTime, Utc};
use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct BlockingId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Blocking {
    pub id: BlockingId,
    pub created_at: DateTime<Utc>,
    pub blockee_id: UserId,
    pub blockee: User,
}

impl_entity!(Blocking, BlockingId);
