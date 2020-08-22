use chrono::{DateTime, Utc};
use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct AnnouncementId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Announcement {
    pub id: AnnouncementId,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub text: String,
    pub title: String,
    pub image_url: Option<Url>,
}
