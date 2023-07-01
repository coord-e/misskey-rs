use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use mime::Mime;
use serde::{Deserialize, Serialize};
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
