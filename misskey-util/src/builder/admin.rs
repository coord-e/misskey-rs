use crate::Error;

#[cfg(feature = "12-80-0")]
use chrono::{DateTime, Utc};
#[cfg(feature = "12-80-0")]
use misskey_api::model::ad::{Ad, Place, Priority};
#[cfg(feature = "12-62-0")]
use misskey_api::model::clip::Clip;
#[cfg(feature = "12-9-0")]
use misskey_api::model::emoji::Emoji;
#[cfg(not(feature = "12-93-0"))]
use misskey_api::model::log::{Log, LogLevel};
#[cfg(feature = "12-112-0")]
use misskey_api::model::meta::{SensitiveMediaDetection, SensitiveMediaDetectionSensitivity};
#[cfg(feature = "13-0-0")]
use misskey_api::model::role::{
    self, Policies, PoliciesSimple, PolicyValue, Role, RoleCondFormulaValue, Target,
};
use misskey_api::model::{announcement::Announcement, user::User};
use misskey_api::{endpoint, EntityRef};
use misskey_core::Client;
use url::Url;

#[cfg(not(feature = "12-93-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-93-0"))))]
/// Builder for the [`server_logs`][`crate::ClientExt::server_logs`] method.
pub struct ServerLogListBuilder<C> {
    client: C,
    request: endpoint::admin::logs::Request,
}

#[cfg(not(feature = "12-93-0"))]
impl<C> ServerLogListBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::admin::logs::Request::default();
        ServerLogListBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::admin::logs::Request {
        &self.request
    }

    /// Limits the number of logs to be listed.
    pub fn take(&mut self, length: u8) -> &mut Self {
        self.request.limit.replace(length);
        self
    }

    /// Limits the level of logs to be listed to the specified one.
    pub fn level(&mut self, level: LogLevel) -> &mut Self {
        self.request.level.replace(level);
        self
    }

    /// Limits the listed logs to errors.
    ///
    /// This is equivalent to `.level(LogLevel::Error)`.
    pub fn error(&mut self) -> &mut Self {
        self.level(LogLevel::Error)
    }

    /// Limits the listed logs to warnings.
    ///
    /// This is equivalent to `.level(LogLevel::Warning)`.
    pub fn warning(&mut self) -> &mut Self {
        self.level(LogLevel::Warning)
    }

    /// Limits the listed logs to informations.
    ///
    /// This is equivalent to `.level(LogLevel::Info)`.
    pub fn info(&mut self) -> &mut Self {
        self.level(LogLevel::Info)
    }

    /// Limits the listed logs to successes.
    ///
    /// This is equivalent to `.level(LogLevel::Success)`.
    pub fn success(&mut self) -> &mut Self {
        self.level(LogLevel::Success)
    }

    /// Limits the listed logs to debug logs.
    ///
    /// This is equivalent to `.level(LogLevel::Debug)`.
    pub fn debug(&mut self) -> &mut Self {
        self.level(LogLevel::Debug)
    }

    /// Adds a domain name to be included in the listed logs.
    ///
    /// You can add more domains to be included with subsequent calls to this method.
    pub fn with_domain(&mut self, domain: impl AsRef<str>) -> &mut Self {
        if let Some(domains) = self.request.domain.as_mut() {
            domains.push(' ');
            domains.push_str(domain.as_ref());
        } else {
            self.request.domain.replace(domain.as_ref().to_owned());
        }
        self
    }

    /// Adds a domain name to be excluded from the listed logs.
    ///
    /// You can add more domains to be excluded with subsequent calls to this method.
    pub fn without_domain(&mut self, domain: impl AsRef<str>) -> &mut Self {
        if let Some(domains) = self.request.domain.as_mut() {
            domains.push_str(" -");
            domains.push_str(domain.as_ref());
        } else {
            self.request.domain.replace(format!("-{}", domain.as_ref()));
        }
        self
    }
}

#[cfg(not(feature = "12-93-0"))]
impl<C: Client> ServerLogListBuilder<C> {
    /// Lists the logs.
    pub async fn list(&self) -> Result<Vec<Log>, Error<C::Error>> {
        let logs = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(logs)
    }
}

/// Builder for the [`update_meta`][`crate::ClientExt::update_meta`] method.
pub struct MetaUpdateBuilder<C> {
    client: C,
    request: endpoint::admin::update_meta::Request,
}

