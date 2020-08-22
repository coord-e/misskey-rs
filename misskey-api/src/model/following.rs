use crate::model::user::{User, UserId};

use chrono::{DateTime, Utc};
use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct FollowingId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowingWithFollowee {
    pub id: FollowingId,
    pub created_at: DateTime<Utc>,
    pub followee_id: UserId,
    pub followee: User,
    pub follower_id: UserId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowingWithFollower {
    pub id: FollowingId,
    pub created_at: DateTime<Utc>,
    pub followee_id: UserId,
    pub follower_id: UserId,
    pub follower: User,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct FollowRequestId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowRequest {
    pub id: FollowRequestId,
    pub followee: User,
    pub follower: User,
}
