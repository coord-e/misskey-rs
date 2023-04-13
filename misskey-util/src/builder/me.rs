#[cfg(feature = "12-48-0")]
use std::collections::HashSet;

use crate::Error;

#[cfg(feature = "12-48-0")]
use misskey_api::model::notification::NotificationType;
#[cfg(feature = "12-96-0")]
use misskey_api::model::user::FfVisibility;
#[cfg(feature = "12-70-0")]
use misskey_api::model::user::UserEmailNotificationType;
use misskey_api::model::{
    drive::DriveFile,
    page::Page,
    query::Query,
    user::{User, UserField},
};
use misskey_api::{endpoint, EntityRef};
use misskey_core::Client;

/// Conversion to fields in users' profile.
///
/// The purpose of this trait is to specify the type that the [`set_fields`][set_fields] method
/// takes as its parameter. This trait is implemented for arrays of length from 1 to 4, whose
/// elements are name-value string pairs or [`UserField`][user_field] objects.
///
/// [set_fields]: MeUpdateBuilder::set_fields
/// [user_field]: misskey_api::model::user::UserField
pub trait IntoUserFields {
    /// Performs the conversion.
    fn into_user_fields(self) -> [Option<UserField>; 4];
}

macro_rules! impl_into_field_requests {
    (expand default) => { None };
    (expand $i:ident) => { Some($i) };
    (expand_pair default $name:ident $value:ident) => { None };
    (expand_pair $i:ident $name:ident $value:ident) => {
        Some(UserField {
            name: $name.into(),
            value: $value.into(),
        })
    };
    ($len:expr; $($in_field:ident),* => $($out_field:ident),*) => {
        paste::paste! {
            impl IntoUserFields for [UserField; $len] {
                fn into_user_fields(self) -> [Option<UserField>; 4] {
                    let [$($in_field,)*] = self;
                    [$(impl_into_field_requests!(expand $out_field), )*]
                }
            }
            impl<T, U> IntoUserFields for [(T, U); $len] where T: Into<String>, U: Into<String> {
                fn into_user_fields(self) -> [Option<UserField>; 4] {
                    let [$(([<name_ $in_field>], [<value_ $in_field>]),)*] = self;
                    [$(impl_into_field_requests!(expand_pair $out_field [<name_ $out_field>] [<value_ $out_field>]), )*]
                }
            }
        }
    };
}

impl_into_field_requests! { 1; f1 => f1, default, default, default }
impl_into_field_requests! { 2; f1, f2 => f1, f2, default, default }
impl_into_field_requests! { 3; f1, f2, f3 => f1, f2, f3, default }
impl_into_field_requests! { 4; f1, f2, f3, f4 => f1, f2, f3, f4 }

/// Builder for the [`update_me`][`crate::ClientExt::update_me`] method.
pub struct MeUpdateBuilder<C> {
    client: C,
    request: endpoint::i::update::Request,
}