impl<C> MetaUpdateBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::admin::update_meta::Request::default();
        MetaUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::admin::update_meta::Request {
        &self.request
    }

    update_builder_bool_field! {
        /// Sets whether the instance has registration enabled.
        pub disable_registration;
        /// Sets whether the instance has local timeline enabled.
        #[cfg(not(feature = "13-0-0"))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
        pub disable_local_timeline;
        /// Sets whether the instance has global timeline enabled.
        #[cfg(not(feature = "13-0-0"))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
        pub disable_global_timeline;
        /// Sets whether the instance uses ★ as fallback if the reaction emoji is unknown.
        pub use_star_for_reaction_fallback;
    }

    update_builder_string_collection_field! {
        /// Sets the pinned users of the instance.
        pub pinned_users;
        /// Sets the hashtags that the instance will ignore for statistics, etc.
        pub hidden_tags;
        /// Sets the hosts to be blocked by the instance.
        pub blocked_hosts;
        /// Sets the pinned pages of the instance.
        #[cfg(feature = "12-58-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-58-0")))]
        pub pinned_pages;
    }

    update_builder_option_field! {
        #[doc_name = "pinned clip"]
        #[cfg(feature = "12-62-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-62-0")))]
        pub pinned_clip : impl EntityRef<Clip> { pinned_clip_id =  pinned_clip.entity_ref() };
    }

    update_builder_string_option_field! {
        #[doc_name = "theme color for the instance"]
        #[cfg(feature = "12-105-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-105-0")))]
        pub theme_color;
        #[doc_name = "URL of the mascot image for the instance"]
        pub mascot_image_url;
        #[doc_name = "URL of the banner image for the instance"]
        pub banner_url;
        #[doc_name = "URL of the error image for the instance"]
        pub error_image_url;
        #[doc_name = "URL of the icon for the instance"]
        pub icon_url;
        #[doc_name = "name of the instance"]
        pub name;
        #[doc_name = "description of the instance"]
        pub description;
        #[doc_name = "URL of the background image for the instance"]
        #[cfg(feature = "12-60-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-60-0")))]
        pub background_image_url;
        #[doc_name = "URL of the logo image for the instance"]
        #[cfg(feature = "12-60-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-60-0")))]
        pub logo_image_url;
    }

    /// Sets the maximum number of characters for posts in the instance.
    #[cfg(not(feature = "12-108-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-108-0"))))]
    pub fn max_note_text_length(&mut self, max_note_text_length: u64) -> &mut Self {
        self.request
            .max_note_text_length
            .replace(max_note_text_length);
        self
    }

    /// Sets the drive capacity per local user in megabytes.
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub fn local_drive_capacity(&mut self, mb: u64) -> &mut Self {
        self.request.local_drive_capacity_mb.replace(mb);
        self
    }

    /// Sets the drive capacity per remote user in megabytes.
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub fn remote_drive_capacity(&mut self, mb: u64) -> &mut Self {
        self.request.remote_drive_capacity_mb.replace(mb);
        self
    }

    update_builder_bool_field! {
        /// Sets whether or not the instance would cache remote files.
        pub cache_remote_files;
        /// Sets whether or not the instance would proxy remote files that are not available
        /// locally.
        #[cfg(not(feature = "12-108-0"))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "12-108-0"))))]
        pub proxy_remote_files;
    }

    update_builder_bool_field! {
        /// Sets whether or not the instance requires email for signup.
        #[cfg(feature = "12-92-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-92-0")))]
        pub email_required_for_signup;

        /// Sets whether or not the instance enables hCaptcha.
        #[cfg(feature = "12-37-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-37-0")))]
        pub enable_hcaptcha;

        /// Sets whether or not the instance enables reCAPTCHA.
        pub enable_recaptcha;
    }

    update_builder_string_option_field! {
        #[doc_name = "hCaptcha site key"]
        #[cfg(feature = "12-37-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-37-0")))]
        pub hcaptcha_site_key;

        #[doc_name = "hCaptcha secret key"]
        #[cfg(feature = "12-37-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-37-0")))]
        pub hcaptcha_secret_key;

        #[doc_name = "reCAPTCHA site key"]
        pub recaptcha_site_key;
        #[doc_name = "reCAPTCHA secret key"]
        pub recaptcha_secret_key;
    }

    /// Sets sensitive media detection target.
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub fn detect_sensitive_media(&mut self, detection: SensitiveMediaDetection) -> &mut Self {
        self.request.sensitive_media_detection.replace(detection);
        self
    }

    /// Sets sensitive media detection target to none.
    ///
    /// This is equivalent to `.detect_sensitive_media(SensitiveMediaDetection::None)`.
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub fn disable_sensitive_media_detection(&mut self) -> &mut Self {
        self.detect_sensitive_media(SensitiveMediaDetection::None)
    }

    /// Sets sensitive media detection target to all.
    ///
    /// This is equivalent to `.detect_sensitive_media(SensitiveMediaDetection::All)`.
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub fn detect_sensitive_media_for_all_posts(&mut self) -> &mut Self {
        self.detect_sensitive_media(SensitiveMediaDetection::All)
    }

    /// Sets sensitive media detection target to local.
    ///
    /// This is equivalent to `.detect_sensitive_media(SensitiveMediaDetection::Local)`.
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub fn detect_sensitive_media_for_local_posts(&mut self) -> &mut Self {
        self.detect_sensitive_media(SensitiveMediaDetection::Local)
    }

    /// Sets sensitive media detection target to remote.
    ///
    /// This is equivalent to `.detect_sensitive_media(SensitiveMediaDetection::Remote)`.
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub fn detect_sensitive_media_for_remote_posts(&mut self) -> &mut Self {
        self.detect_sensitive_media(SensitiveMediaDetection::Remote)
    }

    /// Sets sensitivity of sensitive media detection.
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub fn sensitive_media_detection_sensitivity(
        &mut self,
        sensitivity: SensitiveMediaDetectionSensitivity,
    ) -> &mut Self {
        self.request
            .sensitive_media_detection_sensitivity
            .replace(sensitivity);
        self
    }

    /// Sets sensitivity of sensitive media detection to medium.
    ///
    /// This is equivalent to
    /// `.sensitive_media_detection_sensitivity(SensitiveMediaDetectionSensitivity::Medium)`.
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub fn sensitive_media_detection_medium_sensitivity(&mut self) -> &mut Self {
        self.sensitive_media_detection_sensitivity(SensitiveMediaDetectionSensitivity::Medium)
    }

    /// Sets sensitivity of sensitive media detection to low.
    ///
    /// This is equivalent to
    /// `.sensitive_media_detection_sensitivity(SensitiveMediaDetectionSensitivity::Low)`.
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub fn sensitive_media_detection_low_sensitivity(&mut self) -> &mut Self {
        self.sensitive_media_detection_sensitivity(SensitiveMediaDetectionSensitivity::Low)
    }

    /// Sets sensitivity of sensitive media detection to high.
    ///
    /// This is equivalent to
    /// `.sensitive_media_detection_sensitivity(SensitiveMediaDetectionSensitivity::High)`.
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub fn sensitive_media_detection_high_sensitivity(&mut self) -> &mut Self {
        self.sensitive_media_detection_sensitivity(SensitiveMediaDetectionSensitivity::High)
    }

    /// Sets sensitivity of sensitive media detection to very low.
    ///
    /// This is equivalent to
    /// `.sensitive_media_detection_sensitivity(SensitiveMediaDetectionSensitivity::VeryLow)`.
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub fn sensitive_media_detection_very_low_sensitivity(&mut self) -> &mut Self {
        self.sensitive_media_detection_sensitivity(SensitiveMediaDetectionSensitivity::VeryLow)
    }

    /// Sets sensitivity of sensitive media detection to very high.
    ///
    /// This is equivalent to
    /// `.sensitive_media_detection_sensitivity(SensitiveMediaDetectionSensitivity::VeryHigh)`.
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub fn sensitive_media_detection_very_high_sensitivity(&mut self) -> &mut Self {
        self.sensitive_media_detection_sensitivity(SensitiveMediaDetectionSensitivity::VeryHigh)
    }

    update_builder_bool_field! {
        /// Sets whether to set sensitive flag automatically on detected media.
        #[cfg(feature = "12-112-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
        pub set_sensitive_flag_automatically;
        /// Sets whether to enable sensitive media detection for videos.
        #[cfg(feature = "12-112-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
        pub enable_sensitive_media_detection_for_videos;
    }

    update_builder_option_field! {
        #[doc_name = "proxy account for the instance"]
        pub proxy_account: impl EntityRef<User> { proxy_account_id =  proxy_account.entity_ref() };
    }

    update_builder_string_option_field! {
        #[doc_name = "name of the instance maintainer"]
        pub maintainer_name;
        #[doc_name = "email of the instance maintainer"]
        pub maintainer_email;
    }

    update_builder_string_collection_field! {
        /// Sets the target language of the instance.
        pub languages { langs };
    }

    update_builder_option_field! {
        #[doc_name = "summaly proxy URL"]
        pub summaly_proxy: Url { summaly_proxy };
    }

    update_builder_string_option_field! {
        #[doc_name = "DeepL auth key"]
        #[cfg(feature = "12-88-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-88-0")))]
        pub deepl_auth_key;
    }

    update_builder_bool_field! {
        /// Sets whether or not DeepL is pro.
        #[cfg(feature = "12-89-1")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-89-1")))]
        pub deepl_is_pro;

        /// Sets whether or not to enable the Twitter integration.
        #[cfg(not(feature = "13-3-0"))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
        pub enable_twitter_integration;
    }
    update_builder_string_option_field! {
        #[doc_name = "Twitter consumer key"]
        #[cfg(not(feature = "13-3-0"))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
        pub twitter_consumer_key;
        #[doc_name = "Twitter consumer secret"]
        #[cfg(not(feature = "13-3-0"))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
        pub twitter_consumer_secret;
    }

    update_builder_bool_field! {
        /// Sets whether or not to enable the GitHub integration.
        #[cfg(not(feature = "13-3-0"))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
        pub enable_github_integration;
    }
    update_builder_string_option_field! {
        #[doc_name = "GitHub client ID"]
        #[cfg(not(feature = "13-3-0"))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
        pub github_client_id;
        #[doc_name = "GitHub client secret"]
        #[cfg(not(feature = "13-3-0"))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
        pub github_client_secret;
    }

    update_builder_bool_field! {
        /// Sets whether or not to enable the Discord integration.
        #[cfg(not(feature = "13-3-0"))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
        pub enable_discord_integration;
    }
    update_builder_string_option_field! {
        #[doc_name = "Discord client ID"]
        #[cfg(not(feature = "13-3-0"))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
        pub discord_client_id;
        #[doc_name = "Discord client secret"]
        #[cfg(not(feature = "13-3-0"))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "13-3-0"))))]
        pub discord_client_secret;
    }

    update_builder_bool_field! {
        /// Sets whether or not to enable email delivery.
        pub enable_email;
        /// Sets whether or not the SMTP server uses SSL.
        pub smtp_secure;
    }
    update_builder_string_option_field! {
        #[doc_name = "email address to be used for email delivery"]
        pub email_address { email };
        #[doc_name = "host of the SMTP server"]
        pub smtp_host;
        #[doc_name = "username of the SMTP server"]
        pub smtp_user;
        #[doc_name = "password of the SMTP server"]
        pub smtp_pass;
    }
    update_builder_option_field! {
        #[doc_name = "port number of the SMTP server"]
        pub smtp_port: u16 { smtp_port };
    }

    update_builder_bool_field! {
        /// Sets whether or not to enable the service worker.
        pub enable_service_worker;
    }
    update_builder_string_option_field! {
        #[doc_name = "public key for the service worker's VAPID key pair"]
        pub service_worker_public_key { sw_public_key };
        #[doc_name = "private key for the service worker's VAPID key pair"]
        pub service_worker_private_key { sw_private_key };
    }

    update_builder_string_option_field! {
        #[doc_name = "URL for the Terms of Service"]
        pub tos_url;
    }

    /// Sets the repository URL.
    pub fn repository_url(&mut self, url: Url) -> &mut Self {
        self.request.repository_url.replace(url);
        self
    }

    update_builder_string_option_field! {
        #[doc_name = "URL for the feedback"]
        pub feedback_url;
    }

    update_builder_bool_field! {
        /// Sets whether or not to use extenal object storage.
        pub use_object_storage;
        /// Sets whether or not the extenal object storage uses SSL.
        pub object_storage_use_ssl;
        /// Sets whether or not the extenal object storage uses the proxy.
        #[cfg(feature = "12-31-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-31-0")))]
        pub object_storage_use_proxy;
        /// Sets whether or not to set `'public-read'` when uploading to the extenal object
        /// storage.
        #[cfg(feature = "12-47-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
        pub object_storage_set_public_read;
        /// Sets whether or not to set `s3ForcePathStyle` option for the `aws-sdk` when using the
        /// external object storage.
        #[cfg(feature = "12-69-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-69-0")))]
        pub object_storage_s3_force_path_style;
        /// Sets whether or not to log ip address of the users.
        #[cfg(feature = "12-112-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
        pub enable_ip_logging;
        /// Sets whether or not to validate email address strictly.
        #[cfg(feature = "12-112-3")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-112-3")))]
        pub enable_active_email_validation;
    }
    update_builder_option_field! {
        #[doc_name = "base URL of the extenal object storage"]
        pub object_storage_base_url: Url { object_storage_base_url };
        #[doc_name = "port number of the extenal object storage"]
        pub object_storage_port: u16 { object_storage_port };
    }
    update_builder_string_option_field! {
        #[doc_name = "bucket name for the extenal object storage"]
        pub object_storage_bucket;
        #[doc_name = "prefix for the extenal object storage"]
        pub object_storage_prefix;
        #[doc_name = "endpoint for the extenal object storage"]
        pub object_storage_endpoint;
        #[doc_name = "region for the extenal object storage"]
        pub object_storage_region;
        #[doc_name = "access key for the extenal object storage"]
        pub object_storage_access_key;
        #[doc_name = "secret key for the extenal object storage"]
        pub object_storage_secret_key;
    }
}

