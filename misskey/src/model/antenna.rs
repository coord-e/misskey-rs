use crate::model::{user::UserId, user_group::UserGroupId, user_list::UserListId};

use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct AntennaId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Antenna {
    pub id: AntennaId,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub case_sensitive: bool,
    pub exclude_keywords: Vec<Vec<String>>,
    pub keywords: Vec<Vec<String>>,
    pub expression: Option<String>,
    pub src: AntennaSource,
    pub user_group_id: Option<UserGroupId>,
    pub user_list_id: Option<UserListId>,
    pub user_id: UserId,
    pub users: Vec<UserId>,
    pub notify: bool,
    pub with_file: bool,
    pub with_replies: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AntennaSource {
    All,
    Home,
    Users,
    List,
    Group,
}
