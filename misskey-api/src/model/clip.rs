use crate::model::id::Id;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub id: Id<Clip>,
    pub created_at: DateTime<Utc>,
    pub name: String,
}

impl_entity!(Clip);