impl<C: Client> MetaUpdateBuilder<C> {
    /// Updates the instance information.
    pub async fn update(&self) -> Result<(), Error<C::Error>> {
        self.client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(())
    }
}

/// Builder for the [`update_announcement`][`crate::ClientExt::update_announcement`] method.
pub struct AnnouncementUpdateBuilder<C> {
    client: C,
    request: endpoint::admin::announcements::update::Request,
}

impl<C> AnnouncementUpdateBuilder<C> {
    /// Creates a builder with the client and the announcement you are going to update.
    pub fn new(client: C, announcement: Announcement) -> Self {
        let Announcement {
            id,
            title,
            text,
            image_url,
            ..
        } = announcement;
        let request = endpoint::admin::announcements::update::Request {
            id,
            title,
            text,
            image_url,
        };
        AnnouncementUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::admin::announcements::update::Request {
        &self.request
    }

    /// Sets the title of the announcement.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.request.title = title.into();
        self
    }

    /// Sets the body text of the announcement.
    pub fn text(&mut self, text: impl Into<String>) -> &mut Self {
        self.request.text = text.into();
        self
    }

    /// Sets the image for the announcement.
    pub fn set_image(&mut self, image_url: Url) -> &mut Self {
        self.request.image_url.replace(image_url);
        self
    }

