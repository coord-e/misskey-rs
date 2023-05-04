#[cfg(all(feature = "12-111-0", not(feature = "13-3-0")))]
use std::collections::HashMap;

#[cfg(not(feature = "12-111-0"))]
use crate::model::drive::DriveFile;
#[cfg(feature = "13-0-0")]
use crate::model::role::{PoliciesSimple, Role};
#[cfg(all(feature = "12-111-0", not(feature = "13-3-0")))]
use crate::model::user::IntegrationValue;
use crate::model::{id::Id, user::User};
#[cfg(feature = "12-111-0")]
use crate::model::{notification::NotificationType, signin::Signin};

#[cfg(any(not(feature = "12-111-0"), feature = "12-112-0"))]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "12-111-0"))]
use url::Url;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: Id<User>,
}

#[cfg(not(feature = "12-111-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-111-0"))))]
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

#[cfg(feature = "12-111-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-111-0")))]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub email_verified: Option<bool>,
    #[serde(default)]
    pub auto_accept_followed: Option<bool>,
    #[serde(default)]
    pub no_crawle: Option<bool>,
    #[serde(default)]
    pub always_mark_nsfw: Option<bool>,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    #[serde(default)]
    pub auto_sensitive: Option<bool>,
    #[serde(default)]
    pub careful_bot: Option<bool>,
    #[serde(default)]
    pub inject_featured_note: Option<bool>,
    #[serde(default)]
    pub receive_announcement_email: Option<bool>,
    #[cfg(not(feature = "13-3-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
    #[serde(default)]
    pub integrations: Option<HashMap<String, IntegrationValue>>,
    #[serde(default)]
    pub muted_words: Option<Vec<Vec<String>>>,
    #[serde(default)]
    pub muted_instances: Option<Vec<String>>,
    #[serde(default)]
    pub muting_notification_types: Option<Vec<NotificationType>>,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub is_moderator: bool,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    #[serde(default)]
    pub is_moderator: Option<bool>,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub is_silenced: bool,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    #[serde(default)]
    pub is_silenced: Option<bool>,
    pub is_suspended: bool,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    #[serde(default)]
    pub last_active_date: Option<DateTime<Utc>>,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    #[serde(default)]
    pub moderation_note: Option<String>,
    #[serde(default)]
    pub signins: Option<Vec<Signin>>,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    #[serde(default)]
    pub policies: Option<PoliciesSimple>,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    #[serde(default)]
    pub roles: Option<Vec<Role>>,
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

    #[cfg(not(feature = "13-0-0"))]
    #[tokio::test]
    async fn request_moderator() {
        let client = TestClient::new();
        let user_id = client.user.me().await.id;
        client
            .admin
            .test(crate::endpoint::admin::moderators::add::Request { user_id })
            .await;

        client.user.test(Request { user_id }).await;
    }

    #[cfg(feature = "13-0-0")]
    #[tokio::test]
    async fn request_moderator() {
        let client = TestClient::new();
        let user_id = client.user.me().await.id;
        let role = client
            .admin
            .test(
                crate::endpoint::admin::roles::create::Request::builder()
                    .is_moderator(true)
                    .build(),
            )
            .await;
        client
            .admin
            .test(
                crate::endpoint::admin::roles::assign::Request::builder()
                    .role_id(role.id)
                    .user_id(user_id)
                    .build(),
            )
            .await;

        client.user.test(Request { user_id }).await;
    }
}
