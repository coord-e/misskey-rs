use crate::Error;

#[cfg(feature = "12-62-0")]
use misskey_api::model::clip::Clip;
#[cfg(feature = "12-9-0")]
use misskey_api::model::emoji::Emoji;
use misskey_api::model::{
    announcement::Announcement,
    log::{Log, LogLevel},
    user::User,
};
use misskey_api::{endpoint, EntityRef};
use misskey_core::Client;
use url::Url;

/// Builder for the [`server_logs`][`crate::ClientExt::server_logs`] method.
pub struct ServerLogListBuilder<C> {
    client: C,
    request: endpoint::admin::logs::Request,
}

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
        pub disable_local_timeline;
        /// Sets whether the instance has global timeline enabled.
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
        #[doc_name = "URL of the mascot image for the instance"]
        pub mascot_image_url;
        #[doc_name = "URL of the banner image for the instance"]
        pub bannar_url;
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
    pub fn max_note_text_length(&mut self, max_note_text_length: u64) -> &mut Self {
        self.request
            .max_note_text_length
            .replace(max_note_text_length);
        self
    }

    /// Sets the drive capacity per local user in megabytes.
    pub fn local_drive_capacity(&mut self, mb: u64) -> &mut Self {
        self.request.local_drive_capacity_mb.replace(mb);
        self
    }

    /// Sets the drive capacity per remote user in megabytes.
    pub fn remote_drive_capacity(&mut self, mb: u64) -> &mut Self {
        self.request.remote_drive_capacity_mb.replace(mb);
        self
    }

    update_builder_bool_field! {
        /// Sets whether or not the instance would cache remote files.
        pub cache_remote_files;
        /// Sets whether or not the instance would proxy remote files that are not available
        /// locally.
        pub proxy_remote_files;
    }

    update_builder_bool_field! {
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

    update_builder_bool_field! {
        /// Sets whether or not to enable the Twitter integration.
        pub enable_twitter_integration;
    }
    update_builder_string_option_field! {
        #[doc_name = "Twitter consumer key"]
        pub twitter_consumer_key;
        #[doc_name = "Twitter consumer secret"]
        pub twitter_consumer_secret;
    }

    update_builder_bool_field! {
        /// Sets whether or not to enable the GitHub integration.
        pub enable_github_integration;
    }
    update_builder_string_option_field! {
        #[doc_name = "GitHub client ID"]
        pub github_client_id;
        #[doc_name = "GitHub client secret"]
        pub github_client_secret;
    }

    update_builder_bool_field! {
        /// Sets whether or not to enable the Discord integration.
        pub enable_discord_integration;
    }
    update_builder_string_option_field! {
        #[doc_name = "Discord client ID"]
        pub discord_client_id;
        #[doc_name = "Discord client secret"]
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
