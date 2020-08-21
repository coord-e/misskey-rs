use crate::model::{user::UserId, user_group::UserGroupId, user_list::UserListId};

use chrono::{DateTime, Utc};
use derive_more::{Display, Error, FromStr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct AntennaId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Antenna {
    pub id: AntennaId,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub case_sensitive: bool,
    /// not available in <12.19.0
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_keywords: Option<Vec<Vec<String>>>,
    pub keywords: Vec<Vec<String>>,
    pub expression: Option<String>,
    pub src: AntennaSource,
    /// not available in <12.10.0
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_id: Option<UserGroupId>,
    pub user_list_id: Option<UserListId>,
    pub users: Vec<UserId>,
    pub notify: bool,
    pub with_file: bool,
    pub with_replies: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum AntennaSource {
    All,
    Home,
    Users,
    List,
    /// not available in <12.10.0
    Group,
}

#[derive(Debug, Display, Error, Clone)]
#[display(fmt = "invalid antenna source")]
pub struct ParseAntennaSourceError;

impl std::str::FromStr for AntennaSource {
    type Err = ParseAntennaSourceError;

    fn from_str(s: &str) -> Result<AntennaSource, Self::Err> {
        match s {
            "all" | "All" => Ok(AntennaSource::All),
            "home" | "Home" => Ok(AntennaSource::Home),
            "users" | "Users" => Ok(AntennaSource::Users),
            "list" | "List" => Ok(AntennaSource::List),
            "group" | "Group" => Ok(AntennaSource::Group),
            _ => Err(ParseAntennaSourceError),
        }
    }
}
