#[cfg(feature = "13-10-0")]
use std::fmt::{self, Display};

use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use mime::Mime;
use serde::{Deserialize, Serialize};
#[cfg(feature = "13-10-0")]
use thiserror::Error;
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DriveFileProperties {
    #[cfg(feature = "12-75-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-75-0")))]
    pub width: Option<u64>,
    #[cfg(feature = "12-75-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-75-0")))]
    pub height: Option<u64>,
    #[cfg(feature = "12-98-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-98-0")))]
    pub orientation: Option<u8>,
    #[cfg(feature = "12-75-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-75-0")))]
    pub avg_color: Option<String>,
    #[cfg(not(feature = "12-75-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-75-0"))))]
    #[serde(flatten)]
    pub properties: serde_json::Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DriveFile {
    pub id: Id<DriveFile>,
    pub created_at: DateTime<Utc>,
    pub name: String,
    #[serde(rename = "type", with = "crate::serde::string")]
    pub type_: Mime,
    pub md5: String,
    pub size: u64,
    pub url: Option<Url>,
    pub folder_id: Option<Id<DriveFolder>>,
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    pub comment: Option<String>,
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    pub user_id: Option<Id<User>>,
    pub user: Option<User>,
    #[serde(default)]
    pub folder: Option<DriveFolder>,
    pub is_sensitive: bool,
    pub properties: DriveFileProperties,
}

impl_entity!(DriveFile);

#[cfg(feature = "13-10-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum DriveFileSortKey {
    CreatedAt,
    Name,
    Size,
}

#[cfg(feature = "13-10-0")]
impl Display for DriveFileSortKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DriveFileSortKey::CreatedAt => f.write_str("createdAt"),
            DriveFileSortKey::Name => f.write_str("name"),
            DriveFileSortKey::Size => f.write_str("size"),
        }
    }
}

#[cfg(feature = "13-10-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
#[derive(Debug, Error, Clone)]
#[error("invalid sort key")]
pub struct ParseDriveFileSortKeyError {
    _priv: (),
}

#[cfg(feature = "13-10-0")]
impl std::str::FromStr for DriveFileSortKey {
    type Err = ParseDriveFileSortKeyError;

    fn from_str(s: &str) -> Result<DriveFileSortKey, Self::Err> {
        match s {
            "createdAt" | "CreatedAt" => Ok(DriveFileSortKey::CreatedAt),
            "name" | "Name" => Ok(DriveFileSortKey::Name),
            "size" | "Size" => Ok(DriveFileSortKey::Size),
            _ => Err(ParseDriveFileSortKeyError { _priv: () }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DriveFolder {
    pub id: Id<DriveFolder>,
    pub created_at: DateTime<Utc>,
    pub name: String,
    #[serde(default)]
    pub folders_count: Option<u64>,
    #[serde(default)]
    pub files_count: Option<u64>,
    pub parent_id: Option<Id<DriveFolder>>,
    #[serde(default)]
    pub parent: Option<Box<DriveFolder>>,
}

impl_entity!(DriveFolder);
