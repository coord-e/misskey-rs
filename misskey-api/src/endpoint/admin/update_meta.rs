#[cfg(all(feature = "12-62-0", not(feature = "13-10-0")))]
use crate::model::clip::Clip;
#[cfg(feature = "12-112-0")]
use crate::model::meta::{SensitiveMediaDetection, SensitiveMediaDetectionSensitivity};
use crate::model::{id::Id, user::User};

use serde::Serialize;
use typed_builder::TypedBuilder;
use url::Url;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[builder(default, setter(strip_option))]
    pub disable_registration: Option<bool>,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    #[builder(default, setter(strip_option))]
    pub disable_local_timeline: Option<bool>,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    #[builder(default, setter(strip_option))]
    pub disable_global_timeline: Option<bool>,
    #[cfg(not(feature = "13-10-3"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-10-3"))))]
    #[builder(default, setter(strip_option))]
    pub use_star_for_reaction_fallback: Option<bool>,
    #[builder(default, setter(strip_option))]
    pub pinned_users: Option<Vec<String>>,
    #[cfg(all(feature = "12-58-0", not(feature = "13-10-0")))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "12-58-0", not(feature = "13-10-0")))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub pinned_pages: Option<Vec<String>>,
    #[cfg(all(feature = "12-62-0", not(feature = "13-10-0")))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "12-62-0", not(feature = "13-10-0")))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub pinned_clip_id: Option<Option<Id<Clip>>>,
    #[builder(default, setter(strip_option))]
    pub hidden_tags: Option<Vec<String>>,
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    #[builder(default, setter(strip_option))]
    pub sensitive_words: Option<Vec<String>>,
    #[builder(default, setter(strip_option))]
    pub blocked_hosts: Option<Vec<String>>,
    #[cfg(feature = "12-105-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-105-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub theme_color: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub mascot_image_url: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub banner_url: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub icon_url: Option<Option<String>>,
    #[cfg(feature = "12-60-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-60-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub background_image_url: Option<Option<String>>,
    #[cfg(feature = "12-60-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-60-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub logo_image_url: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<Option<String>>,
    #[cfg(feature = "12-108-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-108-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub default_light_theme: Option<Option<String>>,
    #[cfg(feature = "12-108-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-108-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub default_dark_theme: Option<Option<String>>,
    #[cfg(not(feature = "12-108-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-108-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub max_note_text_length: Option<u64>,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub local_drive_capacity_mb: Option<u64>,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub remote_drive_capacity_mb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub cache_remote_files: Option<bool>,
    #[cfg(not(feature = "12-108-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-108-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub proxy_remote_files: Option<bool>,
    #[cfg(feature = "12-92-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-92-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email_required_for_signup: Option<bool>,
    #[cfg(feature = "12-37-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-37-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_hcaptcha: Option<bool>,
    #[cfg(feature = "12-37-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-37-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub hcaptcha_site_key: Option<Option<String>>,
    #[cfg(feature = "12-37-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-37-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub hcaptcha_secret_key: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_recaptcha: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub recaptcha_site_key: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub recaptcha_secret_key: Option<Option<String>>,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_turnstile: Option<bool>,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub turnstile_site_key: Option<Option<String>>,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub turnstile_secret_key: Option<Option<String>>,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sensitive_media_detection: Option<SensitiveMediaDetection>,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sensitive_media_detection_sensitivity: Option<SensitiveMediaDetectionSensitivity>,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub set_sensitive_flag_automatically: Option<bool>,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_sensitive_media_detection_for_videos: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub proxy_account_id: Option<Option<Id<User>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub maintainer_name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub maintainer_email: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub langs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub summaly_proxy: Option<Option<Url>>,
    #[cfg(feature = "12-88-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-88-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub deepl_auth_key: Option<Option<String>>,
    #[cfg(feature = "12-89-1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-89-1")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub deepl_is_pro: Option<bool>,
    #[cfg(not(feature = "13-3-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_twitter_integration: Option<bool>,
    #[cfg(not(feature = "13-3-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub twitter_consumer_key: Option<Option<String>>,
    #[cfg(not(feature = "13-3-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub twitter_consumer_secret: Option<Option<String>>,
    #[cfg(not(feature = "13-3-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_github_integration: Option<bool>,
    #[cfg(not(feature = "13-3-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub github_client_id: Option<Option<String>>,
    #[cfg(not(feature = "13-3-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub github_client_secret: Option<Option<String>>,
    #[cfg(not(feature = "13-3-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_discord_integration: Option<bool>,
    #[cfg(not(feature = "13-3-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub discord_client_id: Option<Option<String>>,
    #[cfg(not(feature = "13-3-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub discord_client_secret: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub smtp_secure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub smtp_host: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub smtp_port: Option<Option<u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub smtp_user: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub smtp_pass: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub error_image_url: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_service_worker: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sw_public_key: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sw_private_key: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tos_url: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub repository_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub feedback_url: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub use_object_storage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub object_storage_base_url: Option<Option<Url>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub object_storage_bucket: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub object_storage_prefix: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub object_storage_endpoint: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub object_storage_region: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub object_storage_port: Option<Option<u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub object_storage_access_key: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub object_storage_secret_key: Option<Option<String>>,
    #[serde(
        rename = "objectStorageUseSSL",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(strip_option))]
    pub object_storage_use_ssl: Option<bool>,
    #[cfg(feature = "12-31-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-31-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub object_storage_use_proxy: Option<bool>,
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub object_storage_set_public_read: Option<bool>,
    #[cfg(feature = "12-69-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-69-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub object_storage_s3_force_path_style: Option<bool>,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_ip_logging: Option<bool>,
    #[cfg(feature = "12-112-3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-3")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_active_email_validation: Option<bool>,
    #[cfg(feature = "13-10-3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-3")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_charts_for_remote_user: Option<bool>,
    #[cfg(feature = "13-10-3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-3")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_charts_for_federated_instances: Option<bool>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/update-meta";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    // this will fail with requests without any updates
    #[tokio::test]
    async fn request_with_name() {
        let client = TestClient::new();
        client
            .admin
            .test(
                Request::builder()
                    .name(Some("instance name".to_string()))
                    .build(),
            )
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        #[cfg(feature = "12-112-0")]
        use crate::model::meta::{SensitiveMediaDetection, SensitiveMediaDetectionSensitivity};

        let client = TestClient::new();
        let image_url = client.avatar_url().await;

        client
            .admin
            .test(Request {
                disable_registration: Some(false),
                #[cfg(not(feature = "13-0-0"))]
                disable_local_timeline: Some(false),
                #[cfg(not(feature = "13-0-0"))]
                disable_global_timeline: Some(false),
                #[cfg(not(feature = "13-10-3"))]
                use_star_for_reaction_fallback: Some(false),
                #[cfg(all(feature = "12-58-0", not(feature = "13-10-0")))]
                pinned_pages: Some(vec!["/announcements".to_string()]),
                #[cfg(all(feature = "12-62-0", not(feature = "13-10-0")))]
                pinned_clip_id: Some(None),
                pinned_users: Some(vec!["@admin".to_string(), "@testuser".to_string()]),
                hidden_tags: Some(vec!["not_good".to_string()]),
                blocked_hosts: Some(vec!["not.good.host".to_string()]),
                #[cfg(feature = "13-10-0")]
                sensitive_words: Some(vec!["sensitive".to_string()]),
                #[cfg(feature = "12-105-0")]
                theme_color: Some(Some("#31748f".to_string())),
                mascot_image_url: Some(Some(image_url.to_string())),
                banner_url: Some(Some(image_url.to_string())),
                icon_url: Some(Some(image_url.to_string())),
                #[cfg(feature = "12-60-0")]
                background_image_url: Some(Some(image_url.to_string())),
                #[cfg(feature = "12-60-0")]
                logo_image_url: Some(Some(image_url.to_string())),
                name: None,
                description: Some(Some("description!".to_string())),
                #[cfg(feature = "12-108-0")]
                default_light_theme: Some(Some("{}".to_string())),
                #[cfg(feature = "12-108-0")]
                default_dark_theme: Some(Some("{}".to_string())),
                #[cfg(not(feature = "12-108-0"))]
                max_note_text_length: Some(1000),
                #[cfg(not(feature = "13-0-0"))]
                local_drive_capacity_mb: Some(1000),
                #[cfg(not(feature = "13-0-0"))]
                remote_drive_capacity_mb: Some(1000),
                cache_remote_files: Some(true),
                #[cfg(not(feature = "12-108-0"))]
                proxy_remote_files: Some(true),
                #[cfg(feature = "12-92-0")]
                email_required_for_signup: Some(true),
                #[cfg(feature = "12-37-0")]
                enable_hcaptcha: Some(false),
                #[cfg(feature = "12-37-0")]
                hcaptcha_site_key: Some(None),
                #[cfg(feature = "12-37-0")]
                hcaptcha_secret_key: Some(None),
                enable_recaptcha: Some(false),
                recaptcha_site_key: Some(None),
                recaptcha_secret_key: Some(None),
                #[cfg(feature = "13-0-0")]
                enable_turnstile: Some(false),
                #[cfg(feature = "13-0-0")]
                turnstile_site_key: Some(None),
                #[cfg(feature = "13-0-0")]
                turnstile_secret_key: Some(None),
                #[cfg(feature = "12-112-0")]
                sensitive_media_detection: Some(SensitiveMediaDetection::None),
                #[cfg(feature = "12-112-0")]
                sensitive_media_detection_sensitivity: Some(
                    SensitiveMediaDetectionSensitivity::Medium,
                ),
                #[cfg(feature = "12-112-0")]
                set_sensitive_flag_automatically: Some(false),
                #[cfg(feature = "12-112-0")]
                enable_sensitive_media_detection_for_videos: Some(false),
                proxy_account_id: Some(None),
                maintainer_name: Some(Some("coord_e".to_string())),
                maintainer_email: Some(Some("me@coord-e.com".to_string())),
                langs: Some(vec!["ja_JP".to_string()]),
                summaly_proxy: Some(None),
                #[cfg(feature = "12-88-0")]
                deepl_auth_key: Some(None),
                #[cfg(feature = "12-89-1")]
                deepl_is_pro: Some(false),
                #[cfg(not(feature = "13-3-0"))]
                enable_twitter_integration: Some(false),
                #[cfg(not(feature = "13-3-0"))]
                twitter_consumer_key: Some(None),
                #[cfg(not(feature = "13-3-0"))]
                twitter_consumer_secret: Some(None),
                #[cfg(not(feature = "13-3-0"))]
                enable_github_integration: Some(false),
                #[cfg(not(feature = "13-3-0"))]
                github_client_id: Some(None),
                #[cfg(not(feature = "13-3-0"))]
                github_client_secret: Some(None),
                #[cfg(not(feature = "13-3-0"))]
                enable_discord_integration: Some(false),
                #[cfg(not(feature = "13-3-0"))]
                discord_client_id: Some(None),
                #[cfg(not(feature = "13-3-0"))]
                discord_client_secret: Some(None),
                enable_email: Some(false),
                email: Some(None),
                smtp_secure: Some(true),
                smtp_host: Some(None),
                smtp_port: Some(None),
                smtp_user: Some(None),
                smtp_pass: Some(None),
                error_image_url: Some(Some(image_url.to_string())),
                enable_service_worker: Some(false),
                sw_public_key: Some(None),
                sw_private_key: Some(None),
                tos_url: Some(None),
                repository_url: Some(image_url.clone()),
                feedback_url: Some(Some(image_url.to_string())),
                use_object_storage: Some(false),
                object_storage_base_url: Some(None),
                object_storage_bucket: Some(None),
                object_storage_prefix: Some(None),
                object_storage_endpoint: Some(None),
                object_storage_region: Some(None),
                object_storage_port: Some(None),
                object_storage_access_key: Some(None),
                object_storage_secret_key: Some(None),
                object_storage_use_ssl: Some(false),
                #[cfg(feature = "12-31-0")]
                object_storage_use_proxy: Some(false),
                #[cfg(feature = "12-47-0")]
                object_storage_set_public_read: Some(false),
                #[cfg(feature = "12-69-0")]
                object_storage_s3_force_path_style: Some(false),
                #[cfg(feature = "12-112-0")]
                enable_ip_logging: Some(false),
                #[cfg(feature = "12-112-3")]
                enable_active_email_validation: Some(false),
                #[cfg(feature = "13-10-3")]
                enable_charts_for_remote_user: Some(false),
                #[cfg(feature = "13-10-3")]
                enable_charts_for_federated_instances: Some(false),
            })
            .await;
    }
}