    /// Deletes the image of the announcement.
    pub fn delete_image(&mut self) -> &mut Self {
        self.request.image_url.take();
        self
    }
}

impl<C: Client> AnnouncementUpdateBuilder<C> {
    /// Updates the announcement.
    pub async fn update(&self) -> Result<(), Error<C::Error>> {
        self.client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(())
    }
}

/// Builder for the [`update_emoji`][`crate::ClientExt::update_emoji`] method.
pub struct EmojiUpdateBuilder<C> {
    client: C,
    request: endpoint::admin::emoji::update::Request,
}

impl<C> EmojiUpdateBuilder<C> {
    /// Creates a builder with the client and the emoji you are going to update.
    #[cfg(feature = "12-9-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-9-0")))]
    pub fn new(client: C, emoji: Emoji) -> Self {
        let Emoji {
            id,
            name,
            category,
            aliases,
            ..
        } = emoji;
        let request = endpoint::admin::emoji::update::Request {
            id,
            name,
            category,
            aliases,
        };
        EmojiUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::admin::emoji::update::Request {
        &self.request
    }

    /// Sets the name of the custom emoji.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name = name.into();
        self
    }

    /// Sets the category of the custom emoji.
    pub fn set_category(&mut self, category: impl Into<String>) -> &mut Self {
        self.request.category.replace(category.into());
        self
    }

    /// Deletes the category of the custom emoji.
    pub fn delete_category(&mut self) -> &mut Self {
        self.request.category.take();
        self
    }

    /// Sets the aliases of the custom emoji.
    pub fn aliases(&mut self, aliases: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.request.aliases = aliases.into_iter().map(Into::into).collect();
        self
    }

    /// Adds the given aliases to the custom emoji.
    pub fn add_aliases(
        &mut self,
        aliases: impl IntoIterator<Item = impl Into<String>>,
    ) -> &mut Self {
        self.request
            .aliases
            .extend(aliases.into_iter().map(Into::into));
        self
    }

    /// Adds the given alias to the custom emoji.
    pub fn add_alias(&mut self, alias: impl Into<String>) -> &mut Self {
        self.request.aliases.push(alias.into());
        self
    }
}

impl<C: Client> EmojiUpdateBuilder<C> {
    /// Updates the custom emoji.
    pub async fn update(&self) -> Result<(), Error<C::Error>> {
        self.client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(())
    }
}

#[cfg(feature = "12-80-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-80-0")))]
/// Builder for the [`build_ad`][`crate::ClientExt::build_ad`] method.
pub struct AdBuilder<C> {
    client: C,
    request: endpoint::admin::ad::create::Request,
}

#[cfg(feature = "12-80-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-80-0")))]
impl<C> AdBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::admin::ad::create::Request {
            url: String::default(),
            memo: String::default(),
            place: Place::default(),
            priority: Priority::default(),
            #[cfg(feature = "12-81-0")]
            ratio: 1,
            #[cfg(feature = "13-7-0")]
            starts_at: DateTime::default(),
            expires_at: DateTime::default(),
            image_url: String::default(),
        };
        AdBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::admin::ad::create::Request {
        &self.request
    }

    /// Sets the url of the ad.
    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.request.url = url.into();
        self
    }

    /// Sets the memo of the ad.
    pub fn memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.request.memo = memo.into();
        self
    }

    /// Sets the place of the ad.
    pub fn place(&mut self, place: Place) -> &mut Self {
        self.request.place = place;
        self
    }

    /// Sets the place of the ad to square.
    pub fn square(&mut self) -> &mut Self {
        self.place(Place::Square)
    }

    /// Sets the place of the ad to horizontal.
    pub fn horizontal(&mut self) -> &mut Self {
        self.place(Place::Horizontal)
    }

    #[cfg(feature = "12-81-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-81-0")))]
    /// Sets the place of the ad to horizontal-big.
    pub fn horizontal_big(&mut self) -> &mut Self {
        self.place(Place::HorizontalBig)
    }

    /// Sets the priority of the ad.
    pub fn priority(&mut self, priority: Priority) -> &mut Self {
        self.request.priority = priority;
        self
    }

    /// Sets the priority of the ad to high.
    pub fn high_priority(&mut self) -> &mut Self {
        self.priority(Priority::High)
    }

    /// Sets the priority of the ad to middle.
    pub fn middle_priority(&mut self) -> &mut Self {
        self.priority(Priority::Middle)
    }

    /// Sets the priority of the ad to low.
    pub fn low_priority(&mut self) -> &mut Self {
        self.priority(Priority::Low)
    }

    #[cfg(feature = "12-81-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-81-0")))]
    /// Sets the ratio of the ad.
    pub fn ratio(&mut self, ratio: u64) -> &mut Self {
        self.request.ratio = ratio;
        self
    }

    #[cfg(feature = "13-7-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-7-0")))]
    /// Sets the start date of the ad.
    pub fn starts_at(&mut self, starts_at: impl Into<DateTime<Utc>>) -> &mut Self {
        self.request.starts_at = starts_at.into();
        self
    }

    /// Sets the expiration date of the ad.
    pub fn expires_at(&mut self, expires_at: impl Into<DateTime<Utc>>) -> &mut Self {
        self.request.expires_at = expires_at.into();
        self
    }

    /// Sets the image url of the ad.
    pub fn image_url(&mut self, image_url: impl Into<String>) -> &mut Self {
        self.request.image_url = image_url.into();
        self
    }
}

