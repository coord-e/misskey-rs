#[cfg(feature = "12-81-0")]
use crate::model::ad::{Ad, Place};
#[cfg(feature = "12-62-0")]
use crate::model::clip::Clip;
use crate::model::{emoji::Emoji, id::Id, user::User};

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(default)]
    pub features: Option<FeaturesMeta>,
    #[serde(default, flatten)]
    pub admin: Option<AdminMeta>,
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
    #[cfg(not(feature = "12-108-0"))]
    pub secure: bool,
    #[cfg(feature = "12-108-0")]
    pub default_dark_theme: Option<String>,
    #[cfg(feature = "12-108-0")]
    pub default_light_theme: Option<String>,
    pub disable_registration: bool,
    pub disable_local_timeline: bool,
    pub disable_global_timeline: bool,
    pub drive_capacity_per_local_user_mb: u64,
    pub drive_capacity_per_remote_user_mb: u64,
    /// This field is [`bool`] (i.e. not [`Option`]) on <span class="module-item stab portability" style="display: inline-block; font-size: 80%;"><strong>non-<code style="background-color: transparent;">feature="12-58-0"</code></strong></span>.
    #[cfg(feature = "12-58-0")]
    pub cache_remote_files: Option<bool>,
    #[cfg(not(feature = "12-58-0"))]
    pub cache_remote_files: bool,
    /// This field is [`bool`] (i.e. not [`Option`]) on <span class="module-item stab portability" style="display: inline-block; font-size: 80%;"><strong>non-<code style="background-color: transparent;">feature="12-58-0"</code></strong></span>.
    #[cfg(all(feature = "12-58-0", not(feature = "12-108-0")))]
    pub proxy_remote_files: Option<bool>,
    #[cfg(not(feature = "12-58-0"))]
    pub proxy_remote_files: bool,
    #[cfg(feature = "12-92-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-92-0")))]
    pub email_required_for_signup: bool,
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
    #[cfg(feature = "12-105-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-105-0")))]
    pub theme_color: Option<String>,
    pub mascot_image_url: Option<String>,
    pub bannar_url: Option<String>,
    pub error_image_url: Option<String>,
    pub icon_url: Option<String>,
    pub max_note_text_length: u64,
    pub emojis: Vec<Emoji>,
    #[cfg(feature = "12-81-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-81-0")))]
    pub ads: Vec<MetaAd>,
    /// This field is [`bool`] (i.e. not [`Option`]) on <span class="module-item stab portability" style="display: inline-block; font-size: 80%;"><strong>non-<code style="background-color: transparent;">feature="12-58-0"</code></strong></span>.
    #[cfg(feature = "12-58-0")]
    pub require_setup: Option<bool>,
    #[cfg(not(feature = "12-58-0"))]
    pub require_setup: bool,
    pub enable_email: bool,
    pub enable_twitter_integration: bool,
    pub enable_github_integration: bool,
    pub enable_discord_integration: bool,
    pub enable_service_worker: bool,
    #[cfg(feature = "12-88-0")]
    pub translator_available: bool,
    /// This field is [`Option<String>`][`Option`] on <span class="module-item stab portability" style="display: inline-block; font-size: 80%;"><strong>non-<code style="background-color: transparent;">feature="12-58-0"</code></strong></span>.
    #[cfg(feature = "12-58-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    #[serde(default)]
    pub proxy_account_name: Option<Option<String>>,
    #[cfg(all(feature = "12-48-0", not(feature = "12-58-0")))]
    pub proxy_account_name: Option<String>,
    #[cfg(all(
        feature = "12-58-0",
        any(not(feature = "12-62-0"), feature = "12-62-2")
    ))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(
            feature = "12-58-0",
            any(not(feature = "12-62-0"), feature = "12-62-2")
        )))
    )]
    pub pinned_pages: Option<Vec<String>>,
    #[cfg(feature = "12-62-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-62-0")))]
    pub pinned_clip_id: Option<Id<Clip>>,
}

#[cfg(feature = "12-81-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-81-0")))]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetaAd {
    pub id: Id<Ad>,
    pub url: String,
    pub place: Place,
    pub ratio: u64,
    pub image_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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
    pub proxy_account_id: Option<Id<User>>,
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
    #[cfg(feature = "12-69-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-69-0")))]
    pub object_storage_s3_force_path_style: bool,
    #[cfg(feature = "12-89-1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-89-1")))]
    pub deepl_auth_key: Option<String>,
    #[cfg(feature = "12-89-1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-89-1")))]
    pub deepl_is_pro: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeaturesMeta {
    pub registration: bool,
    pub local_time_line: bool,
    pub global_time_line: bool,
    #[cfg(feature = "12-92-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-92-0")))]
    pub email_required_for_signup: bool,
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
