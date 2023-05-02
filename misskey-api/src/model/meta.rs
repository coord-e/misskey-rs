#[cfg(feature = "12-112-0")]
use std::fmt::{self, Display};

#[cfg(feature = "12-81-0")]
use crate::model::ad::{Ad, Place};
#[cfg(feature = "12-62-0")]
use crate::model::clip::Clip;
#[cfg(not(feature = "13-0-0"))]
use crate::model::emoji::Emoji;
#[cfg(feature = "13-0-0")]
use crate::model::role::PoliciesSimple;
use crate::model::{id::Id, user::User};

use serde::{Deserialize, Serialize};
#[cfg(feature = "12-112-0")]
use thiserror::Error;
use url::Url;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(default)]
    pub features: Option<FeaturesMeta>,
    #[cfg(not(feature = "12-109-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-109-0"))))]
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
    pub disable_registration: bool,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub disable_local_timeline: bool,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub disable_global_timeline: bool,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub drive_capacity_per_local_user_mb: u64,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
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
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    pub enable_turnstile: bool,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    pub turnstile_site_key: Option<String>,
    #[serde(rename = "swPublickey")]
    pub sw_public_key: Option<String>,
    #[cfg(feature = "12-105-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-105-0")))]
    pub theme_color: Option<String>,
    pub mascot_image_url: Option<String>,
    pub banner_url: Option<String>,
    pub error_image_url: Option<String>,
    pub icon_url: Option<String>,
    #[cfg(feature = "12-60-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-60-0")))]
    pub background_image_url: Option<String>,
    #[cfg(feature = "12-60-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-60-0")))]
    pub logo_image_url: Option<String>,
    pub max_note_text_length: u64,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    #[serde(default)]
    pub emojis: Vec<Emoji>,
    #[cfg(feature = "12-108-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-108-0")))]
    pub default_dark_theme: Option<String>,
    #[cfg(feature = "12-108-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-108-0")))]
    pub default_light_theme: Option<String>,
    #[cfg(feature = "12-81-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-81-0")))]
    #[serde(default)]
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
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    pub policies: PoliciesSimple,
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

#[cfg(not(feature = "12-109-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-109-0"))))]
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

#[cfg(feature = "12-109-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-109-0")))]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminMeta {
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
    pub disable_registration: bool,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub disable_local_timeline: bool,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub disable_global_timeline: bool,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub drive_capacity_per_local_user_mb: u64,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub drive_capacity_per_remote_user_mb: u64,
    pub email_required_for_signup: bool,
    pub enable_hcaptcha: bool,
    pub hcaptcha_site_key: Option<String>,
    pub enable_recaptcha: bool,
    pub recaptcha_site_key: Option<String>,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    pub enable_turnstile: bool,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    pub turnstile_site_key: Option<String>,
    #[serde(rename = "swPublickey")]
    pub sw_public_key: Option<String>,
    pub theme_color: Option<String>,
    pub mascot_image_url: Option<String>,
    pub banner_url: Option<String>,
    pub error_image_url: Option<String>,
    pub icon_url: Option<String>,
    pub background_image_url: Option<String>,
    pub logo_image_url: Option<String>,
    pub max_note_text_length: u64,
    pub default_light_theme: Option<String>,
    pub default_dark_theme: Option<String>,
    pub enable_email: bool,
    pub enable_twitter_integration: bool,
    pub enable_github_integration: bool,
    pub enable_discord_integration: bool,
    pub enable_service_worker: bool,
    pub translator_available: bool,
    pub pinned_pages: Option<Vec<String>>,
    pub pinned_clip_id: Option<Id<Clip>>,
    pub cache_remote_files: Option<bool>,
    pub use_star_for_reaction_fallback: bool,
    pub pinned_users: Vec<String>,
    pub hidden_tags: Vec<String>,
    pub blocked_hosts: Vec<String>,
    pub hcaptcha_secret_key: Option<String>,
    pub recaptcha_secret_key: Option<String>,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    pub turnstile_secret_key: Option<String>,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub sensitive_media_detection: Option<SensitiveMediaDetection>,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub sensitive_media_detection_sensitivity: Option<SensitiveMediaDetectionSensitivity>,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub set_sensitive_flag_automatically: Option<bool>,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub enable_sensitive_media_detection_for_videos: Option<bool>,
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
    pub object_storage_use_proxy: bool,
    pub object_storage_set_public_read: bool,
    pub object_storage_s3_force_path_style: bool,
    pub deepl_auth_key: Option<String>,
    pub deepl_is_pro: bool,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub enable_ip_logging: bool,
    #[cfg(feature = "12-112-3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-3")))]
    pub enable_active_email_validation: bool,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    pub policies: PoliciesSimple,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeaturesMeta {
    pub registration: bool,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub local_time_line: bool,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub global_time_line: bool,
    #[cfg(feature = "12-92-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-92-0")))]
    pub email_required_for_signup: bool,
    pub elasticsearch: bool,
    #[cfg(feature = "12-37-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-37-0")))]
    pub hcaptcha: bool,
    pub recaptcha: bool,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    pub turnstile: bool,
    pub object_storage: bool,
    pub twitter: bool,
    pub github: bool,
    pub discord: bool,
    pub service_worker: bool,
    #[cfg(feature = "12-28-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-28-0")))]
    pub miauth: bool,
}