#[cfg(feature = "12-80-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-80-0")))]
impl<C: Client> AdBuilder<C> {
    /// Creates the ad.
    pub async fn create(&self) -> Result<(), Error<C::Error>> {
        self.client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(())
    }
}

#[cfg(feature = "12-80-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-80-0")))]
/// Builder for the [`update_ad`][`crate::ClientExt::update_ad`] method.
pub struct AdUpdateBuilder<C> {
    client: C,
    request: endpoint::admin::ad::update::Request,
}

#[cfg(feature = "12-80-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-80-0")))]
impl<C> AdUpdateBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C, ad: Ad) -> Self {
        let Ad {
            id,
            #[cfg(feature = "13-7-0")]
            starts_at,
            expires_at,
            place,
            priority,
            #[cfg(feature = "12-81-0")]
            ratio,
            url,
            image_url,
            memo,
            ..
        } = ad;
        let request = endpoint::admin::ad::update::Request {
            id,
            url,
            memo,
            place,
            priority,
            #[cfg(feature = "12-81-0")]
            ratio,
            #[cfg(feature = "13-7-0")]
            starts_at,
            expires_at,
            image_url,
        };
        AdUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::admin::ad::update::Request {
        &self.request
    }

    /// Sets the url of the ad.
    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.request.url = url.into();
        self
    }

    /// Sets the memo of the ad.
    pub fn memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.request.memo = memo.into();
        self
    }

    /// Sets the place of the ad.
    pub fn place(&mut self, place: Place) -> &mut Self {
        self.request.place = place;
        self
    }

    /// Sets the place of the ad to square.
    pub fn square(&mut self) -> &mut Self {
        self.place(Place::Square)
    }

    /// Sets the place of the ad to horizontal.
    pub fn horizontal(&mut self) -> &mut Self {
        self.place(Place::Horizontal)
    }

    #[cfg(feature = "12-81-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-81-0")))]
    /// Sets the place of the ad to horizontal-big.
    pub fn horizontal_big(&mut self) -> &mut Self {
        self.place(Place::HorizontalBig)
    }

    /// Sets the priority of the ad.
    pub fn priority(&mut self, priority: Priority) -> &mut Self {
        self.request.priority = priority;
        self
    }

    /// Sets the priority of the ad to high.
    pub fn high_priority(&mut self) -> &mut Self {
        self.priority(Priority::High)
    }

    /// Sets the priority of the ad to middle.
    pub fn middle_priority(&mut self) -> &mut Self {
        self.priority(Priority::Middle)
    }

    /// Sets the priority of the ad to low.
    pub fn low_priority(&mut self) -> &mut Self {
        self.priority(Priority::Low)
    }

    #[cfg(feature = "12-81-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-81-0")))]
    /// Sets the ratio of the ad.
    pub fn ratio(&mut self, ratio: u64) -> &mut Self {
        self.request.ratio = ratio;
        self
    }

    #[cfg(feature = "13-7-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-7-0")))]
    /// Sets the start date of the ad.
    pub fn starts_at(&mut self, starts_at: impl Into<DateTime<Utc>>) -> &mut Self {
        self.request.starts_at = starts_at.into();
        self
    }
    /// Sets the expiration date of the ad.
    pub fn expires_at(&mut self, expires_at: impl Into<DateTime<Utc>>) -> &mut Self {
        self.request.expires_at = expires_at.into();
        self
    }

    /// Sets the image url of the ad.
    pub fn image_url(&mut self, image_url: impl Into<String>) -> &mut Self {
        self.request.image_url = image_url.into();
        self
    }
}

#[cfg(feature = "12-80-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-80-0")))]
impl<C: Client> AdUpdateBuilder<C> {
    /// Updates the ad.
    pub async fn update(&self) -> Result<(), Error<C::Error>> {
        self.client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(())
    }
}

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
/// Builder for building [`PolicyValue`].
pub struct PolicyValueBuilder<T> {
    use_default: bool,
    priority: role::Priority,
    value: T,
}

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
impl<T> PolicyValueBuilder<T> {
    /// Creates a builder.
    pub fn new(value: T) -> Self {
        PolicyValueBuilder {
            use_default: false,
            priority: role::Priority::Low,
            value,
        }
    }

    /// Sets whether to use default policy.
    pub fn use_default(&mut self, use_default: bool) -> &mut Self {
        self.use_default = use_default;
        self
    }

    /// Sets the priority of the value.
    pub fn priority(&mut self, priority: role::Priority) -> &mut Self {
        self.priority = priority;
        self
    }

    /// Sets the priority of the value to high.
    pub fn priority_high(&mut self) -> &mut Self {
        self.priority(role::Priority::High)
    }

    /// Sets the priority of the value to middle.
    pub fn priority_middle(&mut self) -> &mut Self {
        self.priority(role::Priority::Middle)
    }

    /// Sets the priority of the value to low.
    pub fn priority_low(&mut self) -> &mut Self {
        self.priority(role::Priority::Low)
    }

