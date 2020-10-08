use chrono::{DateTime, Utc};
use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct DriveFileId(pub String);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct DriveFolderId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DriveFile {
    pub id: DriveFileId,
    pub created_at: DateTime<Utc>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub md5: String,
    pub size: u64,
    pub url: Option<Url>,
    pub folder_id: Option<DriveFolderId>,
    pub is_sensitive: bool,
}

impl_entity!(DriveFile, DriveFileId);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DriveFolder {
    pub id: DriveFolderId,
    pub created_at: DateTime<Utc>,
    pub name: String,
    #[serde(default)]
    pub folders_count: Option<u64>,
    #[serde(default)]
    pub files_count: Option<u64>,
    pub parent_id: Option<DriveFolderId>,
    #[serde(default)]
    pub parent: Option<Box<DriveFolder>>,
}

impl_entity!(DriveFolder, DriveFolderId);
