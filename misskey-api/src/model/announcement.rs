use crate::model::id::Id;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Announcement {
    pub id: Id<Announcement>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub text: String,
    pub title: String,
    pub image_url: Option<Url>,
}

impl_entity!(Announcement);