    /// Sets the policy value.
    pub fn value(&mut self, value: T) -> &mut Self {
        self.value = value;
        self
    }
}

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
impl<T: Clone> PolicyValueBuilder<T> {
    pub fn build(&self) -> PolicyValue<T> {
        PolicyValue {
            use_default: self.use_default,
            priority: self.priority.to_owned(),
            value: self.value.to_owned(),
        }
    }
}

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
/// Builder for the [`build_role`][`crate::ClientExt::build_role`] method.
pub struct RoleBuilder<C> {
    client: C,
    request: endpoint::admin::roles::create::Request,
}

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
impl<C> RoleBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::admin::roles::create::Request::default();
        RoleBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::admin::roles::create::Request {
        &self.request
    }

    /// Sets the name of the role.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name = name.into();
        self
    }

    /// Sets the description of the role.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.request.description = description.into();
        self
    }

    /// Sets the color of the role.
    pub fn color(&mut self, color: impl Into<String>) -> &mut Self {
        self.request.color.replace(color.into());
        self
    }

    /// Sets the icon url of the role.
    #[cfg(feature = "13-4-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-4-0")))]
    pub fn icon_url(&mut self, icon_url: impl Into<String>) -> &mut Self {
        self.request.icon_url.replace(icon_url.into());
        self
    }

    /// Sets the assignment type of the role.
    pub fn target(&mut self, target: impl Into<Target>) -> &mut Self {
        self.request.target = target.into();
        self
    }

    /// Sets the role to be assigned manually.
    ///
    /// This is equivalent to `.target(Target::Manual)`.
    pub fn manual(&mut self) -> &mut Self {
        self.target(Target::Manual)
    }

    /// Sets the role to be assigned automatically based on the condition.
    ///
    /// This is equivalent to `.target(Target::Conditional)`.
    pub fn conditional(&mut self) -> &mut Self {
        self.target(Target::Conditional)
    }

    /// Sets the condition of the role.
    pub fn condition(&mut self, condition: impl Into<RoleCondFormulaValue>) -> &mut Self {
        self.conditional();
        self.request.cond_formula.replace(condition.into());
        self
    }

    /// Sets whether the role is public or not.
    pub fn public(&mut self, public: bool) -> &mut Self {
        self.request.is_public = public;
        self
    }

    /// Sets whether to give the moderator permission to the members of the role.
    pub fn moderator(&mut self, moderator: bool) -> &mut Self {
        self.request.is_moderator = moderator;
        self
    }

    /// Sets whether to give the administrator permission to the members of the role.
    pub fn administrator(&mut self, administrator: bool) -> &mut Self {
        self.request.is_administrator = administrator;
        self
    }

    /// Sets whether to show the icon image next to the usernames.
    #[cfg(feature = "13-4-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-4-0")))]
    pub fn show_as_badge(&mut self, as_badge: bool) -> &mut Self {
        self.request.as_badge = as_badge;
        self
    }

    /// Sets whether to allow moderators to edit members of the role.
    pub fn allow_moderator_to_edit_members(
        &mut self,
        can_edit_members_by_moderator: bool,
    ) -> &mut Self {
        self.request.can_edit_members_by_moderator = can_edit_members_by_moderator;
        self
    }

    /// Sets the policies of the role.
    pub fn policies(&mut self, policies: impl Into<Policies>) -> &mut Self {
        self.request.policies = policies.into();
        self
    }

    /// Sets whether to allow the members of the role to view the global timeline.
    pub fn allow_global_timeline(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<bool>) -> PolicyValue<bool>,
    ) -> &mut Self {
        self.request
            .policies
            .gtl_available
            .replace(build_policy_value(PolicyValueBuilder::new(false)));
        self
    }

    /// Sets whether to allow the members of the role to view the local timeline.
    pub fn allow_local_timeline(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<bool>) -> PolicyValue<bool>,
    ) -> &mut Self {
        self.request
            .policies
            .ltl_available
            .replace(build_policy_value(PolicyValueBuilder::new(false)));
        self
    }

    /// Sets whether to allow the members of the role to post public note.
    pub fn allow_public_note(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<bool>) -> PolicyValue<bool>,
    ) -> &mut Self {
        self.request
            .policies
            .can_public_note
            .replace(build_policy_value(PolicyValueBuilder::new(false)));
        self
    }

    /// Sets whether to allow the members of the role to create invitation code of the instance.
    pub fn allow_invitation(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<bool>) -> PolicyValue<bool>,
    ) -> &mut Self {
        self.request
            .policies
            .can_invite
            .replace(build_policy_value(PolicyValueBuilder::new(false)));
        self
    }

    /// Sets whether to allow the members of the role to manage custom emojis.
    pub fn allow_custom_emojis_management(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<bool>) -> PolicyValue<bool>,
    ) -> &mut Self {
        self.request
            .policies
            .can_manage_custom_emojis
            .replace(build_policy_value(PolicyValueBuilder::new(false)));
        self
    }

    /// Sets whether to allow the members of the role to hide ads.
    pub fn allow_hiding_ads(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<bool>) -> PolicyValue<bool>,
    ) -> &mut Self {
        self.request
            .policies
            .can_hide_ads
            .replace(build_policy_value(PolicyValueBuilder::new(false)));
        self
    }

    /// Sets the drive capacity of the members of the role in megabytes.
    pub fn drive_capacity(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .drive_capacity_mb
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of pinned notes for the members of the role.
    pub fn pin_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .pin_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of antennas for the members of the role.
    pub fn antenna_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .antenna_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of characters in word mutes for the members of the role.
    pub fn word_mute_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .word_mute_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of webhooks for the members of the role.
    pub fn webhook_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .webhook_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of clips for the members of the role.
    pub fn clip_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .clip_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of notes per clip for the members of the role.
    pub fn note_each_clips_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .note_each_clips_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of user lists for the members of the role.
    pub fn user_list_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .user_list_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of users per user list for the members of the role.
    pub fn user_each_user_lists_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .user_each_user_lists_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the rate limit factor of the members of the role.
    pub fn rate_limit_factor(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<f64>) -> PolicyValue<f64>,
    ) -> &mut Self {
        self.request
            .policies
            .rate_limit_factor
            .replace(build_policy_value(PolicyValueBuilder::new(0.0)));
        self
    }
}

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
impl<C: Client> RoleBuilder<C> {
    /// Creates the role.
    pub async fn create(&self) -> Result<Role, Error<C::Error>> {
        let role = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(role)
    }
}

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
/// Builder for the [`update_role`][`crate::ClientExt::update_role`] method.
pub struct RoleUpdateBuilder<C> {
    client: C,
    request: endpoint::admin::roles::update::Request,
}

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
impl<C> RoleUpdateBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C, role: Role) -> Self {
        let Role {
            id,
            name,
            description,
            color,
            #[cfg(feature = "13-4-0")]
            icon_url,
            target,
            cond_formula,
            is_public,
            is_moderator,
            is_administrator,
            #[cfg(feature = "13-4-0")]
            as_badge,
            can_edit_members_by_moderator,
            policies,
            ..
        } = role;
        let request = endpoint::admin::roles::update::Request {
            role_id: id,
            name,
            description,
            color,
            #[cfg(feature = "13-4-0")]
            icon_url,
            target,
            cond_formula,
            is_public,
            is_moderator,
            is_administrator,
            #[cfg(feature = "13-4-0")]
            as_badge,
            can_edit_members_by_moderator,
            policies,
        };
        RoleUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::admin::roles::update::Request {
        &self.request
    }

