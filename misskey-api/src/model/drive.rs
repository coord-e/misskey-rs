use crate::model::id::Id;

use chrono::{DateTime, Utc};
use mime::Mime;
use serde::{Deserialize, Serialize};
use url::Url;

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
    pub is_sensitive: bool,
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
