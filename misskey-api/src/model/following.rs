use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Following {
    pub id: Id<Following>,
    pub created_at: DateTime<Utc>,
    pub followee_id: Id<User>,
    pub follower_id: Id<User>,
}

impl_entity!(Following);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowRequest {
    pub id: Id<FollowRequest>,
    pub followee: User,
    pub follower: User,
}

impl_entity!(FollowRequest);
