#[cfg(feature = "12-10-0")]
use crate::model::user_group::UserGroup;
use crate::model::{id::Id, query::Query, user_list::UserList};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Antenna {
    pub id: Id<Antenna>,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub case_sensitive: bool,
    #[cfg(feature = "12-19-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-19-0")))]
    pub exclude_keywords: Query<String>,
    pub keywords: Query<String>,
    pub src: AntennaSource,
    #[cfg(feature = "12-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-10-0")))]
    pub user_group_id: Option<Id<UserGroup>>,
    pub user_list_id: Option<Id<UserList>>,
    pub users: Vec<String>,
    pub notify: bool,
    pub with_file: bool,
    pub with_replies: bool,
}

impl_entity!(Antenna);

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum AntennaSource {
    #[default]
    All,
    Home,
    Users,
    List,
    #[cfg(feature = "12-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-10-0")))]
    Group,
}

#[derive(Debug, Error, Clone)]
#[error("invalid antenna source")]
pub struct ParseAntennaSourceError {
    _priv: (),
}

impl std::str::FromStr for AntennaSource {
    type Err = ParseAntennaSourceError;

    fn from_str(s: &str) -> Result<AntennaSource, Self::Err> {
        match s {
            "all" | "All" => Ok(AntennaSource::All),
            "home" | "Home" => Ok(AntennaSource::Home),
            "users" | "Users" => Ok(AntennaSource::Users),
            "list" | "List" => Ok(AntennaSource::List),
            #[cfg(feature = "12-10-0")]
            "group" | "Group" => Ok(AntennaSource::Group),
            _ => Err(ParseAntennaSourceError { _priv: () }),
        }
    }
}
