use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Retention {
    pub created_at: DateTime<Utc>,
    pub users: u64,
    pub data: HashMap<String, u64>,
}
