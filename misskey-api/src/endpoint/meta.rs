use crate::model::{emoji::Emoji, user::UserId};

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub maintainer_name: Option<String>,
    pub maintainer_email: Option<String>,
    pub version: String,
    pub name: Option<String>,
    pub uri: String,
    pub description: Option<String>,
    pub langs: Vec<String>,
    pub tos_url: Option<String>,
    pub repository_url: Url,
    pub feedback_url: Option<String>,
    pub secure: bool,
    pub disable_registration: bool,
    pub disable_local_timeline: bool,
    pub disable_global_timeline: bool,
    pub drive_capacity_per_local_user_mb: u64,
    pub drive_capacity_per_remote_user_mb: u64,
    pub cache_remote_files: bool,
    pub proxy_remote_files: bool,
    #[cfg(feature = "12-37-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-37-0")))]
    pub enable_hcaptcha: bool,
    #[cfg(feature = "12-37-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-37-0")))]
    pub hcaptcha_site_key: Option<String>,
    pub enable_recaptcha: bool,
    pub recaptcha_site_key: Option<String>,
    #[serde(rename = "swPublickey")]
    pub sw_public_key: Option<String>,
    pub mascot_image_url: Option<String>,
    pub bannar_url: Option<String>,
    pub error_image_url: Option<String>,
    pub icon_url: Option<String>,
    pub max_note_text_length: u64,
    pub emojis: Vec<Emoji>,
    pub require_setup: bool,
    pub enable_email: bool,
    pub enable_twitter_integration: bool,
    pub enable_github_integration: bool,
    pub enable_discord_integration: bool,
    pub enable_service_worker: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminMeta {
    pub use_star_for_reaction_fallback: bool,
    pub pinned_users: Vec<String>,
    pub hidden_tags: Vec<String>,
    pub blocked_hosts: Vec<String>,
    #[cfg(feature = "12-37-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-37-0")))]
    pub hcaptcha_secret_key: Option<String>,
    pub recaptcha_secret_key: Option<String>,
    pub proxy_account_id: Option<UserId>,
    pub twitter_consumer_key: Option<String>,
    pub twitter_consumer_secret: Option<String>,
    pub github_client_id: Option<String>,
    pub github_client_secret: Option<String>,
    pub discord_client_id: Option<String>,
    pub discord_client_secret: Option<String>,
    pub summaly_proxy: Option<Url>,
    pub email: Option<String>,
    pub smtp_secure: bool,
    pub smtp_host: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_user: Option<String>,
    pub smtp_pass: Option<String>,
    pub sw_private_key: Option<String>,
    pub use_object_storage: bool,
    pub object_storage_base_url: Option<Url>,
    pub object_storage_bucket: Option<String>,
    pub object_storage_prefix: Option<String>,
    pub object_storage_endpoint: Option<String>,
    pub object_storage_region: Option<String>,
    pub object_storage_port: Option<u16>,
    pub object_storage_access_key: Option<String>,
    pub object_storage_secret_key: Option<String>,
    #[serde(rename = "objectStorageUseSSL")]
    pub object_storage_use_ssl: bool,
    #[cfg(feature = "12-31-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-31-0")))]
    pub object_storage_use_proxy: bool,
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    pub object_storage_set_public_read: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeaturesMeta {
    pub registration: bool,
    pub local_time_line: bool,
    pub global_time_line: bool,
    pub elasticsearch: bool,
    #[cfg(feature = "12-37-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-37-0")))]
    pub hcaptcha: bool,
    pub recaptcha: bool,
    pub object_storage: bool,
    pub twitter: bool,
    pub github: bool,
    pub discord: bool,
    pub service_worker: bool,
    #[cfg(feature = "12-28-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-28-0")))]
    pub miauth: bool,
}

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(default)]
    pub features: Option<FeaturesMeta>,
    #[serde(default, flatten)]
    pub admin: Option<AdminMeta>,
    #[serde(flatten)]
    pub meta: Meta,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "meta";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_by_admin() {
        let client = TestClient::new();
        client.admin.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_details() {
        let client = TestClient::new();
        client.test(Request { detail: Some(true) }).await;
    }

    #[tokio::test]
    async fn request_without_details() {
        let client = TestClient::new();
        client
            .test(Request {
                detail: Some(false),
            })
            .await;
    }
}