impl<C> MeUpdateBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        MeUpdateBuilder {
            client,
            request: endpoint::i::update::Request::default(),
        }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::i::update::Request {
        &self.request
    }

    update_builder_string_option_field! {
        pub name;
        pub description;
        pub language { lang };
        pub location;
        pub birthday;
    }

    update_builder_option_field! {
        pub avatar: impl EntityRef<DriveFile> { avatar_id = avatar.entity_ref() };
        pub banner: impl EntityRef<DriveFile> { banner_id = banner.entity_ref() };
        #[doc_name = "pinned page"]
        pub pinned_page: impl EntityRef<Page> { pinned_page_id = pinned_page.entity_ref() };
    }

    /// Sets the fields in this user's profile.
    ///
    /// Since the user has four fields, it takes an array of length 1 to 4 as its argument.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// client
    ///     .update_me()
    ///     .set_fields([
    ///         ("Website", "https://example.com/"),
    ///         ("Twitter", "@username"),
    ///     ])
    ///     .update()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_fields(&mut self, fields: impl IntoUserFields) -> &mut Self {
        fn to_request(field: UserField) -> endpoint::i::update::UserFieldRequest {
            endpoint::i::update::UserFieldRequest {
                name: Some(field.name),
                value: Some(field.value),
            }
        }
        let [f1, f2, f3, f4] = fields.into_user_fields();
        let fields = [
            f1.map(to_request).unwrap_or_default(),
            f2.map(to_request).unwrap_or_default(),
            f3.map(to_request).unwrap_or_default(),
            f4.map(to_request).unwrap_or_default(),
        ];
        self.request.fields.replace(fields);
        self
    }

    /// Deletes all the fields in this user's profile.
    pub fn delete_fields(&mut self) -> &mut Self {
        self.request.fields.replace(Default::default());
        self
    }

    update_builder_bool_field! {
        /// Sets whether this user is locked or not.
        pub locked { is_locked };

        /// Sets whether this user is visible in "Explore" section of the instance.
        #[cfg(feature = "12-63-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-63-0")))]
        pub explorable { is_explorable };

        /// Sets whether to hide online status from other users.
        #[cfg(feature = "12-77-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-77-0")))]
        pub hide_online_status;

        /// Sets whether to make the reactions public.
        #[cfg(feature = "12-93-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-93-0")))]
        pub public_reactions;

        /// Sets whether this user requires a follow request from bots.
        pub require_follow_request_for_bot { careful_bot };

        /// Sets whether to automatically accept follow requests from following users.
        pub auto_accept_followed;

        /// Sets whether to display this user as a bot.
        pub bot { is_bot };

        /// Sets whether to display this user as a cot.
        pub cat { is_cat };

        /// Sets whether to display featured notes in the timeline.
        pub inject_featured_note;

        /// Sets whether to mark uploaded media as NSFW by default.
        pub always_mark_nsfw;

        /// Sets whether to receive notifications about other users' notes that this user has
        /// reacted to or replied to.
        #[cfg(any(docsrs, not(feature = "12-55-0")))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "12-55-0"))))]
        pub auto_watch;

        /// Sets whether to ask search engines not to index this user's contents.
        #[cfg(feature = "12-60-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-60-0")))]
        pub no_crawle;

        /// Sets whether to receive announcement emails.
        #[cfg(feature = "12-69-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-69-0")))]
        pub receive_announcement_email;
    }

    /// Sets the visibility of following and followers.
    #[cfg(feature = "12-96-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-96-0")))]
    pub fn ff_visibility(&mut self, ff_visibility: impl Into<FfVisibility>) -> &mut Self {
        self.request.ff_visibility.replace(ff_visibility.into());
        self
    }

    /// Sets following/followers to be visible to everyone.
    ///
    /// This is equivalent to `.ff_visibility(FfVisibility::Public)`.
    #[cfg(feature = "12-96-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-96-0")))]
    pub fn ff_public(&mut self) -> &mut Self {
        self.ff_visibility(FfVisibility::Public)
    }

    /// Sets following/followers to be visible only to the followers.
    ///
    /// This is equivalent to `.ff_visibility(FfVisibility::Followers)`.
    #[cfg(feature = "12-96-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-96-0")))]
    pub fn ff_followers(&mut self) -> &mut Self {
        self.ff_visibility(FfVisibility::Followers)
    }

    /// Sets following/followers to be invisible to other users.
    ///
    /// This is equivalent to `.ff_visibility(FfVisibility::Private)`.
    #[cfg(feature = "12-96-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-96-0")))]
    pub fn ff_private(&mut self) -> &mut Self {
        self.ff_visibility(FfVisibility::Private)
    }

    /// Sets the muted notification type for this user.
    ///
    /// Note that you can subsequently use this method to add more muted notification types to be used for updates.
    ///
    /// # Examples
    ///
    /// The example below updates the user setting to mute `'follow'` and `'reaction'` notifications.
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # use misskey_api as misskey;
    /// use misskey::model::notification::NotificationType;
    ///
    /// client
    ///     .update_me()
    ///     .muted_notification_type(NotificationType::Follow)
    ///     .muted_notification_type(NotificationType::Reaction)
    ///     .update()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    pub fn muted_notification_type(&mut self, notification_type: NotificationType) -> &mut Self {
        self.request
            .muting_notification_types
            .get_or_insert_with(HashSet::new)
            .insert(notification_type);
        self
    }

    /// Sets the muted notification types for this user.
    ///
    /// Note that you can subsequently use this method to add more muted notification types to be used for updates.
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    pub fn muted_notification_types(
        &mut self,
        notification_types: impl IntoIterator<Item = NotificationType>,
    ) -> &mut Self {
        self.request
            .muting_notification_types
            .get_or_insert_with(HashSet::new)
            .extend(notification_types);
        self
    }

    /// Sets the email notification type for this user.
    ///
    /// Note that you can subsequently use this method to add more email notification types to be used for updates.
    ///
    /// # Examples
    ///
    /// The example below updates the user setting to email `'follow'` and `'groupInvited'` notifications.
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # use misskey_api as misskey;
    /// use misskey::model::user::UserEmailNotificationType;
    ///
    /// client
    ///     .update_me()
    ///     .email_notification_type(UserEmailNotificationType::Follow)
    ///     .email_notification_type(UserEmailNotificationType::GroupInvited)
    ///     .update()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "12-70-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-70-0")))]
    pub fn email_notification_type(
        &mut self,
        notification_type: UserEmailNotificationType,
    ) -> &mut Self {
        self.request
            .email_notification_types
            .get_or_insert_with(HashSet::new)
            .insert(notification_type);
        self
    }

    /// Sets the email notification types for this user.
    ///
    /// Note that you can subsequently use this method to add more email notification types to be used for updates.
    #[cfg(feature = "12-70-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-70-0")))]
    pub fn email_notification_types(
        &mut self,
        notification_types: impl IntoIterator<Item = UserEmailNotificationType>,
    ) -> &mut Self {
        self.request
            .email_notification_types
            .get_or_insert_with(HashSet::new)
            .extend(notification_types);
        self
    }

    /// Sets the muted words for this user.
    pub fn muted_words(&mut self, muted_words: impl Into<Query<String>>) -> &mut Self {
        self.request.muted_words.replace(muted_words.into());
        self
    }

    update_builder_string_collection_field! {
        /// Sets the muted instances for this user.
        #[cfg(feature = "12-99-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-99-0")))]
        pub muted_instances;
    }
}

impl<C: Client> MeUpdateBuilder<C> {
    /// Updates the user.
    pub async fn update(&self) -> Result<User, Error<C::Error>> {
        let response = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(response)
    }
}
