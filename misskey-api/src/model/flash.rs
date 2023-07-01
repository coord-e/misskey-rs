use crate::model::id::Id;
use crate::model::user::User;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Flash {
    pub id: Id<Flash>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Id<User>,
    pub user: User,
    pub title: String,
    pub summary: String,
    pub script: String,
    pub liked_count: u64,
    pub is_liked: Option<bool>,
}

impl_entity!(Flash);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FlashLike {
    pub id: Id<FlashLike>,
    pub flash: Flash,
}

impl_entity!(FlashLike);
