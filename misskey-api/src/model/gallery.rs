use crate::model::{drive::DriveFile, id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GalleryPost {
    pub id: Id<GalleryPost>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Id<User>,
    pub user: User,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    pub file_ids: Vec<Id<DriveFile>>,
    pub files: Vec<DriveFile>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    pub is_sensitive: bool,
    pub liked_count: u64,
    #[serde(default)]
    pub is_liked: Option<bool>,
}

impl_entity!(GalleryPost);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GalleryLike {
    pub id: Id<GalleryLike>,
    pub post: GalleryPost,
}

impl_entity!(GalleryLike);
