use crate::model::user::UserId;

use chrono::{DateTime, Utc};
use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct UserGroupId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGroup {
    pub id: UserGroupId,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub owner_id: UserId,
    pub user_ids: Vec<UserId>,
}

impl_entity!(UserGroup, UserGroupId);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct UserGroupInvitationId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupInvitation {
    pub id: UserGroupInvitationId,
    pub group: UserGroup,
}

impl_entity!(UserGroupInvitation, UserGroupInvitationId);
