use crate::model::{drive::DriveFile, id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: Id<User>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: Id<User>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_fetched_at: Option<DateTime<Utc>>,
    pub username: String,
    pub name: Option<String>,
    pub followers_count: u64,
    pub following_count: u64,
    pub notes_count: u64,
    pub avatar_id: Option<Id<DriveFile>>,
    pub banner_id: Option<Id<DriveFile>>,
    pub tags: Vec<String>,
    pub avatar_url: Option<Url>,
    pub banner_url: Option<Url>,
    #[cfg(feature = "12-42-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-42-0")))]
    pub avatar_blurhash: Option<String>,
    #[cfg(feature = "12-42-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-42-0")))]
    pub banner_blurhash: Option<String>,
    pub is_suspended: bool,
    pub is_silenced: bool,
    pub is_locked: bool,
    pub is_bot: bool,
    pub is_cat: bool,
    pub is_admin: bool,
    pub is_moderator: bool,
    #[cfg(feature = "12-63-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-63-0")))]
    pub is_explorable: bool,
    pub emojis: Vec<String>,
    pub host: Option<String>,
    pub inbox: Option<String>,
    pub shared_inbox: Option<String>,
    pub featured: Option<String>,
    pub uri: Option<String>,
    #[cfg(feature = "12-69-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-69-0")))]
    pub followers_uri: Option<String>,
    pub token: Option<String>,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "admin/show-user";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let user = client.user.me().await;

        client.admin.test(Request { user_id: user.id }).await;
    }
}