#[cfg(feature = "12-112-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum SensitiveMediaDetection {
    None,
    All,
    Local,
    Remote,
}

#[cfg(feature = "12-112-0")]
impl Display for SensitiveMediaDetection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SensitiveMediaDetection::None => f.write_str("none"),
            SensitiveMediaDetection::All => f.write_str("all"),
            SensitiveMediaDetection::Local => f.write_str("local"),
            SensitiveMediaDetection::Remote => f.write_str("remote"),
        }
    }
}

#[cfg(feature = "12-112-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
#[derive(Debug, Error, Clone)]
#[error("invalid sensitive media detection")]
pub struct ParseSensitiveMediaDetectionError {
    _priv: (),
}

#[cfg(feature = "12-112-0")]
impl std::str::FromStr for SensitiveMediaDetection {
    type Err = ParseSensitiveMediaDetectionError;

    fn from_str(s: &str) -> Result<SensitiveMediaDetection, Self::Err> {
        match s {
            "none" | "None" => Ok(SensitiveMediaDetection::None),
            "all" | "All" => Ok(SensitiveMediaDetection::All),
            "local" | "Local" => Ok(SensitiveMediaDetection::Local),
            "remote" | "Remote" => Ok(SensitiveMediaDetection::Remote),
            _ => Err(ParseSensitiveMediaDetectionError { _priv: () }),
        }
    }
}

#[cfg(feature = "12-112-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum SensitiveMediaDetectionSensitivity {
    Medium,
    Low,
    High,
    VeryLow,
    VeryHigh,
}

#[cfg(feature = "12-112-0")]
impl Display for SensitiveMediaDetectionSensitivity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SensitiveMediaDetectionSensitivity::Medium => f.write_str("medium"),
            SensitiveMediaDetectionSensitivity::Low => f.write_str("low"),
            SensitiveMediaDetectionSensitivity::High => f.write_str("high"),
            SensitiveMediaDetectionSensitivity::VeryLow => f.write_str("veryLow"),
            SensitiveMediaDetectionSensitivity::VeryHigh => f.write_str("veryHigh"),
        }
    }
}

#[cfg(feature = "12-112-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
#[derive(Debug, Error, Clone)]
#[error("invalid sensitive media detection")]
pub struct ParseSensitiveMediaDetectionSensitivityError {
    _priv: (),
}

#[cfg(feature = "12-112-0")]
impl std::str::FromStr for SensitiveMediaDetectionSensitivity {
    type Err = ParseSensitiveMediaDetectionSensitivityError;

    fn from_str(s: &str) -> Result<SensitiveMediaDetectionSensitivity, Self::Err> {
        match s {
            "medium" | "Medium" => Ok(SensitiveMediaDetectionSensitivity::Medium),
            "low" | "Low" => Ok(SensitiveMediaDetectionSensitivity::Low),
            "high" | "High" => Ok(SensitiveMediaDetectionSensitivity::High),
            "veryLow" | "VeryLow" => Ok(SensitiveMediaDetectionSensitivity::VeryLow),
            "veryHigh" | "VeryHigh" => Ok(SensitiveMediaDetectionSensitivity::VeryHigh),
            _ => Err(ParseSensitiveMediaDetectionSensitivityError { _priv: () }),
        }
    }
}