    /// Sets the name of the role.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name = name.into();
        self
    }

    /// Sets the description of the role.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.request.description = description.into();
        self
    }

    /// Sets the color of the role.
    pub fn color(&mut self, color: impl Into<String>) -> &mut Self {
        self.request.color.replace(color.into());
        self
    }

    /// Sets the icon url of the role.
    #[cfg(feature = "13-4-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-4-0")))]
    pub fn icon_url(&mut self, icon_url: impl Into<String>) -> &mut Self {
        self.request.icon_url.replace(icon_url.into());
        self
    }

    /// Sets the assignment type of the role.
    pub fn target(&mut self, target: impl Into<Target>) -> &mut Self {
        self.request.target = target.into();
        self
    }

    /// Sets the role to be assigned manually.
    ///
    /// This is equivalent to `.target(Target::Manual)`.
    pub fn manual(&mut self) -> &mut Self {
        self.target(Target::Manual)
    }

    /// Sets the role to be assigned automatically based on conditions.
    ///
    /// This is equivalent to `.target(Target::Conditional)`.
    pub fn conditional(&mut self) -> &mut Self {
        self.target(Target::Conditional)
    }

    /// Sets the conditions of the role.
    pub fn cond_formula(&mut self, cond_formula: impl Into<RoleCondFormulaValue>) -> &mut Self {
        self.request.cond_formula.replace(cond_formula.into());
        self
    }

    /// Sets whether the role is public or not.
    pub fn public(&mut self, public: bool) -> &mut Self {
        self.request.is_public = public;
        self
    }

    /// Sets whether to give the moderator permission to the members of the role.
    pub fn moderator(&mut self, moderator: bool) -> &mut Self {
        self.request.is_moderator = moderator;
        self
    }

    /// Sets whether to give the administrator permission to the members of the role.
    pub fn administrator(&mut self, administrator: bool) -> &mut Self {
        self.request.is_administrator = administrator;
        self
    }

    /// Sets whether to show the icon image next to the usernames.
    #[cfg(feature = "13-4-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-4-0")))]
    pub fn show_as_badge(&mut self, as_badge: bool) -> &mut Self {
        self.request.as_badge = as_badge;
        self
    }

    /// Sets whether to allow moderators to edit members of the role.
    pub fn allow_moderator_to_edit_members(
        &mut self,
        can_edit_members_by_moderator: bool,
    ) -> &mut Self {
        self.request.can_edit_members_by_moderator = can_edit_members_by_moderator;
        self
    }

    /// Sets the policies of the role.
    pub fn policies(&mut self, policies: impl Into<Policies>) -> &mut Self {
        self.request.policies = policies.into();
        self
    }

    /// Sets whether to allow the members of the role to view the global timeline.
    pub fn allow_global_timeline(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<bool>) -> PolicyValue<bool>,
    ) -> &mut Self {
        self.request
            .policies
            .gtl_available
            .replace(build_policy_value(PolicyValueBuilder::new(false)));
        self
    }

    /// Sets whether to allow the members of the role to view the local timeline.
    pub fn allow_local_timeline(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<bool>) -> PolicyValue<bool>,
    ) -> &mut Self {
        self.request
            .policies
            .ltl_available
            .replace(build_policy_value(PolicyValueBuilder::new(false)));
        self
    }

    /// Sets whether to allow the members of the role to post public note.
    pub fn allow_public_note(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<bool>) -> PolicyValue<bool>,
    ) -> &mut Self {
        self.request
            .policies
            .can_public_note
            .replace(build_policy_value(PolicyValueBuilder::new(false)));
        self
    }

    /// Sets whether to allow the members of the role to create invitation code of the instance.
    pub fn allow_invitation(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<bool>) -> PolicyValue<bool>,
    ) -> &mut Self {
        self.request
            .policies
            .can_invite
            .replace(build_policy_value(PolicyValueBuilder::new(false)));
        self
    }

    /// Sets whether to allow the members of the role to manage custom emojis.
    pub fn allow_custom_emojis_management(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<bool>) -> PolicyValue<bool>,
    ) -> &mut Self {
        self.request
            .policies
            .can_manage_custom_emojis
            .replace(build_policy_value(PolicyValueBuilder::new(false)));
        self
    }

    /// Sets whether to allow the members of the role to hide ads.
    pub fn allow_hiding_ads(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<bool>) -> PolicyValue<bool>,
    ) -> &mut Self {
        self.request
            .policies
            .can_hide_ads
            .replace(build_policy_value(PolicyValueBuilder::new(false)));
        self
    }

    /// Sets the drive capacity of the members of the role in megabytes.
    pub fn drive_capacity(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .drive_capacity_mb
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of pinned notes for the members of the role.
    pub fn pin_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .pin_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of antennas for the members of the role.
    pub fn antenna_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .antenna_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of characters in word mutes for the members of the role.
    pub fn word_mute_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .word_mute_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of webhooks for the members of the role.
    pub fn webhook_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .webhook_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of clips for the members of the role.
    pub fn clip_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .clip_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of notes per clip for the members of the role.
    pub fn note_each_clips_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .note_each_clips_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of user lists for the members of the role.
    pub fn user_list_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .user_list_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the maximum number of users per user list for the members of the role.
    pub fn user_each_user_lists_limit(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<u64>) -> PolicyValue<u64>,
    ) -> &mut Self {
        self.request
            .policies
            .user_each_user_lists_limit
            .replace(build_policy_value(PolicyValueBuilder::new(0)));
        self
    }

    /// Sets the rate limit factor of the members of the role.
    pub fn rate_limit_factor(
        &mut self,
        build_policy_value: impl FnOnce(PolicyValueBuilder<f64>) -> PolicyValue<f64>,
    ) -> &mut Self {
        self.request
            .policies
            .rate_limit_factor
            .replace(build_policy_value(PolicyValueBuilder::new(0.0)));
        self
    }
}

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
impl<C: Client> RoleUpdateBuilder<C> {
    /// Updates the role.
    pub async fn update(&self) -> Result<(), Error<C::Error>> {
        self.client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(())
    }
}

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
/// Builder for the [`update_role`][`crate::ClientExt::update_role`] method.
pub struct DefaultPoliciesUpdateBuilder<C> {
    client: C,
    request: endpoint::admin::roles::update_default_policies::Request,
}

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
impl<C> DefaultPoliciesUpdateBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C, policies: PoliciesSimple) -> Self {
        let request = endpoint::admin::roles::update_default_policies::Request { policies };
        DefaultPoliciesUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::admin::roles::update_default_policies::Request {
        &self.request
    }

