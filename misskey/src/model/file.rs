use chrono::{DateTime, Utc};
use derivative::Derivative;
use derive_more::FromStr;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct FileId(pub String);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct FolderId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DriveFile {
    pub id: FileId,
    pub created_at: DateTime<Utc>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub md5: String,
    pub size: u64,
    pub url: Option<Url>,
    pub folder_id: Option<FolderId>,
    pub is_sensitive: bool,
}