    /// Sets whether to allow users to view the global timeline.
    pub fn allow_global_timeline(&mut self, allow_global_timeline: bool) -> &mut Self {
        self.request
            .policies
            .gtl_available
            .replace(allow_global_timeline);
        self
    }

    /// Sets whether to allow users to view the local timeline.
    pub fn allow_local_timeline(&mut self, allow_local_timeline: bool) -> &mut Self {
        self.request
            .policies
            .ltl_available
            .replace(allow_local_timeline);
        self
    }

    /// Sets whether to allow users to post public note.
    pub fn allow_public_note(&mut self, allow_public_note: bool) -> &mut Self {
        self.request
            .policies
            .can_public_note
            .replace(allow_public_note);
        self
    }

    /// Sets whether to allow users to create invitation code of the instance.
    pub fn allow_invitation(&mut self, allow_invitation: bool) -> &mut Self {
        self.request.policies.can_invite.replace(allow_invitation);
        self
    }

    /// Sets whether to allow users to manage custom emojis.
    pub fn allow_custom_emojis_management(
        &mut self,
        allow_custom_emojis_management: bool,
    ) -> &mut Self {
        self.request
            .policies
            .can_manage_custom_emojis
            .replace(allow_custom_emojis_management);
        self
    }

    /// Sets whether to allow users to hide ads.
    pub fn allow_hiding_ads(&mut self, allow_hiding_ads: bool) -> &mut Self {
        self.request.policies.can_hide_ads.replace(allow_hiding_ads);
        self
    }

    /// Sets the drive capacity per user in megabytes.
    pub fn drive_capacity(&mut self, mb: u64) -> &mut Self {
        self.request.policies.drive_capacity_mb.replace(mb);
        self
    }

    /// Sets the maximum number of pinned notes for the users.
    pub fn pin_limit(&mut self, pin_limit: u64) -> &mut Self {
        self.request.policies.pin_limit.replace(pin_limit);
        self
    }

    /// Sets the maximum number of antennas for the users.
    pub fn antenna_limit(&mut self, antenna_limit: u64) -> &mut Self {
        self.request.policies.antenna_limit.replace(antenna_limit);
        self
    }

    /// Sets the maximum number of characters in word mutes for the users.
    pub fn word_mute_limit(&mut self, word_mute_limit: u64) -> &mut Self {
        self.request
            .policies
            .word_mute_limit
            .replace(word_mute_limit);
        self
    }

    /// Sets the maximum number of webhooks for the users.
    pub fn webhook_limit(&mut self, webhook_limit: u64) -> &mut Self {
        self.request.policies.webhook_limit.replace(webhook_limit);
        self
    }

    /// Sets the maximum number of clips for the users.
    pub fn clip_limit(&mut self, clip_limit: u64) -> &mut Self {
        self.request.policies.clip_limit.replace(clip_limit);
        self
    }

    /// Sets the maximum number of notes per clip for the users.
    pub fn note_each_clips_limit(&mut self, note_each_clips_limit: u64) -> &mut Self {
        self.request
            .policies
            .note_each_clips_limit
            .replace(note_each_clips_limit);
        self
    }

    /// Sets the maximum number of user lists for the users.
    pub fn user_list_limit(&mut self, user_list_limit: u64) -> &mut Self {
        self.request
            .policies
            .user_list_limit
            .replace(user_list_limit);
        self
    }

    /// Sets the maximum number of users per user list for the users.
    pub fn user_each_user_lists_limit(&mut self, user_each_user_lists_limit: u64) -> &mut Self {
        self.request
            .policies
            .user_each_user_lists_limit
            .replace(user_each_user_lists_limit);
        self
    }

    /// Sets the rate limit factor of users.
    pub fn rate_limit_factor(&mut self, rate_limit_factor: f64) -> &mut Self {
        self.request
            .policies
            .rate_limit_factor
            .replace(rate_limit_factor);
        self
    }
}

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
impl<C: Client> DefaultPoliciesUpdateBuilder<C> {
    /// Updates the default policies.
    pub async fn update(&self) -> Result<(), Error<C::Error>> {
        self.client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(())
    }
}
