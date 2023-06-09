#[cfg(feature = "12-67-0")]
use std::collections::HashMap;
use std::path::Path;

#[cfg(feature = "12-9-0")]
use crate::builder::EmojiUpdateBuilder;
#[cfg(feature = "12-79-0")]
use crate::builder::GalleryPostBuilder;
#[cfg(feature = "12-79-2")]
use crate::builder::GalleryPostUpdateBuilder;
#[cfg(feature = "12-27-0")]
use crate::builder::NotificationBuilder;
#[cfg(not(feature = "12-93-0"))]
use crate::builder::ServerLogListBuilder;
#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
use crate::builder::UserListBuilder;
#[cfg(feature = "12-80-0")]
use crate::builder::{AdBuilder, AdUpdateBuilder};
use crate::builder::{
    AnnouncementUpdateBuilder, AntennaBuilder, AntennaUpdateBuilder, DriveFileBuilder,
    DriveFileListBuilder, DriveFileUpdateBuilder, DriveFileUrlBuilder, DriveFolderUpdateBuilder,
    MeUpdateBuilder, MessagingMessageBuilder, MetaUpdateBuilder, NoteBuilder, PageBuilder,
    PageUpdateBuilder,
};
#[cfg(feature = "12-47-0")]
use crate::builder::{ChannelBuilder, ChannelUpdateBuilder};
#[cfg(feature = "12-57-0")]
use crate::builder::{ClipBuilder, ClipUpdateBuilder};
use crate::pager::{BackwardPager, BoxPager, ForwardPager, OffsetPager, PagerStream};
use crate::Error;
use crate::{TimelineCursor, TimelineRange};

#[cfg(feature = "12-13-0")]
use chrono::DateTime;
use chrono::Utc;
use futures::{future::BoxFuture, stream::TryStreamExt};
use mime::Mime;
#[cfg(feature = "12-80-0")]
use misskey_api::model::ad::Ad;
#[cfg(feature = "12-47-0")]
use misskey_api::model::channel::Channel;
#[cfg(feature = "12-79-0")]
use misskey_api::model::gallery::GalleryPost;
#[cfg(feature = "12-109-0")]
use misskey_api::model::meta::AdminMeta;
#[cfg(feature = "12-67-0")]
use misskey_api::model::registry::{RegistryKey, RegistryScope, RegistryValue};
#[cfg(feature = "12-93-0")]
use misskey_api::model::user::UserOrigin;
use misskey_api::model::{
    abuse_user_report::AbuseUserReport,
    announcement::Announcement,
    antenna::Antenna,
    clip::Clip,
    drive::{DriveFile, DriveFolder},
    emoji::Emoji,
    following::FollowRequest,
    id::Id,
    log::ModerationLog,
    messaging::MessagingMessage,
    meta::Meta,
    note::{Note, Reaction, Tag},
    note_reaction::NoteReaction,
    notification::Notification,
    page::Page,
    query::Query,
    user::{User, UserRelation},
    user_group::{UserGroup, UserGroupInvitation},
    user_list::UserList,
};
use misskey_api::{endpoint, EntityRef};
use misskey_core::{Client, UploadFileClient};
use url::Url;

// {{{ Utility
macro_rules! impl_timeline_method {
    ($timeline:ident, $endpoint:path $(,$reqname:ident = $argname:ident : $argentity:ident)* ) => {
        paste::paste! {
            #[doc = "Lists the notes in the specified range of the " $timeline " timeline."]
            ///
            /// The bound `Into<TimelineRange<Note>>` on the argument type is satisfied by the type
            /// of some range expressions such as `..` or `start..` (which are desugared into [`RangeFull`][range_full] and
            /// [`RangeFrom`][range_from] respectively). A note or [`DateTime<Utc>`][datetime] can
            /// be used to specify the start and end bounds of the range.
            ///
            /// [range_full]: std::ops::RangeFull
            /// [range_from]: std::ops::RangeFrom
            /// [datetime]: chrono::DateTime
            ///
            /// # Examples
            ///
            /// ```
            /// # use misskey_util::ClientExt;
            /// # use futures::stream::TryStreamExt;
            /// # #[tokio::main]
            /// # async fn main() -> anyhow::Result<()> {
            /// # let client = misskey_test::test_client().await?;
            /// # let user = client.me().await?;
            /// # #[cfg(feature = "12-47-0")]
            /// # let channel = client.create_channel("test").await?;
            /// # let list = client.create_user_list("test").await?;
            /// # #[cfg(feature = "12-98-0")]
            /// # let antenna = client.create_antenna("antenna", "misskey").await?;
            /// use futures::stream::{StreamExt, TryStreamExt};
            ///
            #[doc = "// `notes` variable here is a `Stream` to enumerate first 100 " $timeline " notes."]
            #[doc = "let mut notes = client." $timeline "_notes(" $("&" $argname ", ")* "..).take(100);"]
            ///
            /// // Retrieve all notes until there are no more.
            /// while let Some(note) = notes.try_next().await? {
            ///     // Print the text of the note, if any.
            ///     if let Some(text) = note.text {
            ///         println!("@{}: {}", note.user.username, text);
            ///     }
            /// }
            /// # Ok(())
            /// # }
            /// ```
            ///
            /// ```
            /// # use misskey_util::ClientExt;
            /// # use futures::stream::TryStreamExt;
            /// # #[tokio::main]
            /// # async fn main() -> anyhow::Result<()> {
            /// # let client = misskey_test::test_client().await?;
            /// # let user = client.me().await?;
            /// # #[cfg(feature = "12-47-0")]
            /// # let channel = client.create_channel("test").await?;
            /// # let list = client.create_user_list("test").await?;
            /// # #[cfg(feature = "12-98-0")]
            /// # let antenna = client.create_antenna("antenna", "misskey").await?;
            /// use chrono::Utc;
            ///
            #[doc = "// Get the " $timeline " notes since `time`."]
            /// let time = Utc::today().and_hms(0, 0, 0);
            #[doc = "let mut notes = client." $timeline "_notes(" $("&" $argname ", ")* "time..);"]
            /// # Ok(())
            /// # }
            /// ```
            fn [<$timeline _notes>] (
                &self,
                $($argname : impl EntityRef<$argentity>,)*
                range: impl Into<TimelineRange<Note>>,
            ) -> PagerStream<BoxPager<Self, Note>> {
                $(
                let $reqname = $argname.entity_ref();
                )*
                let base_request =
                    endpoint::$endpoint::Request::builder()
                      $(.$reqname($reqname))*
                      .build();
                let pager = match range.into() {
                    TimelineRange::Id {
                        since_id,
                        until_id: None,
                    } => BackwardPager::with_since_id(
                        self,
                        since_id,
                        base_request
                    ),
                    TimelineRange::Id {
                        since_id,
                        until_id: Some(until_id),
                    } => BackwardPager::new(
                        self,
                        endpoint::$endpoint::Request {
                            since_id,
                            until_id: Some(until_id),
                            ..base_request
                        },
                    ),
                    TimelineRange::DateTime {
                        since_date,
                        until_date,
                    } => BackwardPager::new(
                        self,
                        endpoint::$endpoint::Request {
                            since_date,
                            until_date: Some(until_date.unwrap_or(Utc::now())),
                            ..base_request
                        },
                    ),
                    TimelineRange::Unbounded => {
                        BackwardPager::new(self, base_request)
                    }
                };
                PagerStream::new(Box::pin(pager))
            }

            #[doc = "Lists all notes since the specified point in the " $timeline " timeline in reverse order (i.e. the old note comes first, the new note comes after)."]
            ///
            /// # Examples
            ///
            /// ```
            /// # use misskey_util::ClientExt;
            /// # use futures::stream::TryStreamExt;
            /// # #[tokio::main]
            /// # async fn main() -> anyhow::Result<()> {
            /// # let client = misskey_test::test_client().await?;
            /// # let user = client.me().await?;
            /// # #[cfg(feature = "12-47-0")]
            /// # let channel = client.create_channel("test").await?;
            /// # let list = client.create_user_list("test").await?;
            /// # #[cfg(feature = "12-98-0")]
            /// # let antenna = client.create_antenna("antenna", "misskey").await?;
            /// use futures::stream::{StreamExt, TryStreamExt};
            /// use chrono::Utc;
            ///
            /// let time = Utc::today().and_hms(0, 0, 0);
            ///
            #[doc = "// `notes_since` is a `Stream` to enumerate first 100 " $timeline " notes since `time` in reverse order."]
            #[doc = "let mut notes_since = client." $timeline "_notes_since(" $("&" $argname ", ")* "time).take(100);"]
            ///
            /// // Retrieve all notes until there are no more.
            /// while let Some(note) = notes_since.try_next().await? {
            ///     // Print the text of the note, if any.
            ///     if let Some(text) = note.text {
            ///         println!("@{}: {}", note.user.username, text);
            ///     }
            /// }
            /// # Ok(())
            /// # }
            /// ```
            fn [<$timeline _notes_since>] (
                &self,
                $($argname : impl EntityRef<$argentity>,)*
                since: impl Into<TimelineCursor<Note>>,
            ) -> PagerStream<BoxPager<Self, Note>> {
                $(
                let $reqname = $argname.entity_ref();
                )*
                let base_request =
                    endpoint::$endpoint::Request::builder()
                      $(.$reqname($reqname))*
                      .build();
                let request = match since.into() {
                    TimelineCursor::DateTime(since_date) => endpoint::$endpoint::Request {
                        since_date: Some(since_date),
                        ..base_request
                    },
                    TimelineCursor::Id(since_id) => endpoint::$endpoint::Request {
                        since_id: Some(since_id),
                        ..base_request
                    },
                };
                let pager = ForwardPager::new(self, request);
                PagerStream::new(Box::pin(pager))
            }

            #[doc = "Returns a set of streams that fetch notes around the specified point in the " $timeline " timeline in both directions."]
            fn [<$timeline _notes_around>](
                &self,
                $($argname : impl EntityRef<$argentity>,)*
                cursor: impl Into<TimelineCursor<Note>>,
            ) -> (
                PagerStream<BoxPager<Self, Note>>,
                PagerStream<BoxPager<Self, Note>>,
            ) {
                let cursor = cursor.into();
                $(
                let $reqname = $argname.entity_ref();
                )*
                (
                    self.[<$timeline _notes_since>]($($reqname,)* cursor),
                    self.[<$timeline _notes>]($($reqname,)* TimelineRange::until(cursor)),
                )
            }
        }
    };
}
// }}}

/// An extension trait for [`Client`][client] that provides convenient high-level APIs.
///
/// [client]: misskey_core::Client
///
/// # Streams
///
/// Some methods (e.g. [`followers`][followers], [`local_notes`][local_notes], etc.) return a [`Stream`][stream]
/// that uses pagination to fetch all entries.
/// You can use methods from [`TryStreamExt`][try_stream_ext] or [`StreamExt`][stream_ext]
/// to work with this.
///
/// [followers]: ClientExt::followers
/// [local_notes]: ClientExt::local_notes
/// [stream]: futures::stream::Stream
/// [try_stream_ext]: futures::stream::TryStreamExt
/// [stream_ext]: futures::stream::StreamExt
pub trait ClientExt: Client + Sync {
    // {{{ User
    /// Gets the information of the user who is logged in with this client.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let me = client.me().await?;
    /// println!("Logged in as @{}", me.username);
    /// # Ok(())
    /// # }
    /// ```
    fn me(&self) -> BoxFuture<Result<User, Error<Self::Error>>> {
        Box::pin(async move {
            let user = self
                .request(endpoint::i::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(user)
        })
    }

    /// Updates the user logged in with this client.
    ///
    /// This method actually returns a builder, namely [`MeUpdateBuilder`].
    /// You can chain the method calls to it corresponding to the fields you want to update.
    /// Finally, calling [`update`][builder_update] method will actually perform the update.
    /// See [`MeUpdateBuilder`] for the fields that can be updated.
    ///
    /// [builder_update]: MeUpdateBuilder::update
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// // Flag it as a bot and set the name to "my awesome bot"
    /// let updated = client
    ///     .update_me()
    ///     .bot(true)
    ///     .set_name("my awesome bot")
    ///     .update()
    ///     .await?;
    ///
    /// assert!(updated.is_bot);
    /// assert_eq!(updated.name.unwrap(), "my awesome bot");
    /// # Ok(())
    /// # }
    /// ```
    fn update_me(&self) -> MeUpdateBuilder<&Self> {
        MeUpdateBuilder::new(self)
    }

    /// Follows the specified user.
    fn follow(&self, user: impl EntityRef<User>) -> BoxFuture<Result<User, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            let user = self
                .request(endpoint::following::create::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(user)
        })
    }

    /// Unfollows the specified user.
    fn unfollow(&self, user: impl EntityRef<User>) -> BoxFuture<Result<User, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            let user = self
                .request(endpoint::following::delete::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(user)
        })
    }

    #[cfg(feature = "12-98-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-98-0")))]
    /// Removes follow from the specified user.
    fn remove_follower(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<User, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            let user = self
                .request(endpoint::following::invalidate::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(user)
        })
    }

    /// Mutes the specified user.
    fn mute(&self, user: impl EntityRef<User>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::mute::create::Request {
                user_id,
                #[cfg(feature = "12-108-0")]
                expires_at: None,
            })
            .await
            .map_err(Error::Client)?
            .into_result()?;
            Ok(())
        })
    }

    /// Unmutes the specified user.
    fn unmute(&self, user: impl EntityRef<User>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::mute::delete::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Blocks the specified user.
    fn block(&self, user: impl EntityRef<User>) -> BoxFuture<Result<User, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            let user = self
                .request(endpoint::blocking::create::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(user)
        })
    }

    /// Unblocks the specified user.
    fn unblock(&self, user: impl EntityRef<User>) -> BoxFuture<Result<User, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            let user = self
                .request(endpoint::blocking::delete::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(user)
        })
    }

    /// Lists the followers of the specified user.
    ///
    /// # Examples
    ///
    /// This example uses [`TryStreamExt::try_next`][try_next] and [`while let`][while_let]
    /// to retrieve notifications one after another until there are no more.
    ///
    /// [try_next]: futures::stream::TryStreamExt::try_next
    /// [while_let]: https://doc.rust-lang.org/std/keyword.while.html
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// use futures::stream::{StreamExt, TryStreamExt};
    ///
    /// // In this example, we will fetch all the followers and follow them.
    /// // First, obtain your information to pass to `.follwers` method.
    /// let me = client.me().await?;
    ///
    /// // `follwers` variable here is a `Stream` to enumerate first 50 followers of `me`.
    /// let mut followers = client.followers(&me).take(50);
    ///
    /// // Retrieve all followers until there are no more.
    /// while let Some(user) = followers.try_next().await? {
    ///     // Follow the `user` if you haven't already.
    ///     if !client.is_following(&user).await? {
    ///         client.follow(&user).await?;
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn followers(&self, user: impl EntityRef<User>) -> PagerStream<BoxPager<Self, User>> {
        let pager = BackwardPager::new(
            self,
            endpoint::users::followers::RequestWithUserId::builder()
                .user_id(user.entity_ref())
                .build(),
        )
        .map_ok(|v| v.into_iter().map(|f| f.follower).collect());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the users that the specified user is following.
    fn following(&self, user: impl EntityRef<User>) -> PagerStream<BoxPager<Self, User>> {
        let pager = BackwardPager::new(
            self,
            endpoint::users::following::RequestWithUserId::builder()
                .user_id(user.entity_ref())
                .build(),
        )
        .map_ok(|v| v.into_iter().map(|f| f.followee).collect());
        PagerStream::new(Box::pin(pager))
    }

    /// Pins the specified note to the profile.
    fn pin_note(&self, note: impl EntityRef<Note>) -> BoxFuture<Result<User, Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            let user = self
                .request(endpoint::i::pin::Request { note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(user)
        })
    }

    /// Unpins the specified note to the profile.
    fn unpin_note(
        &self,
        note: impl EntityRef<Note>,
    ) -> BoxFuture<Result<User, Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            let user = self
                .request(endpoint::i::unpin::Request { note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(user)
        })
    }

    /// Lists the follow requests sent to the user logged in with this client.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// // Accept all follow requests
    /// for request in client.follow_requests().await? {
    ///     client.accept_follow_request(&request.follower).await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn follow_requests(&self) -> BoxFuture<Result<Vec<FollowRequest>, Error<Self::Error>>> {
        Box::pin(async move {
            let requests = self
                .request(endpoint::following::requests::list::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(requests)
        })
    }

    /// Cancels the follow request being sent to the specified user.
    fn cancel_follow_request(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<User, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            let user = self
                .request(endpoint::following::requests::cancel::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(user)
        })
    }

    /// Accepts the follow request that have been received from the specified user.
    fn accept_follow_request(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::following::requests::accept::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Rejects the follow request that have been received from the specified user.
    fn reject_follow_request(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::following::requests::reject::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Lists the users muted by the user logged in with this client.
    fn muting_users(&self) -> PagerStream<BoxPager<Self, User>> {
        let pager = BackwardPager::new(self, endpoint::mute::list::Request::default())
            .map_ok(|v| v.into_iter().map(|m| m.mutee).collect());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the users blocked by the user logged in with this client.
    fn blocking_users(&self) -> PagerStream<BoxPager<Self, User>> {
        let pager = BackwardPager::new(self, endpoint::blocking::list::Request::default())
            .map_ok(|v| v.into_iter().map(|b| b.blockee).collect());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the notes favorited by the user logged in with this client.
    fn favorited_notes(&self) -> PagerStream<BoxPager<Self, Note>> {
        let pager = BackwardPager::new(self, endpoint::i::favorites::Request::default())
            .map_ok(|v| v.into_iter().map(|f| f.note).collect());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the notifications to the user logged in with this client.
    ///
    /// # Examples
    ///
    /// This example uses [`TryStreamExt::try_next`][try_next] and [`while let`][while_let]
    /// to retrieve notifications one after another until there are no more.
    ///
    /// [try_next]: futures::stream::TryStreamExt::try_next
    /// [while_let]: https://doc.rust-lang.org/std/keyword.while.html
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// use futures::stream::{StreamExt, TryStreamExt};
    ///
    /// // `notifications` here is a `Stream` to enumerate first 10 notifications.
    /// let mut notifications = client.notifications().take(10);
    /// // Retrieve notifications until there are no more.
    /// while let Some(notification) = notifications.try_next().await? {
    ///     // Print some information about the notification.
    ///     println!("notification {}: created at {}", notification.id, notification.created_at);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn notifications(&self) -> PagerStream<BoxPager<Self, Notification>> {
        let pager = BackwardPager::new(self, endpoint::i::notifications::Request::default());
        PagerStream::new(Box::pin(pager))
    }

    /// Returns the relationship between the specified user and the user logged in with this
    /// client.
    ///
    /// The returned [`UserRelation`] object records the various information.
    /// There are separate methods to examine each relationship (e.g.
    /// [`is_following`][is_following]), so if you are only interested in one relationship,
    /// it can be simpler to use those.
    ///
    /// [is_following]: ClientExt::is_following
    fn user_relation(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<UserRelation, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            let relation = self
                .request(endpoint::users::relation::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(relation)
        })
    }

    /// Checks if the specified user is followed by the user logged in with this client.
    ///
    /// If you are also interested in other relationships, use [`user_relation`][user_relation].
    ///
    /// [user_relation]: ClientExt::user_relation
    ///
    #[cfg_attr(any(not(feature = "12-88-0"), feature = "12-89-0"), doc = "```")]
    #[cfg_attr(all(feature = "12-88-0", not(feature = "12-89-0")), doc = "```ignore")]
    /// # use misskey_util::ClientExt;
    /// # use futures::stream::TryStreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # let user = client.users().list().try_next().await?.unwrap();
    /// let relation = client.user_relation(&user).await?;
    /// assert_eq!(client.is_following(&user).await?, relation.is_following);
    /// # Ok(())
    /// # }
    /// ```
    fn is_following(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<bool, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move { Ok(self.user_relation(user_id).await?.is_following) })
    }

    /// Checks if the specified user follows the user logged in with this client.
    ///
    /// If you are also interested in other relationships, use [`user_relation`][user_relation].
    ///
    /// [user_relation]: ClientExt::user_relation
    ///
    #[cfg_attr(any(not(feature = "12-88-0"), feature = "12-89-0"), doc = "```")]
    #[cfg_attr(all(feature = "12-88-0", not(feature = "12-89-0")), doc = "```ignore")]
    /// # use misskey_util::ClientExt;
    /// # use futures::stream::TryStreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # let user = client.users().list().try_next().await?.unwrap();
    /// let relation = client.user_relation(&user).await?;
    /// assert_eq!(client.is_followed(&user).await?, relation.is_followed);
    /// # Ok(())
    /// # }
    /// ```
    fn is_followed(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<bool, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move { Ok(self.user_relation(user_id).await?.is_followed) })
    }

    /// Checks if the specified user is blocked by the user logged in with this client.
    ///
    /// If you are also interested in other relationships, use [`user_relation`][user_relation].
    ///
    /// [user_relation]: ClientExt::user_relation
    ///
    #[cfg_attr(any(not(feature = "12-88-0"), feature = "12-89-0"), doc = "```")]
    #[cfg_attr(all(feature = "12-88-0", not(feature = "12-89-0")), doc = "```ignore")]
    /// # use misskey_util::ClientExt;
    /// # use futures::stream::TryStreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # let user = client.users().list().try_next().await?.unwrap();
    /// let relation = client.user_relation(&user).await?;
    /// assert_eq!(client.is_blocking(&user).await?, relation.is_blocking);
    /// # Ok(())
    /// # }
    /// ```
    fn is_blocking(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<bool, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move { Ok(self.user_relation(user_id).await?.is_blocking) })
    }

    /// Checks if the specified user blocks the user logged in with this client.
    ///
    /// If you are also interested in other relationships, use [`user_relation`][user_relation].
    ///
    /// [user_relation]: ClientExt::user_relation
    ///
    #[cfg_attr(any(not(feature = "12-88-0"), feature = "12-89-0"), doc = "```")]
    #[cfg_attr(all(feature = "12-88-0", not(feature = "12-89-0")), doc = "```ignore")]
    /// # use misskey_util::ClientExt;
    /// # use futures::stream::TryStreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # let user = client.users().list().try_next().await?.unwrap();
    /// let relation = client.user_relation(&user).await?;
    /// assert_eq!(client.is_blocked(&user).await?, relation.is_blocked);
    /// # Ok(())
    /// # }
    /// ```
    fn is_blocked(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<bool, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move { Ok(self.user_relation(user_id).await?.is_blocked) })
    }

    /// Checks if the specified user is muted by the user logged in with this client.
    ///
    /// If you are also interested in other relationships, use [`user_relation`][user_relation].
    ///
    /// [user_relation]: ClientExt::user_relation
    ///
    #[cfg_attr(any(not(feature = "12-88-0"), feature = "12-89-0"), doc = "```")]
    #[cfg_attr(all(feature = "12-88-0", not(feature = "12-89-0")), doc = "```ignore")]
    /// # use misskey_util::ClientExt;
    /// # use futures::stream::TryStreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # let user = client.users().list().try_next().await?.unwrap();
    /// let relation = client.user_relation(&user).await?;
    /// assert_eq!(client.is_muted(&user).await?, relation.is_muted);
    /// # Ok(())
    /// # }
    /// ```
    fn is_muted(&self, user: impl EntityRef<User>) -> BoxFuture<Result<bool, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move { Ok(self.user_relation(user_id).await?.is_muted) })
    }

    /// Checks if the specified user has a pending follow request from the user logged in with this client.
    ///
    /// If you are also interested in other relationships, use [`user_relation`][user_relation].
    ///
    /// [user_relation]: ClientExt::user_relation
    ///
    #[cfg_attr(any(not(feature = "12-88-0"), feature = "12-89-0"), doc = "```")]
    #[cfg_attr(all(feature = "12-88-0", not(feature = "12-89-0")), doc = "```ignore")]
    /// # use misskey_util::ClientExt;
    /// # use futures::stream::TryStreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # let user = client.users().list().try_next().await?.unwrap();
    /// let relation = client.user_relation(&user).await?;
    /// assert_eq!(client.has_pending_follow_request_from_me(&user).await?, relation.has_pending_follow_request_from_you);
    /// # Ok(())
    /// # }
    /// ```
    fn has_pending_follow_request_from_me(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<bool, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            Ok(self
                .user_relation(user_id)
                .await?
                .has_pending_follow_request_from_you)
        })
    }

    /// Checks if the specified user has a pending follow request to the user logged in with this client.
    ///
    /// If you are also interested in other relationships, use [`user_relation`][user_relation].
    ///
    /// [user_relation]: ClientExt::user_relation
    ///
    #[cfg_attr(any(not(feature = "12-88-0"), feature = "12-89-0"), doc = "```")]
    #[cfg_attr(all(feature = "12-88-0", not(feature = "12-89-0")), doc = "```ignore")]
    /// # use misskey_util::ClientExt;
    /// # use futures::stream::TryStreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # let user = client.users().list().try_next().await?.unwrap();
    /// let relation = client.user_relation(&user).await?;
    /// assert_eq!(client.has_pending_follow_request_to_me(&user).await?, relation.has_pending_follow_request_to_you);
    /// # Ok(())
    /// # }
    /// ```
    fn has_pending_follow_request_to_me(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<bool, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            Ok(self
                .user_relation(user_id)
                .await?
                .has_pending_follow_request_to_you)
        })
    }

    /// Gets the corresponding user from the ID.
    fn get_user(&self, id: Id<User>) -> BoxFuture<Result<User, Error<Self::Error>>> {
        Box::pin(async move {
            let note = self
                .request(endpoint::users::show::Request::WithUserId { user_id: id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(note)
        })
    }

    /// Gets the list of corresponding user from the list of IDs.
    fn get_users(
        &self,
        ids: impl IntoIterator<Item = Id<User>>,
    ) -> BoxFuture<Result<Vec<User>, Error<Self::Error>>> {
        let user_ids = ids.into_iter().collect();
        Box::pin(async move {
            let note = self
                .request(endpoint::users::show::RequestWithUserIds { user_ids })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(note)
        })
    }

    /// Reports abuse by the specified user.
    fn report_abuse(
        &self,
        user: impl EntityRef<User>,
        comment: impl Into<String>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let user_id = user.entity_ref();
        let comment = comment.into();
        Box::pin(async move {
            self.request(endpoint::users::report_abuse::Request { user_id, comment })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Searches for users with the specified query string.
    fn search_users(&self, query: impl Into<String>) -> PagerStream<BoxPager<Self, User>> {
        let pager = OffsetPager::new(
            self,
            endpoint::users::search::Request::builder()
                .query(query)
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Searches for users in the instance with the specified query string.
    #[cfg(feature = "12-93-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-93-0")))]
    fn search_local_users(&self, query: impl Into<String>) -> PagerStream<BoxPager<Self, User>> {
        let pager = OffsetPager::new(
            self,
            endpoint::users::search::Request::builder()
                .query(query)
                .origin(UserOrigin::Local)
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Searches for users in remote instances with the specified query string.
    #[cfg(feature = "12-93-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-93-0")))]
    fn search_remote_users(&self, query: impl Into<String>) -> PagerStream<BoxPager<Self, User>> {
        let pager = OffsetPager::new(
            self,
            endpoint::users::search::Request::builder()
                .query(query)
                .origin(UserOrigin::Remote)
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the users in the instance.
    ///
    /// This method actually returns a builder, namely [`UserListBuilder`].
    /// You can specify how you want to list users by chaining methods.
    /// The [`list`][builder_list] method of the builder returns a [`Stream`][stream]
    /// that lists users in the specified way.
    ///
    /// [builder_list]: UserListBuilder::list
    /// [stream]: futures::stream::Stream
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # use misskey_api as misskey;
    /// use futures::stream::TryStreamExt;
    /// use misskey::model::user::{User, UserSortKey};
    ///
    /// // Get a list of local moderator users sorted by number of followers.
    /// let users: Vec<User> = client
    ///     .users()
    ///     .local()
    ///     .moderator()
    ///     .sort_by_followers()
    ///     .list()
    ///     .try_collect()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    // misskey-dev/misskey#7656
    #[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
    #[cfg_attr(docsrs, doc(cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))))]
    fn users(&self) -> UserListBuilder<&Self> {
        UserListBuilder::new(self)
    }

    /// Lists the recommended users of the instance.
    // misskey-dev/misskey#7656
    #[cfg(not(feature = "12-88-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-88-0"))))]
    fn recommended_users(&self) -> PagerStream<BoxPager<Self, User>> {
        let pager = OffsetPager::new(self, endpoint::users::recommendation::Request::default());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the users who frequently reply to the specified user.
    fn frequently_replied_users(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<Vec<User>, Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            let users = self
                .request(endpoint::users::get_frequently_replied_users::Request {
                    user_id,
                    limit: None,
                })
                .await
                .map_err(Error::Client)?
                .into_result()?
                .into_iter()
                .map(|endpoint::users::get_frequently_replied_users::Reply { user, .. }| user)
                .collect();
            Ok(users)
        })
    }

    /// Lists the users pinned to the instance.
    fn pinned_users(&self) -> BoxFuture<Result<Vec<User>, Error<Self::Error>>> {
        Box::pin(async move {
            let users = self
                .request(endpoint::pinned_users::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(users)
        })
    }

    // }}}

    // {{{ Note
    /// Returns a builder for composing a note.
    ///
    /// The returned builder provides methods to customize details of the note,
    /// and you can chain them to compose a note incrementally.
    /// Finally, calling [`create`][builder_create] method will actually create a note.
    /// See [`NoteBuilder`] for the provided methods.
    ///
    /// [builder_create]: NoteBuilder::create
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let note = client
    ///     .build_note()
    ///     .text("Hello, World")
    ///     .followers_only()
    ///     .create()
    ///     .await?;
    ///
    /// assert_eq!(note.text.unwrap(), "Hello, World");
    /// # Ok(())
    /// # }
    /// ```
    fn build_note(&self) -> NoteBuilder<&Self> {
        NoteBuilder::new(self)
    }

    /// Deletes the specified note.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let note = client.create_note("Oops!").await?;
    /// client.delete_note(&note).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn delete_note(&self, note: impl EntityRef<Note>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            self.request(endpoint::notes::delete::Request { note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Gets the corresponding note from the ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let note_id = "2f2vxw5kzh";
    /// let note = client.get_note(note_id.parse()?).await?;
    /// println!("@{}: {}", note.user.username, note.text.unwrap_or_default());
    /// # Ok(())
    /// # }
    /// ```
    fn get_note(&self, id: Id<Note>) -> BoxFuture<Result<Note, Error<Self::Error>>> {
        Box::pin(async move {
            let note = self
                .request(endpoint::notes::show::Request { note_id: id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(note)
        })
    }

    /// Creates a note with the given text.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let note = client.create_note("Hello, Misskey!").await?;
    /// assert_eq!(note.text.unwrap(), "Hello, Misskey!");
    /// # Ok(())
    /// # }
    /// ```
    fn create_note(&self, text: impl Into<String>) -> BoxFuture<Result<Note, Error<Self::Error>>> {
        let text = text.into();
        Box::pin(async move { self.build_note().text(text).create().await })
    }

    /// Creates a poll with the given text and choices.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let note = client
    ///     .poll("Which fruit is your favorite?", vec!["Apple", "Orange", "Banana"])
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::needless_collect)]
    fn poll(
        &self,
        text: impl Into<String>,
        choices: impl IntoIterator<Item = impl Into<String>>,
    ) -> BoxFuture<Result<Note, Error<Self::Error>>> {
        let text = text.into();
        let choices: Vec<_> = choices.into_iter().map(Into::into).collect();
        Box::pin(async move { self.build_note().text(text).poll(choices).create().await })
    }

    /// Creates a reply note with the given text.
    fn reply(
        &self,
        note: impl EntityRef<Note>,
        text: impl Into<String>,
    ) -> BoxFuture<Result<Note, Error<Self::Error>>> {
        let note_id = note.entity_ref();
        let text = text.into();
        Box::pin(async move { self.build_note().reply(note_id).text(text).create().await })
    }

    /// Creates a renote.
    fn renote(&self, note: impl EntityRef<Note>) -> BoxFuture<Result<Note, Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move { self.build_note().renote(note_id).create().await })
    }

    /// Creates a quote note with the given text.
    fn quote(
        &self,
        note: impl EntityRef<Note>,
        text: impl Into<String>,
    ) -> BoxFuture<Result<Note, Error<Self::Error>>> {
        let note_id = note.entity_ref();
        let text = text.into();
        Box::pin(async move { self.build_note().renote(note_id).text(text).create().await })
    }

    /// Adds the reaction to the specified note.
    fn react(
        &self,
        note: impl EntityRef<Note>,
        reaction: impl Into<Reaction>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let note_id = note.entity_ref();
        let reaction = reaction.into();
        Box::pin(async move {
            self.request(endpoint::notes::reactions::create::Request { note_id, reaction })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Deletes a reaction from the specified note.
    fn unreact(&self, note: impl EntityRef<Note>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            self.request(endpoint::notes::reactions::delete::Request { note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Favorites the specified note.
    fn favorite(&self, note: impl EntityRef<Note>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            self.request(endpoint::notes::favorites::create::Request { note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Unfavorites the specified note.
    fn unfavorite(&self, note: impl EntityRef<Note>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            self.request(endpoint::notes::favorites::delete::Request { note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Watches the specified note.
    fn watch(&self, note: impl EntityRef<Note>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            self.request(endpoint::notes::watching::create::Request { note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Unwatches the specified note.
    fn unwatch(&self, note: impl EntityRef<Note>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            self.request(endpoint::notes::watching::delete::Request { note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Mutes notifications from threads where the specified note belongs to.
    #[cfg(feature = "12-95-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-95-0")))]
    fn mute_thread(&self, note: impl EntityRef<Note>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            self.request(endpoint::notes::thread_muting::create::Request { note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Unmutes notifications from threads where the specified note belongs to.
    #[cfg(feature = "12-95-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-95-0")))]
    fn unmute_thread(
        &self,
        note: impl EntityRef<Note>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            self.request(endpoint::notes::thread_muting::delete::Request { note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Checks if the specified note is favorited by the user logged in with this client.
    fn is_favorited(
        &self,
        note: impl EntityRef<Note>,
    ) -> BoxFuture<Result<bool, Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            let state = self
                .request(endpoint::notes::state::Request { note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(state.is_favorited)
        })
    }

    /// Checks if the specified note is watched by the user logged in with this client.
    fn is_watched(
        &self,
        note: impl EntityRef<Note>,
    ) -> BoxFuture<Result<bool, Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            let state = self
                .request(endpoint::notes::state::Request { note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(state.is_watching)
        })
    }

    /// Vote on the specified note.
    fn vote(
        &self,
        note: impl EntityRef<Note>,
        choice: u64,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            self.request(endpoint::notes::polls::vote::Request { note_id, choice })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Lists the featured notes.
    fn featured_notes(&self) -> PagerStream<BoxPager<Self, Note>> {
        let pager = OffsetPager::new(self, endpoint::notes::featured::Request::default());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the notes of the conversation.
    fn conversation(&self, note: impl EntityRef<Note>) -> PagerStream<BoxPager<Self, Note>> {
        let pager = OffsetPager::new(
            self,
            endpoint::notes::conversation::Request::builder()
                .note_id(note.entity_ref())
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the reply notes to the specified note.
    fn children_notes(&self, note: impl EntityRef<Note>) -> PagerStream<BoxPager<Self, Note>> {
        let pager = BackwardPager::new(
            self,
            endpoint::notes::children::Request::builder()
                .note_id(note.entity_ref())
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the notes that are mentioning the account you are logged into with this client.
    fn mentioned_notes(&self) -> PagerStream<BoxPager<Self, Note>> {
        let pager = BackwardPager::new(self, endpoint::notes::mentions::Request::default());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the renotes of the specified note.
    fn renotes(&self, note: impl EntityRef<Note>) -> PagerStream<BoxPager<Self, Note>> {
        let pager = BackwardPager::new(
            self,
            endpoint::notes::renotes::Request::builder()
                .note_id(note.entity_ref())
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the replies to the specified note.
    fn replies(&self, note: impl EntityRef<Note>) -> PagerStream<BoxPager<Self, Note>> {
        let pager = BackwardPager::new(
            self,
            endpoint::notes::renotes::Request::builder()
                .note_id(note.entity_ref())
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Searches for notes with the specified query string.
    fn search_notes(&self, query: impl Into<String>) -> PagerStream<BoxPager<Self, Note>> {
        let pager = BackwardPager::new(
            self,
            endpoint::notes::search::Request::builder()
                .query(query)
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    impl_timeline_method! { local, notes::local_timeline }
    impl_timeline_method! { global, notes::global_timeline }
    impl_timeline_method! { social, notes::hybrid_timeline }
    impl_timeline_method! { home, notes::timeline }
    impl_timeline_method! { user, users::notes, user_id = user : User }
    impl_timeline_method! { user_list, notes::user_list_timeline, list_id = list : UserList }

    #[cfg(feature = "12-47-0")]
    impl_timeline_method! { channel, channels::timeline, channel_id = channel : Channel }

    #[cfg(feature = "12-98-0")]
    impl_timeline_method! { antenna, antennas::notes, antenna_id = antenna : Antenna }

    /// Lists the notes with tags as specified in the given query.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// // Get all notes with the "linux" tag.
    /// let mut notes = client.tagged_notes("linux");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # use misskey_api as misskey;
    /// use misskey::model::query::Query;
    ///
    /// // Get all notes tagged with "test" or "bot".
    /// let mut notes = client.tagged_notes(Query::atom("test").or("bot"));
    /// # Ok(())
    /// # }
    /// ```
    fn tagged_notes(&self, query: impl Into<Query<Tag>>) -> PagerStream<BoxPager<Self, Note>> {
        let pager = BackwardPager::new(
            self,
            endpoint::notes::search_by_tag::Request::builder()
                .query(query)
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the local notes with the given file types.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// use mime::IMAGE_STAR;
    ///
    /// // Get all local notes with image files.
    /// let mut notes = client.local_notes_with_file_types(vec![IMAGE_STAR]);
    /// # Ok(())
    /// # }
    /// ```
    fn local_notes_with_file_types(
        &self,
        types: impl IntoIterator<Item = Mime>,
    ) -> PagerStream<BoxPager<Self, Note>> {
        let pager = BackwardPager::new(
            self,
            endpoint::notes::local_timeline::Request::builder()
                .file_type(types.into_iter().map(Into::into).collect())
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }
    // }}}

    // {{{ User List
    /// Creates a user list with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let list = client.create_user_list("list").await?;
    /// assert_eq!(list.name, "list");
    /// # Ok(())
    /// # }
    /// ```
    fn create_user_list(
        &self,
        name: impl Into<String>,
    ) -> BoxFuture<Result<UserList, Error<Self::Error>>> {
        let name = name.into();
        Box::pin(async move {
            let list = self
                .request(endpoint::users::lists::create::Request { name })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(list)
        })
    }

    /// Deletes the specified user list.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let list = client.create_user_list("list").await?;
    /// client.delete_user_list(&list).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn delete_user_list(
        &self,
        list: impl EntityRef<UserList>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let list_id = list.entity_ref();
        Box::pin(async move {
            self.request(endpoint::users::lists::delete::Request { list_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Updates the name of the specified user list to the given one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let list = client.create_user_list("list").await?;
    /// let renamed_list = client.rename_user_list(&list, "list2").await?;
    /// assert_eq!(renamed_list.name, "list2");
    /// # Ok(())
    /// # }
    /// ```
    fn rename_user_list(
        &self,
        list: impl EntityRef<UserList>,
        name: impl Into<String>,
    ) -> BoxFuture<Result<UserList, Error<Self::Error>>> {
        let list_id = list.entity_ref();
        let name = name.into();
        Box::pin(async move {
            let list = self
                .request(endpoint::users::lists::update::Request { list_id, name })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(list)
        })
    }

    /// Gets the corresponding user list from the ID.
    fn get_user_list(&self, id: Id<UserList>) -> BoxFuture<Result<UserList, Error<Self::Error>>> {
        Box::pin(async move {
            let list = self
                .request(endpoint::users::lists::show::Request { list_id: id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(list)
        })
    }

    /// Adds the user from the specified user list.
    fn push_to_user_list(
        &self,
        list: impl EntityRef<UserList>,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let list_id = list.entity_ref();
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::users::lists::push::Request { list_id, user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Deletes the user from the specified user list.
    fn pull_from_user_list(
        &self,
        list: impl EntityRef<UserList>,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let list_id = list.entity_ref();
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::users::lists::pull::Request { list_id, user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }
    // }}}

    // {{{ User Group
    /// Creates a user group with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let group = client.create_user_group("group").await?;
    /// assert_eq!(group.name, "group");
    /// # Ok(())
    /// # }
    /// ```
    fn create_user_group(
        &self,
        name: impl Into<String>,
    ) -> BoxFuture<Result<UserGroup, Error<Self::Error>>> {
        let name = name.into();
        Box::pin(async move {
            let group = self
                .request(endpoint::users::groups::create::Request { name })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(group)
        })
    }

    /// Deletes the specified user group.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let group = client.create_user_group("group").await?;
    /// client.delete_user_group(&group).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn delete_user_group(
        &self,
        group: impl EntityRef<UserGroup>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let group_id = group.entity_ref();
        Box::pin(async move {
            self.request(endpoint::users::groups::delete::Request { group_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Updates the name of the specified user group to the given one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let group = client.create_user_group("group").await?;
    /// let renamed_group = client.rename_user_group(&group, "group2").await?;
    /// assert_eq!(renamed_group.name, "group2");
    /// # Ok(())
    /// # }
    /// ```
    fn rename_user_group(
        &self,
        group: impl EntityRef<UserGroup>,
        name: impl Into<String>,
    ) -> BoxFuture<Result<UserGroup, Error<Self::Error>>> {
        let group_id = group.entity_ref();
        let name = name.into();
        Box::pin(async move {
            let group = self
                .request(endpoint::users::groups::update::Request { group_id, name })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(group)
        })
    }

    /// Gets the corresponding user group from the ID.
    fn get_user_group(
        &self,
        id: Id<UserGroup>,
    ) -> BoxFuture<Result<UserGroup, Error<Self::Error>>> {
        Box::pin(async move {
            let group = self
                .request(endpoint::users::groups::show::Request { group_id: id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(group)
        })
    }

    /// Invites the user to the specified user group.
    fn invite_to_user_group(
        &self,
        group: impl EntityRef<UserGroup>,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let group_id = group.entity_ref();
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::users::groups::invite::Request { group_id, user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Deletes the user from the specified user group.
    ///
    /// Note that the owner of the group cannot be deleted.
    /// If you want to do so, you first need to transfer the group with
    /// [`transfer_user_group`][transfer].
    ///
    /// [transfer]: ClientExt::transfer_user_group
    fn pull_from_user_group(
        &self,
        group: impl EntityRef<UserGroup>,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let group_id = group.entity_ref();
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::users::groups::pull::Request { group_id, user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Leaves the specified user group.
    ///
    /// Note that the owner cannot leave the group.
    #[cfg(feature = "12-92-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-92-0")))]
    fn leave_group(
        &self,
        group: impl EntityRef<UserGroup>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let group_id = group.entity_ref();
        Box::pin(async move {
            self.request(endpoint::users::groups::leave::Request { group_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Transfers the specified user group.
    ///
    /// Note that you can only transfer the group to one of its members.
    fn transfer_user_group(
        &self,
        group: impl EntityRef<UserGroup>,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<UserGroup, Error<Self::Error>>> {
        let group_id = group.entity_ref();
        let user_id = user.entity_ref();
        Box::pin(async move {
            let group = self
                .request(endpoint::users::groups::transfer::Request { group_id, user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(group)
        })
    }

    /// Lists the user group invitations sent to the user who is logged in with this client.
    ///
    /// # Examples
    ///
    /// This example uses [`TryStreamExt::try_next`][try_next] and [`while let`][while_let]
    /// to retrieve invitations one after another until there are no more.
    ///
    /// [try_next]: futures::stream::TryStreamExt::try_next
    /// [while_let]: https://doc.rust-lang.org/std/keyword.while.html
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// use futures::stream::TryStreamExt;
    ///
    /// // `invitations` here is a `Stream` to enumerate all the invitations.
    /// let mut invitations = client.user_group_invitations();
    /// // Retrieve invitations until there are no more.
    /// while let Some(invitation) = invitations.try_next().await? {
    ///     // Accept the invitation.
    ///     client.accept_user_group_invitation(&invitation).await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn user_group_invitations(&self) -> PagerStream<BoxPager<Self, UserGroupInvitation>> {
        let pager = BackwardPager::new(self, endpoint::i::user_group_invites::Request::default());
        PagerStream::new(Box::pin(pager))
    }

    /// Accepts the specified user group invitation sent to the user logged in with this client.
    fn accept_user_group_invitation(
        &self,
        invitation: impl EntityRef<UserGroupInvitation>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let invitation_id = invitation.entity_ref();
        Box::pin(async move {
            self.request(endpoint::users::groups::invitations::accept::Request { invitation_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Rejects the specified user group invitation sent to the user logged in with this client.
    fn reject_user_group_invitation(
        &self,
        invitation: impl EntityRef<UserGroupInvitation>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let invitation_id = invitation.entity_ref();
        Box::pin(async move {
            self.request(endpoint::users::groups::invitations::reject::Request { invitation_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Lists the user groups joined by the user logged in with this client.
    fn joined_user_groups(&self) -> BoxFuture<Result<Vec<UserGroup>, Error<Self::Error>>> {
        Box::pin(async move {
            let groups = self
                .request(endpoint::users::groups::joined::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(groups)
        })
    }

    /// Lists the user groups owned by the user logged in with this client.
    fn owned_user_groups(&self) -> BoxFuture<Result<Vec<UserGroup>, Error<Self::Error>>> {
        Box::pin(async move {
            let groups = self
                .request(endpoint::users::groups::owned::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(groups)
        })
    }
    // }}}

    // {{{ Antenna
    /// Creates an antenna with the given name and query.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # use misskey_api as misskey;
    /// use misskey::model::query::Query;
    ///
    /// // Create an antenna for notes containing "misskey" or "msky"
    /// let antenna = client
    ///     .create_antenna("misskey antenna", Query::atom("misskey").or("msky"))
    ///     .await?;
    ///
    /// assert_eq!(antenna.name, "misskey antenna");
    /// # Ok(())
    /// # }
    /// ```
    fn create_antenna(
        &self,
        name: impl Into<String>,
        query: impl Into<Query<String>>,
    ) -> BoxFuture<Result<Antenna, Error<Self::Error>>> {
        let name = name.into();
        let query = query.into();
        Box::pin(async move {
            self.build_antenna()
                .name(name)
                .include(query)
                .create()
                .await
        })
    }

    /// Returns a builder for creating an antenna.
    ///
    /// The returned builder provides methods to customize details of the antenna,
    /// and you can chain them to create an antenna incrementally.
    /// Finally, calling [`create`][builder_create] method will actually create an antenna.
    /// See [`AntennaBuilder`] for the provided methods.
    ///
    /// [builder_create]: AntennaBuilder::create
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// // Create an antenna for non-reply notes in home timeline that include "misskey"
    /// let antenna = client
    ///     .build_antenna()
    ///     .name("misskey antenna")
    ///     .include("misskey")
    ///     .home()
    ///     .exclude_replies(true)
    ///     .create()
    ///     .await?;
    ///
    /// assert_eq!(antenna.name, "misskey antenna");
    /// # Ok(())
    /// # }
    /// ```
    fn build_antenna(&self) -> AntennaBuilder<&Self> {
        AntennaBuilder::new(self)
    }

    /// Deletes the specified antenna.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let antenna = client
    ///     .create_antenna("antenna", "misskey")
    ///     .await?;
    /// client.delete_antenna(&antenna).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn delete_antenna(
        &self,
        antenna: impl EntityRef<Antenna>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let antenna_id = antenna.entity_ref();
        Box::pin(async move {
            self.request(endpoint::antennas::delete::Request { antenna_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Gets the corresponding antenna from the ID.
    fn get_antenna(&self, id: Id<Antenna>) -> BoxFuture<Result<Antenna, Error<Self::Error>>> {
        Box::pin(async move {
            let antenna = self
                .request(endpoint::antennas::show::Request { antenna_id: id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(antenna)
        })
    }

    /// Updates the antenna.
    ///
    /// This method actually returns a builder, namely [`AntennaUpdateBuilder`].
    /// You can chain the method calls to it corresponding to the fields you want to update.
    /// Finally, calling [`update`][builder_update] method will actually perform the update.
    /// See [`AntennaUpdateBuilder`] for the fields that can be updated.
    ///
    /// [builder_update]: AntennaUpdateBuilder::update
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let antenna = client
    ///     .create_antenna("antenna", "misskey")
    ///     .await?;
    ///
    /// // Change source and case sensitivity of the antenna
    /// client
    ///     .update_antenna(antenna)
    ///     .case_sensitive(true)
    ///     .all()
    ///     .update()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    fn update_antenna(&self, antenna: Antenna) -> AntennaUpdateBuilder<&Self> {
        AntennaUpdateBuilder::new(self, antenna)
    }

    /// Lists the antennas created by the user logged in with this client.
    fn antennas(&self) -> BoxFuture<Result<Vec<Antenna>, Error<Self::Error>>> {
        Box::pin(async move {
            let antennas = self
                .request(endpoint::antennas::list::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(antennas)
        })
    }

    /// Lists the notes that hit the specified antenna.
    #[cfg(not(feature = "12-98-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-98-0"))))]
    fn antenna_notes(&self, antenna: impl EntityRef<Antenna>) -> PagerStream<BoxPager<Self, Note>> {
        let pager = BackwardPager::new(
            self,
            endpoint::antennas::notes::Request::builder()
                .antenna_id(antenna.entity_ref())
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }
    // }}}

    // {{{ Channel
    /// Creates a channel with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let channel = client.create_channel("name").await?;
    /// assert_eq!(channel.name, "name");
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    fn create_channel(
        &self,
        name: impl Into<String>,
    ) -> BoxFuture<Result<Channel, Error<Self::Error>>> {
        let name = name.into();
        Box::pin(async move { self.build_channel().name(name).create().await })
    }

    /// Returns a builder for creating a channel.
    ///
    /// The returned builder provides methods to customize details of the channel,
    /// and you can chain them to create a channel incrementally.
    /// Finally, calling [`create`][builder_create] method will actually create a channel.
    /// See [`ChannelBuilder`] for the provided methods.
    ///
    /// [builder_create]: ChannelBuilder::create
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let channel = client
    ///     .build_channel()
    ///     .name("bot devs")
    ///     .description("Let's talk about Misskey's bot development!")
    ///     .create()
    ///     .await?;
    ///
    /// assert_eq!(channel.name, "bot devs");
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    fn build_channel(&self) -> ChannelBuilder<&Self> {
        ChannelBuilder::new(self)
    }

    /// Gets the corresponding channel from the ID.
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    fn get_channel(&self, id: Id<Channel>) -> BoxFuture<Result<Channel, Error<Self::Error>>> {
        Box::pin(async move {
            let channel = self
                .request(endpoint::channels::show::Request { channel_id: id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(channel)
        })
    }

    /// Updates the specified channel.
    ///
    /// This method actually returns a builder, namely [`ChannelUpdateBuilder`].
    /// You can chain the method calls to it corresponding to the fields you want to update.
    /// Finally, calling [`update`][builder_update] method will actually perform the update.
    /// See [`ChannelUpdateBuilder`] for the fields that can be updated.
    ///
    /// [builder_update]: ChannelUpdateBuilder::update
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let channel = client.create_channel("feedback").await?;
    /// client
    ///     .update_channel(&channel)
    ///     .set_description("Give us feedback on the instance.")
    ///     .update()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    fn update_channel(&self, channel: impl EntityRef<Channel>) -> ChannelUpdateBuilder<&Self> {
        ChannelUpdateBuilder::new(self, channel)
    }

    /// Follows the specified channel.
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    fn follow_channel(
        &self,
        channel: impl EntityRef<Channel>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let channel_id = channel.entity_ref();
        Box::pin(async move {
            self.request(endpoint::channels::follow::Request { channel_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Unfollows the specified channel.
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    fn unfollow_channel(
        &self,
        channel: impl EntityRef<Channel>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let channel_id = channel.entity_ref();
        Box::pin(async move {
            self.request(endpoint::channels::unfollow::Request { channel_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Lists the channels followed by the user logged in with this client.
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    fn followed_channels(&self) -> PagerStream<BoxPager<Self, Channel>> {
        let pager = BackwardPager::new(self, endpoint::channels::followed::Request::default());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the channels owned by the user logged in with this client.
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    fn owned_channels(&self) -> PagerStream<BoxPager<Self, Channel>> {
        let pager = BackwardPager::new(self, endpoint::channels::owned::Request::default());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the featured channels.
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    fn featured_channels(&self) -> BoxFuture<Result<Vec<Channel>, Error<Self::Error>>> {
        Box::pin(async move {
            let channels = self
                .request(endpoint::channels::featured::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(channels)
        })
    }
    // }}}

    // {{{ Clip
    /// Creates a clip with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let clip = client.create_clip("name").await?;
    /// assert_eq!(clip.name, "name");
    /// # Ok(())
    /// # }
    /// ```
    fn create_clip(&self, name: impl Into<String>) -> BoxFuture<Result<Clip, Error<Self::Error>>> {
        let name = name.into();
        #[cfg(not(feature = "12-57-0"))]
        let request = endpoint::clips::create::Request { name };
        #[cfg(feature = "12-57-0")]
        let request = endpoint::clips::create::Request {
            name,
            is_public: Some(false),
            description: None,
        };
        Box::pin(async move {
            let clip = self
                .request(request)
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(clip)
        })
    }

    /// Returns a builder for creating a clip.
    ///
    /// The returned builder provides methods to customize details of the clip,
    /// and you can chain them to create a clip incrementally.
    /// Finally, calling [`create`][builder_create] method will actually create a clip.
    /// See [`ClipBuilder`] for the provided methods.
    ///
    /// [builder_create]: ClipBuilder::create
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let clip = client
    ///     .build_clip()
    ///     .name("kawaii notes")
    ///     .public(true)
    ///     .create()
    ///     .await?;
    ///
    /// assert_eq!(clip.name, "kawaii notes");
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "12-57-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
    fn build_clip(&self) -> ClipBuilder<&Self> {
        ClipBuilder::new(self)
    }

    /// Deletes the specified clip.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let clip = client.create_clip("Oops!").await?;
    /// client.delete_clip(&clip).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn delete_clip(&self, clip: impl EntityRef<Clip>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let clip_id = clip.entity_ref();
        Box::pin(async move {
            self.request(endpoint::clips::delete::Request { clip_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Lists the clips created by the user logged in with this client.
    fn clips(&self) -> BoxFuture<Result<Vec<Clip>, Error<Self::Error>>> {
        Box::pin(async move {
            let clips = self
                .request(endpoint::clips::list::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(clips)
        })
    }

    /// Lists the clips that contain the specified note.
    #[cfg(feature = "12-58-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-58-0")))]
    fn note_clips(
        &self,
        note: impl EntityRef<Note>,
    ) -> BoxFuture<Result<Vec<Clip>, Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            let clips = self
                .request(endpoint::notes::clips::Request { note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(clips)
        })
    }

    /// Clips the specified note.
    #[cfg(feature = "12-57-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
    fn clip_note(
        &self,
        clip: impl EntityRef<Clip>,
        note: impl EntityRef<Note>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let clip_id = clip.entity_ref();
        let note_id = note.entity_ref();
        Box::pin(async move {
            self.request(endpoint::clips::add_note::Request { clip_id, note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Removes the specified note from the clip.
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    fn unclip_note(
        &self,
        clip: impl EntityRef<Clip>,
        note: impl EntityRef<Note>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let clip_id = clip.entity_ref();
        let note_id = note.entity_ref();
        Box::pin(async move {
            self.request(endpoint::clips::remove_note::Request { clip_id, note_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Lists the notes that are clipped to the specified clip.
    fn clip_notes(&self, clip: impl EntityRef<Clip>) -> PagerStream<BoxPager<Self, Note>> {
        let pager = BackwardPager::new(
            self,
            endpoint::clips::notes::Request::builder()
                .clip_id(clip.entity_ref())
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Gets the corresponding clip from the ID.
    fn get_clip(&self, id: Id<Clip>) -> BoxFuture<Result<Clip, Error<Self::Error>>> {
        Box::pin(async move {
            let clip = self
                .request(endpoint::clips::show::Request { clip_id: id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(clip)
        })
    }

    /// Updates the name of the specified clip to the given one.
    #[cfg(not(feature = "12-57-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-57-0"))))]
    fn rename_clip(
        &self,
        clip: impl EntityRef<Clip>,
        name: impl Into<String>,
    ) -> BoxFuture<Result<Clip, Error<Self::Error>>> {
        let clip_id = clip.entity_ref();
        let name = name.into();
        Box::pin(async move {
            let clip = self
                .request(endpoint::clips::update::Request { clip_id, name })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(clip)
        })
    }

    /// Updates the specified clip.
    ///
    /// This method actually returns a builder, namely [`ClipUpdateBuilder`].
    /// You can chain the method calls to it corresponding to the fields you want to update.
    /// Finally, calling [`update`][builder_update] method will actually perform the update.
    /// See [`ClipUpdateBuilder`] for the fields that can be updated.
    ///
    /// [builder_update]: ClipUpdateBuilder::update
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let clip = client.create_clip("kawaii notes").await?;
    /// // Update the description and publish it.
    /// client
    ///     .update_clip(clip)
    ///     .public(true)
    ///     .description("collection of kawaii notes")
    ///     .update()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "12-57-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
    fn update_clip(&self, clip: Clip) -> ClipUpdateBuilder<&Self> {
        ClipUpdateBuilder::new(self, clip)
    }

    /// Lists the clips created by the specified user.
    #[cfg(feature = "12-61-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-61-0")))]
    fn user_clips(&self, user: impl EntityRef<User>) -> PagerStream<BoxPager<Self, Clip>> {
        let pager = BackwardPager::new(
            self,
            endpoint::users::clips::Request::builder()
                .user_id(user.entity_ref())
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }
    // }}}

    // {{{ Messaging
    /// Sends a message to the user with the given text.
    fn create_message(
        &self,
        recipient: impl EntityRef<User>,
        text: impl Into<String>,
    ) -> BoxFuture<Result<MessagingMessage, Error<Self::Error>>> {
        let recipient = recipient.entity_ref();
        let text = text.into();
        Box::pin(async move {
            self.build_message()
                .user(recipient)
                .text(text)
                .create()
                .await
        })
    }

    /// Sends a message to the user group with the given text.
    fn create_group_message(
        &self,
        recipient: impl EntityRef<UserGroup>,
        text: impl Into<String>,
    ) -> BoxFuture<Result<MessagingMessage, Error<Self::Error>>> {
        let recipient = recipient.entity_ref();
        let text = text.into();
        Box::pin(async move {
            self.build_message()
                .group(recipient)
                .text(text)
                .create()
                .await
        })
    }

    /// Returns a builder for creating a message.
    ///
    /// The returned builder provides methods to customize details of the message and its recipients,
    /// and you can chain them to create a message incrementally.
    /// Finally, calling [`create`][builder_create] method will actually create and send a message.
    /// See [`MessagingMessageBuilder`] for the provided methods.
    ///
    /// [builder_create]: MessagingMessageBuilder::create
    fn build_message(&self) -> MessagingMessageBuilder<&Self> {
        MessagingMessageBuilder::new(self)
    }

    /// Deletes the specified message.
    fn delete_message(
        &self,
        message: impl EntityRef<MessagingMessage>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let message_id = message.entity_ref();
        Box::pin(async move {
            self.request(endpoint::messaging::messages::delete::Request { message_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Marks the specified message as read.
    fn read_message(
        &self,
        message: impl EntityRef<MessagingMessage>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let message_id = message.entity_ref();
        Box::pin(async move {
            self.request(endpoint::messaging::messages::read::Request { message_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Lists the messages with the specified user.
    fn user_messages(
        &self,
        user: impl EntityRef<User>,
    ) -> PagerStream<BoxPager<Self, MessagingMessage>> {
        let pager = BackwardPager::new(
            self,
            endpoint::messaging::messages::Request::builder()
                .mark_as_read(false)
                .user_id(user.entity_ref())
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the messages in the specified user group.
    fn group_messages(
        &self,
        group: impl EntityRef<UserGroup>,
    ) -> PagerStream<BoxPager<Self, MessagingMessage>> {
        let pager = BackwardPager::new(
            self,
            endpoint::messaging::messages::Request::builder()
                .mark_as_read(false)
                .group_id(group.entity_ref())
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Gets message logs for the user who is logged in with this client.
    fn messaging_history(&self) -> BoxFuture<Result<Vec<MessagingMessage>, Error<Self::Error>>> {
        Box::pin(async move {
            let mut messages = self
                .request(endpoint::messaging::history::Request {
                    group: Some(false),
                    limit: None,
                })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            let group_messages = self
                .request(endpoint::messaging::history::Request {
                    group: Some(true),
                    limit: None,
                })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            messages.extend(group_messages);
            Ok(messages)
        })
    }
    // }}}

    // {{{ Drive
    /// Uploads the file from the given url to the drive.
    ///
    /// The difference between [`upload_file_from_url_`][alt] and this method is that the former
    /// can get the [`DriveFile`][drive_file] of the uploaded file, while the latter cannot.
    /// If you want to obtain the [`DriveFile`] of an uploaded file in v12.48.0 or later, you can
    /// use [`DriveFileUrlBuilder::upload_and_wait`] or download the file once on the client side
    /// and the use [`UploadFileClientExt::upload_file`] to upload it.
    ///
    /// [alt]: ClientExt::upload_file_from_url_
    /// [drive_file]: misskey_api::model::drive::DriveFile
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    fn upload_file_from_url(&self, url: Url) -> BoxFuture<Result<(), Error<Self::Error>>> {
        Box::pin(async move { self.build_file_from_url(url).upload().await })
    }

    /// Uploads the file from the given url to the drive.
    ///
    /// See [`upload_file_from_url`][alt] for the difference between them.
    ///
    /// [alt]: ClientExt::upload_file_from_url
    #[cfg(any(docsrs, not(feature = "12-48-0")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-48-0"))))]
    fn upload_file_from_url_(&self, url: Url) -> BoxFuture<Result<DriveFile, Error<Self::Error>>> {
        Box::pin(async move { self.build_file_from_url(url).upload_().await })
    }

    /// Returns a builder for creating a file on the drive.
    ///
    /// The returned builder provides methods to customize details of the file,
    /// and you can chain them to create a file incrementally.
    /// See [`DriveFileUrlBuilder`] for the provided methods.
    fn build_file_from_url(&self, url: Url) -> DriveFileUrlBuilder<&Self> {
        DriveFileUrlBuilder::with_url(self, url)
    }

    /// Deletes the specified file on the drive.
    fn delete_file(
        &self,
        file: impl EntityRef<DriveFile>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let file_id = file.entity_ref();
        Box::pin(async move {
            self.request(endpoint::drive::files::delete::Request { file_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Updates the specified file
    ///
    /// This method actually returns a builder, namely [`DriveFileUpdateBuilder`].
    /// You can chain the method calls to it corresponding to the fields you want to update.
    /// Finally, calling [`update`][builder_update] method will actually perform the update.
    /// See [`DriveFileUpdateBuilder`] for the fields that can be updated.
    ///
    /// [builder_update]: DriveFileUpdateBuilder::update
    fn update_file(&self, file: impl EntityRef<DriveFile>) -> DriveFileUpdateBuilder<&Self> {
        DriveFileUpdateBuilder::new(self, file)
    }

    /// Gets the corresponding file from the ID.
    fn get_file(&self, id: Id<DriveFile>) -> BoxFuture<Result<DriveFile, Error<Self::Error>>> {
        Box::pin(async move {
            let file = self
                .request(endpoint::drive::files::show::Request {
                    file_id: Some(id),
                    url: None,
                })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(file)
        })
    }

    /// Creates a folder on the drive with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let folder = client.create_folder("Folder1").await?;
    /// assert_eq!(folder.name, "Folder1");
    /// # Ok(())
    /// # }
    /// ```
    fn create_folder(
        &self,
        name: impl Into<String>,
    ) -> BoxFuture<Result<DriveFolder, Error<Self::Error>>> {
        let name = name.into();
        Box::pin(async move {
            let folder = self
                .request(endpoint::drive::folders::create::Request {
                    name: Some(name),
                    parent_id: None,
                })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(folder)
        })
    }

    /// Creates a folder on the drive with the given name and parent folder.
    fn create_folder_with_parent(
        &self,
        name: impl Into<String>,
        parent: impl EntityRef<DriveFolder>,
    ) -> BoxFuture<Result<DriveFolder, Error<Self::Error>>> {
        let name = name.into();
        let parent_id = parent.entity_ref();
        Box::pin(async move {
            let folder = self
                .request(endpoint::drive::folders::create::Request {
                    name: Some(name),
                    parent_id: Some(parent_id),
                })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(folder)
        })
    }

    /// Deletes the specified folder on the drive.
    fn delete_folder(
        &self,
        folder: impl EntityRef<DriveFolder>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let folder_id = folder.entity_ref();
        Box::pin(async move {
            self.request(endpoint::drive::folders::delete::Request { folder_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Updates the specified folder.
    ///
    /// This method actually returns a builder, namely [`DriveFolderUpdateBuilder`].
    /// You can chain the method calls to it corresponding to the fields you want to update.
    /// Finally, calling [`update`][builder_update] method will actually perform the update.
    /// See [`DriveFolderUpdateBuilder`] for the fields that can be updated.
    ///
    /// [builder_update]: DriveFolderUpdateBuilder::update
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let folder = client.create_folder("Folder1").await?;
    /// client
    ///     .update_folder(&folder)
    ///     .name("Folder2")
    ///     .update()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    fn update_folder(
        &self,
        folder: impl EntityRef<DriveFolder>,
    ) -> DriveFolderUpdateBuilder<&Self> {
        DriveFolderUpdateBuilder::new(self, folder)
    }

    /// Gets the corresponding folder from the ID.
    fn get_folder(
        &self,
        id: Id<DriveFolder>,
    ) -> BoxFuture<Result<DriveFolder, Error<Self::Error>>> {
        Box::pin(async move {
            let folder = self
                .request(endpoint::drive::folders::show::Request { folder_id: id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(folder)
        })
    }

    /// Lists the notes that have the specified file attached.
    fn attached_notes(
        &self,
        file: impl EntityRef<DriveFile>,
    ) -> BoxFuture<Result<Vec<Note>, Error<Self::Error>>> {
        let file_id = file.entity_ref();
        Box::pin(async move {
            let notes = self
                .request(endpoint::drive::files::attached_notes::Request { file_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(notes)
        })
    }

    /// Lists the files with the specified name.
    fn find_file_by_name(
        &self,
        name: impl Into<String>,
    ) -> BoxFuture<Result<Vec<DriveFile>, Error<Self::Error>>> {
        let name = name.into();
        Box::pin(async move {
            let files = self
                .request(endpoint::drive::files::find::Request {
                    name,
                    folder_id: None,
                })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(files)
        })
    }

    /// Lists the files with the specified name in the folder.
    fn find_file_by_name_in_folder(
        &self,
        name: impl Into<String>,
        folder: impl EntityRef<DriveFolder>,
    ) -> BoxFuture<Result<Vec<DriveFile>, Error<Self::Error>>> {
        let name = name.into();
        let folder_id = folder.entity_ref();
        Box::pin(async move {
            let files = self
                .request(endpoint::drive::files::find::Request {
                    name,
                    folder_id: Some(folder_id),
                })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(files)
        })
    }

    /// Lists the folders with the specified name.
    fn find_folder_by_name(
        &self,
        name: impl Into<String>,
    ) -> BoxFuture<Result<Vec<DriveFolder>, Error<Self::Error>>> {
        let name = name.into();
        Box::pin(async move {
            let files = self
                .request(endpoint::drive::folders::find::Request {
                    name,
                    parent_id: None,
                })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(files)
        })
    }

    /// Lists the folders with the specified name in the folder.
    fn find_folder_by_name_in_folder(
        &self,
        name: impl Into<String>,
        folder: impl EntityRef<DriveFolder>,
    ) -> BoxFuture<Result<Vec<DriveFolder>, Error<Self::Error>>> {
        let name = name.into();
        let folder_id = folder.entity_ref();
        Box::pin(async move {
            let files = self
                .request(endpoint::drive::folders::find::Request {
                    name,
                    parent_id: Some(folder_id),
                })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(files)
        })
    }

    /// Lists the files on the drive.
    ///
    /// This method actually returns a builder, namely [`DriveFileListBuilder`].
    /// You can specify how you want to list files by chaining methods.
    /// The [`list`][builder_list] method of the builder returns a [`Stream`][stream]
    /// that lists files in the specified way.
    ///
    /// [builder_list]: DriveFileListBuilder::list
    /// [stream]: futures::stream::Stream
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # use misskey_api as misskey;
    /// use futures::stream::TryStreamExt;
    /// use mime::IMAGE_STAR;
    /// use misskey::model::drive::DriveFile;
    ///
    /// // Get a list of image files
    /// let images: Vec<DriveFile> = client
    ///     .files()
    ///     .type_(IMAGE_STAR)
    ///     .list()
    ///     .try_collect()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    fn files(&self) -> DriveFileListBuilder<&Self> {
        DriveFileListBuilder::new(self)
    }

    /// Lists the folders.
    fn folders(&self) -> PagerStream<BoxPager<Self, DriveFolder>> {
        let pager = BackwardPager::new(self, endpoint::drive::folders::Request::default());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the folders in the folder.
    fn folders_in_folder(
        &self,
        folder: impl EntityRef<DriveFolder>,
    ) -> PagerStream<BoxPager<Self, DriveFolder>> {
        let pager = BackwardPager::new(
            self,
            endpoint::drive::folders::Request {
                folder_id: Some(folder.entity_ref()),
                ..Default::default()
            },
        );
        PagerStream::new(Box::pin(pager))
    }
    // }}}

    // {{{ Registry
    // Note on naming conventions:
    //
    // The general (but loose) naming convention of `ClientExt` that uses verbs as a prefix cannot
    // be applied to the registry API.
    // This is because most other Misskey APIs deal with models (or are REST-like), while the
    // registry API is just an operation (or RPC-like).
    //
    // We choose not to unify the naming conventions in order to have both:
    // - natural API
    //   - e.g. `user_create` seems unnatural compared to `create_user`
    // - clear naming
    //   - e.g. `get_registry` seems misleading
    /// Gets the value corresponding to the specified key in the registry scope.
    ///
    /// Returns `None` if the key is not present in the registry.
    #[cfg(feature = "12-67-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-67-0")))]
    fn registry_get(
        &self,
        scope: RegistryScope,
        key: impl Into<RegistryKey>,
    ) -> BoxFuture<Result<Option<RegistryValue>, Error<Self::Error>>> {
        let key = key.into();
        Box::pin(async move {
            use misskey_core::model::{ApiErrorId, ApiResult};
            let result = self
                .request(endpoint::i::registry::get::Request {
                    scope: Some(scope),
                    key,
                })
                .await
                .map_err(Error::Client)?;
            // https://github.com/misskey-dev/misskey/blob/12.75.1/src/server/api/endpoints/i/registry/get.ts#L26
            let get_no_such_key_id = ApiErrorId("ac3ed68a-62f0-422b-a7bc-d5e09e8f6a6a".to_string());
            if let ApiResult::Err { error } = &result {
                if error.id == get_no_such_key_id {
                    return Ok(None);
                }
            }
            let value = result.into_result()?;
            Ok(Some(value))
        })
    }

    /// Sets the corresponding value to the key in the registry scope.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # use misskey_api as misskey;
    /// use misskey::model::registry::RegistryScope;
    /// let scope = RegistryScope::from_segments(vec!["my", "app"]).unwrap();
    ///
    /// client.registry_set(scope.clone(), "counter", 42).await?;
    /// let count = client.registry_get(scope.clone(), "counter").await?;
    ///
    /// assert_eq!(count.unwrap(), 42);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "12-67-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-67-0")))]
    fn registry_set(
        &self,
        scope: RegistryScope,
        key: impl Into<RegistryKey>,
        value: impl Into<RegistryValue>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let key = key.into();
        let value = value.into();
        Box::pin(async move {
            self.request(endpoint::i::registry::set::Request {
                scope: Some(scope),
                key,
                value,
            })
            .await
            .map_err(Error::Client)?
            .into_result()?;
            Ok(())
        })
    }

    /// Deletes the corresponding value to the key in the registry scope.
    ///
    /// This differs from [`registry_clear`][clear] in that this returns an error
    /// if the specified key does not present.
    ///
    /// [clear]: `ClientExt::registry_clear`
    #[cfg(feature = "12-67-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-67-0")))]
    fn registry_delete(
        &self,
        scope: RegistryScope,
        key: impl Into<RegistryKey>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let key = key.into();
        Box::pin(async move {
            self.request(endpoint::i::registry::remove::Request {
                scope: Some(scope),
                key,
            })
            .await
            .map_err(Error::Client)?
            .into_result()?;
            Ok(())
        })
    }

    /// Clears the specified key in the registry scope.
    ///
    /// This differs from [`registry_delete`][delete] in that this returns `false` instead of
    /// returning an error if the specified key does not present.
    ///
    /// [delete]: `ClientExt::registry_delete`
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # use misskey_api as misskey;
    /// use misskey::model::registry::RegistryScope;
    /// let scope = RegistryScope::from_segments(vec!["my", "app"]).unwrap();
    ///
    /// client.registry_set(scope.clone(), "key", "test").await?;
    ///
    /// // this deletes "key"
    /// assert_eq!(client.registry_clear(scope.clone(), "key").await?, true);
    /// assert_eq!(client.registry_get(scope.clone(), "key").await?, None);
    /// // this won't fail even if "key" does not present
    /// assert_eq!(client.registry_clear(scope.clone(), "key").await?, false);
    /// assert_eq!(client.registry_get(scope.clone(), "key").await?, None);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "12-67-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-67-0")))]
    fn registry_clear(
        &self,
        scope: RegistryScope,
        key: impl Into<RegistryKey>,
    ) -> BoxFuture<Result<bool, Error<Self::Error>>> {
        let key = key.into();
        Box::pin(async move {
            use misskey_core::model::{ApiErrorId, ApiResult};
            let result = self
                .request(endpoint::i::registry::remove::Request {
                    scope: Some(scope),
                    key,
                })
                .await
                .map_err(Error::Client)?;
            // https://github.com/misskey-dev/misskey/blob/12.75.1/src/server/api/endpoints/i/registry/remove.ts#L26
            let remove_no_such_key_id =
                ApiErrorId("1fac4e8a-a6cd-4e39-a4a5-3a7e11f1b019".to_string());
            if let ApiResult::Err { error } = &result {
                if error.id == remove_no_such_key_id {
                    return Ok(false);
                }
            }
            result.into_result()?;
            Ok(true)
        })
    }

    /// Lists all the key-value pairs in the registry scope.
    #[cfg(feature = "12-67-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-67-0")))]
    fn registry_get_all(
        &self,
        scope: RegistryScope,
    ) -> BoxFuture<Result<HashMap<RegistryKey, RegistryValue>, Error<Self::Error>>> {
        Box::pin(async move {
            let values = self
                .request(endpoint::i::registry::get_all::Request { scope: Some(scope) })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(values)
        })
    }

    /// Lists all keys in the registry scope.
    #[cfg(feature = "12-67-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-67-0")))]
    fn registry_keys(
        &self,
        scope: RegistryScope,
    ) -> BoxFuture<Result<Vec<RegistryKey>, Error<Self::Error>>> {
        Box::pin(async move {
            let keys = self
                .request(endpoint::i::registry::keys::Request { scope: Some(scope) })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(keys)
        })
    }

    /// Lists the registry scopes found in all registry entries.
    #[cfg(feature = "12-67-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-67-0")))]
    fn registry_scopes(&self) -> BoxFuture<Result<Vec<RegistryScope>, Error<Self::Error>>> {
        Box::pin(async move {
            let scopes = self
                .request(endpoint::i::registry::scopes::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(scopes)
        })
    }
    // }}}

    // {{{ Page
    /// Returns a builder for creating a page.
    ///
    /// The returned builder provides methods to customize details of the page,
    /// and you can chain them to create a page incrementally.
    /// Finally, calling [`create`][builder_create] method will actually create a page.
    /// See [`PageBuilder`] for the provided methods.
    ///
    /// [builder_create]: PageBuilder::create
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # use misskey_api as misskey;
    /// use misskey::model::page::Content;
    ///
    /// let content: Content = r#"[{"type": "text", "text": "Hello World!"}]"#.parse()?;
    /// let page = client
    ///     .build_page()
    ///     .name("my_page")
    ///     .title("My Page")
    ///     .content(content)
    ///     .create()
    ///     .await?;
    ///
    /// assert_eq!(page.title, "My Page");
    /// # client.delete_page(&page).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn build_page(&self) -> PageBuilder<&Self> {
        PageBuilder::new(self)
    }

    /// Deletes the specified page.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let page = client.build_page().name("page_to_delete").create().await?;
    /// client.delete_page(&page).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn delete_page(&self, page: impl EntityRef<Page>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let page_id = page.entity_ref();
        Box::pin(async move {
            self.request(endpoint::pages::delete::Request { page_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Gets the corresponding page from the ID.
    fn get_page(&self, id: Id<Page>) -> BoxFuture<Result<Page, Error<Self::Error>>> {
        Box::pin(async move {
            let page = self
                .request(endpoint::pages::show::Request::WithPageId { page_id: id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(page)
        })
    }

    /// Gets the corresponding page from the name and the username of the author.
    fn get_page_by_name(
        &self,
        name: impl Into<String>,
        username: impl Into<String>,
    ) -> BoxFuture<Result<Page, Error<Self::Error>>> {
        let name = name.into();
        let username = username.into();
        Box::pin(async move {
            let page = self
                .request(endpoint::pages::show::Request::WithName { name, username })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(page)
        })
    }

    /// Updates the specified page.
    ///
    /// This method actually returns a builder, namely [`PageUpdateBuilder`].
    /// You can chain the method calls to it corresponding to the fields you want to update.
    /// Finally, calling [`update`][builder_update] method will actually perform the update.
    /// See [`PageUpdateBuilder`] for the fields that can be updated.
    ///
    /// [builder_update]: PageUpdateBuilder::update
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// let page = client.build_page().name("empty_page").create().await?;
    /// # let page_id = page.id;
    ///
    /// // Change name and add summary of the page
    /// client
    ///     .update_page(page)
    ///     .name("introduction")
    ///     .summary("Brief introduction to Misskey")
    ///     .update()
    ///     .await?;
    ///
    /// # client.delete_page(page_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn update_page(&self, page: Page) -> PageUpdateBuilder<&Self> {
        PageUpdateBuilder::new(self, page)
    }

    /// Gives a like to the specified page.
    fn like_page(&self, page: impl EntityRef<Page>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let page_id = page.entity_ref();
        Box::pin(async move {
            self.request(endpoint::pages::like::Request { page_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Removes a like from the specified page.
    fn unlike_page(&self, page: impl EntityRef<Page>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let page_id = page.entity_ref();
        Box::pin(async move {
            self.request(endpoint::pages::unlike::Request { page_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Pins the specified page to the profile.
    #[cfg(not(feature = "12-108-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-108-0"))))]
    fn pin_page(&self, page: impl EntityRef<Page>) -> BoxFuture<Result<User, Error<Self::Error>>> {
        let page_id = page.entity_ref();
        Box::pin(async move { self.update_me().set_pinned_page(page_id).update().await })
    }

    /// Unpins the page from the profile.
    #[cfg(not(feature = "12-108-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-108-0"))))]
    fn unpin_page(&self) -> BoxFuture<Result<User, Error<Self::Error>>> {
        Box::pin(async move { self.update_me().delete_pinned_page().update().await })
    }

    /// Lists the pages created by the user logged in with this client.
    fn pages(&self) -> PagerStream<BoxPager<Self, Page>> {
        let pager = BackwardPager::new(self, endpoint::i::pages::Request::default());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the pages liked by the user logged in with this client.
    fn liked_pages(&self) -> PagerStream<BoxPager<Self, Page>> {
        let pager = BackwardPager::new(self, endpoint::i::page_likes::Request::default())
            .map_ok(|v| v.into_iter().map(|l| l.page).collect());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the featured pages.
    #[cfg(feature = "12-58-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-58-0")))]
    fn featured_pages(&self) -> BoxFuture<Result<Vec<Page>, Error<Self::Error>>> {
        Box::pin(async move {
            let pages = self
                .request(endpoint::pages::featured::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(pages)
        })
    }

    /// Lists the pages created by the specified user.
    #[cfg(feature = "12-61-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-61-0")))]
    fn user_pages(&self, user: impl EntityRef<User>) -> PagerStream<BoxPager<Self, Page>> {
        let pager = BackwardPager::new(
            self,
            endpoint::users::pages::Request::builder()
                .user_id(user.entity_ref())
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }
    // }}}

    // {{{ Gallery
    /// Creates a gallery post with the given title and files.
    #[cfg(feature = "12-79-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
    fn create_gallery_post(
        &self,
        title: impl Into<String>,
        files: impl IntoIterator<Item = impl EntityRef<DriveFile>>,
    ) -> BoxFuture<Result<GalleryPost, Error<Self::Error>>> {
        let title = title.into();
        let files: Vec<Id<DriveFile>> = files.into_iter().map(|file| file.entity_ref()).collect();
        Box::pin(async move {
            self.build_gallery_post()
                .title(title)
                .files(files)
                .create()
                .await
        })
    }

    /// Returns a builder for creating a gallery post.
    ///
    /// The returned builder provides methods to customize details of the post,
    /// and you can chain them to create a post incrementally.
    /// Finally, calling [`create`][builder_create] method will actually create a post.
    /// See [`GalleryPostBuilder`] for the provided methods.
    ///
    /// [builder_create]: GalleryPostBuilder::create
    #[cfg(feature = "12-79-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
    fn build_gallery_post(&self) -> GalleryPostBuilder<&Self> {
        GalleryPostBuilder::new(self)
    }

    /// Deletes the specified gallery post.
    #[cfg(feature = "12-79-2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-79-2")))]
    fn delete_gallery_post(
        &self,
        post: impl EntityRef<GalleryPost>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let post_id = post.entity_ref();
        Box::pin(async move {
            self.request(endpoint::gallery::posts::delete::Request { post_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Gets the corresponding gallery post from the ID.
    #[cfg(feature = "12-79-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
    fn get_gallery_post(
        &self,
        id: Id<GalleryPost>,
    ) -> BoxFuture<Result<GalleryPost, Error<Self::Error>>> {
        Box::pin(async move {
            let post = self
                .request(endpoint::gallery::posts::show::Request { post_id: id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(post)
        })
    }

    /// Updates the gallery post.
    ///
    /// This method actually returns a builder, namely [`GalleryPostUpdateBuilder`].
    /// You can chain the method calls to it corresponding to the fields you want to update.
    /// Finally, calling [`update`][builder_update] method will actually perform the update.
    /// See [`GalleryPostUpdateBuilder`] for the fields that can be updated.
    ///
    /// [builder_update]: GalleryPostUpdateBuilder::update
    #[cfg(feature = "12-79-2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-79-2")))]
    fn update_gallery_post(&self, post: GalleryPost) -> GalleryPostUpdateBuilder<&Self> {
        GalleryPostUpdateBuilder::new(self, post)
    }

    /// Gives a like to the specified gallery post.
    #[cfg(feature = "12-79-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
    fn like_gallery_post(
        &self,
        post: impl EntityRef<GalleryPost>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let post_id = post.entity_ref();
        Box::pin(async move {
            self.request(endpoint::gallery::posts::like::Request { post_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Removes a like from the specified gallery post.
    #[cfg(feature = "12-79-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
    fn unlike_gallery_post(
        &self,
        post: impl EntityRef<GalleryPost>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let post_id = post.entity_ref();
        Box::pin(async move {
            self.request(endpoint::gallery::posts::unlike::Request { post_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Lists the gallery posts created by the user logged in with this client.
    #[cfg(feature = "12-79-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
    fn gallery_posts(&self) -> PagerStream<BoxPager<Self, GalleryPost>> {
        let pager = BackwardPager::new(self, endpoint::i::gallery::posts::Request::default());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the gallery posts liked by the user logged in with this client.
    #[cfg(feature = "12-79-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
    fn liked_gallery_posts(&self) -> PagerStream<BoxPager<Self, GalleryPost>> {
        let pager = BackwardPager::new(self, endpoint::i::gallery::likes::Request::default())
            .map_ok(|v| v.into_iter().map(|l| l.post).collect());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the gallery posts created by the specified user.
    #[cfg(feature = "12-79-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
    fn user_gallery_posts(
        &self,
        user: impl EntityRef<User>,
    ) -> PagerStream<BoxPager<Self, GalleryPost>> {
        let pager = BackwardPager::new(
            self,
            endpoint::users::gallery::posts::Request::builder()
                .user_id(user.entity_ref())
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the gallery posts in the instance.
    #[cfg(feature = "12-79-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
    fn all_gallery_posts(&self) -> PagerStream<BoxPager<Self, GalleryPost>> {
        let pager = BackwardPager::new(self, endpoint::gallery::posts::Request::default());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the featured gallery posts.
    #[cfg(feature = "12-79-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
    fn featured_gallery_posts(&self) -> BoxFuture<Result<Vec<GalleryPost>, Error<Self::Error>>> {
        Box::pin(async move {
            let posts = self
                .request(endpoint::gallery::featured::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(posts)
        })
    }

    /// Lists the popular gallery posts.
    #[cfg(feature = "12-79-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
    fn popular_gallery_posts(&self) -> BoxFuture<Result<Vec<GalleryPost>, Error<Self::Error>>> {
        Box::pin(async move {
            let posts = self
                .request(endpoint::gallery::popular::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(posts)
        })
    }
    // }}}

    // {{{ Reactions
    /// Lists the reactions to the specified note.
    fn note_reactions(
        &self,
        note: impl EntityRef<Note>,
    ) -> PagerStream<BoxPager<Self, NoteReaction>> {
        let pager = OffsetPager::new(
            self,
            endpoint::notes::reactions::Request::builder()
                .note_id(note.entity_ref())
                .build(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the reactions from the specified user in the specified range of time.
    ///
    /// The bound `Into<TimelineRange<NoteReaction>>` on the argument type is satisfied by the type
    /// of some range expressions such as `..` or `start..` (which are desugared into [`RangeFull`][range_full] and
    /// [`RangeFrom`][range_from] respectively). A reaction or [`DateTime<Utc>`][datetime] can
    /// be used to specify the start and end bounds of the range.
    ///
    /// [range_full]: std::ops::RangeFull
    /// [range_from]: std::ops::RangeFrom
    /// [datetime]: chrono::DateTime
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # let user = client.me().await?;
    /// use futures::stream::{StreamExt, TryStreamExt};
    /// use chrono::Utc;
    ///
    /// // `reactions` variable here is a `Stream` to enumerate first 100 reactions.
    /// let mut reactions = client.user_reactions(&user, ..).take(100);
    ///
    /// // Retrieve all reactions until there are no more.
    /// while let Some(reaction) = reactions.try_next().await? {
    ///     // Print the type of reaction.
    ///     println!("{}", reaction.type_);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # let user = client.me().await?;
    /// use chrono::{Duration, Utc};
    ///
    /// // Get the user reactions since `time`.
    /// let time = Utc::now() - Duration::days(1);
    /// let mut notes = client.user_notes(&user, time..);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "12-93-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-93-0")))]
    fn user_reactions(
        &self,
        user: impl EntityRef<User>,
        range: impl Into<TimelineRange<NoteReaction>>,
    ) -> PagerStream<BoxPager<Self, NoteReaction>> {
        let user_id = user.entity_ref();
        let base_request = endpoint::users::reactions::Request::builder()
            .user_id(user_id)
            .build();
        let pager = match range.into() {
            TimelineRange::Id {
                since_id,
                until_id: None,
            } => BackwardPager::with_since_id(self, since_id, base_request),
            TimelineRange::Id {
                since_id,
                until_id: Some(until_id),
            } => BackwardPager::new(
                self,
                endpoint::users::reactions::Request {
                    since_id,
                    until_id: Some(until_id),
                    ..base_request
                },
            ),
            TimelineRange::DateTime {
                since_date,
                until_date,
            } => BackwardPager::new(
                self,
                endpoint::users::reactions::Request {
                    since_date,
                    until_date: Some(until_date.unwrap_or_else(Utc::now)),
                    ..base_request
                },
            ),
            TimelineRange::Unbounded => BackwardPager::new(self, base_request),
        };
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the reactions from the specified user since the specified point in reverse order (i.e. the old reaction comes first, the new reaction comes after).
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # let user = client.me().await?;
    /// use futures::stream::{StreamExt, TryStreamExt};
    /// use chrono::{Duration, Utc};
    ///
    /// let time = Utc::now() - Duration::days(1);
    ///
    /// // `reactions_since` variable here is a `Stream` to enumerate first 100 reactions.
    /// let mut reactions_since = client.user_reactions_since(&user, time).take(100);
    ///
    /// // Retrieve all reactions until there are no more.
    /// while let Some(reaction) = reactions_since.try_next().await? {
    ///     // Print the type of reaction.
    ///     println!("{}", reaction.type_);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "12-93-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-93-0")))]
    fn user_reactions_since(
        &self,
        user: impl EntityRef<User>,
        since: impl Into<TimelineCursor<NoteReaction>>,
    ) -> PagerStream<BoxPager<Self, NoteReaction>> {
        let user_id = user.entity_ref();
        let base_request = endpoint::users::reactions::Request::builder()
            .user_id(user_id)
            .build();
        let request = match since.into() {
            TimelineCursor::DateTime(since_date) => endpoint::users::reactions::Request {
                since_date: Some(since_date),
                ..base_request
            },
            TimelineCursor::Id(since_id) => endpoint::users::reactions::Request {
                since_id: Some(since_id),
                ..base_request
            },
        };
        let pager = ForwardPager::new(self, request);
        PagerStream::new(Box::pin(pager))
    }

    /// Returns a set of streams that fetch reactions from the specified user around the specified point.
    #[cfg(feature = "12-93-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-93-0")))]
    fn user_reactions_around(
        &self,
        user: impl EntityRef<User>,
        cursor: impl Into<TimelineCursor<NoteReaction>>,
    ) -> (
        PagerStream<BoxPager<Self, NoteReaction>>,
        PagerStream<BoxPager<Self, NoteReaction>>,
    ) {
        let cursor = cursor.into();
        let user_id = user.entity_ref();
        (
            self.user_reactions_since(user_id, cursor),
            self.user_reactions(user_id, TimelineRange::until(cursor)),
        )
    }
    // }}}

    // {{{ Admin
    /// Sets moderator privileges for the specified user.
    ///
    /// This operation may require this client to be logged in with an admin account.
    fn add_moderator(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::admin::moderators::add::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Removes moderator privileges for the specified user.
    ///
    /// This operation may require this client to be logged in with an admin account.
    fn remove_moderator(
        &self,
        user: impl EntityRef<User>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::admin::moderators::remove::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Promotes the specified note until the time.
    ///
    /// This operation may require moderator privileges.
    #[cfg(feature = "12-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-13-0")))]
    fn promote_note(
        &self,
        note: impl EntityRef<Note>,
        expires_at: DateTime<Utc>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let note_id = note.entity_ref();
        Box::pin(async move {
            self.request(endpoint::admin::promo::create::Request {
                note_id,
                expires_at,
            })
            .await
            .map_err(Error::Client)?
            .into_result()?;
            Ok(())
        })
    }

    /// Lists the abuse user reports.
    ///
    /// This operation may require moderator privileges.
    fn abuse_user_reports(&self) -> PagerStream<BoxPager<Self, AbuseUserReport>> {
        let pager = BackwardPager::new(
            self,
            endpoint::admin::abuse_user_reports::Request::default(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Removes the specified abuse user report.
    ///
    /// This operation may require moderator privileges.
    #[cfg(any(docsrs, not(feature = "12-49-0")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-49-0"))))]
    fn remove_abuse_user_report(
        &self,
        report: impl EntityRef<AbuseUserReport>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let report_id = report.entity_ref();
        Box::pin(async move {
            self.request(endpoint::admin::remove_abuse_user_report::Request { report_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Marks the specified abuse user report as resolved.
    ///
    /// This operation may require moderator privileges.
    #[cfg(feature = "12-49-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-49-0")))]
    fn resolve_abuse_user_report(
        &self,
        report: impl EntityRef<AbuseUserReport>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let report_id = report.entity_ref();
        Box::pin(async move {
            self.request(endpoint::admin::resolve_abuse_user_report::Request {
                report_id,
                #[cfg(feature = "12-102-0")]
                forward: None,
            })
            .await
            .map_err(Error::Client)?
            .into_result()?;
            Ok(())
        })
    }

    /// Lists the server logs in the instance.
    ///
    /// This method actually returns a builder, namely [`ServerLogListBuilder`].
    /// You can specify how you want to list users by chaining methods.
    /// The [`list`][builder_list] method of the builder fetches the actual logs.
    ///
    /// This operation may require moderator privileges.
    ///
    /// [builder_list]: ServerLogListBuilder::list
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_client().await?;
    /// # use misskey_api as misskey;
    /// // Get a first 10 entries of 'info' logs with 'chart' domain
    /// let logs = client
    ///     .server_logs()
    ///     .take(10)
    ///     .info()
    ///     .with_domain("chart")
    ///     .list()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(not(feature = "12-93-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-93-0"))))]
    fn server_logs(&self) -> ServerLogListBuilder<&Self> {
        ServerLogListBuilder::new(self)
    }

    /// Lists the moderation logs in the instance.
    ///
    /// This operation may require moderator privileges.
    fn moderation_logs(&self) -> PagerStream<BoxPager<Self, ModerationLog>> {
        let pager = BackwardPager::new(
            self,
            endpoint::admin::show_moderation_logs::Request::default(),
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Silences the specified user.
    ///
    /// This operation may require moderator privileges.
    fn silence(&self, user: impl EntityRef<User>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::admin::silence_user::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Suspends the specified user.
    ///
    /// This operation may require moderator privileges.
    fn suspend(&self, user: impl EntityRef<User>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::admin::suspend_user::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Unsilences the specified user.
    ///
    /// This operation may require moderator privileges.
    fn unsilence(&self, user: impl EntityRef<User>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::admin::unsilence_user::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Unsuspends the specified user.
    ///
    /// This operation may require moderator privileges.
    fn unsuspend(&self, user: impl EntityRef<User>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::admin::unsuspend_user::Request { user_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Updates the instance information.
    ///
    /// This method actually returns a builder, namely [`MetaUpdateBuilder`].
    /// You can chain the method calls to it corresponding to the fields you want to update.
    /// Finally, calling [`update`][builder_update] method will actually perform the update.
    /// See [`MetaUpdateBuilder`] for the fields that can be updated.
    ///
    /// This operation may require this client to be logged in with an admin account.
    ///
    /// [builder_update]: MetaUpdateBuilder::update
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = misskey_test::test_admin_client().await?;
    /// client
    ///     .update_meta()
    ///     .set_name("The Instance of Saturn")
    ///     .local_drive_capacity(5000)
    ///     .update()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    fn update_meta(&self) -> MetaUpdateBuilder<&Self> {
        MetaUpdateBuilder::new(self)
    }

    /// Creates an announcement with given title and text.
    ///
    /// This operation may require moderator privileges.
    fn create_announcement(
        &self,
        title: impl Into<String>,
        text: impl Into<String>,
    ) -> BoxFuture<Result<Announcement, Error<Self::Error>>> {
        let title = title.into();
        let text = text.into();
        Box::pin(async move {
            let announcement = self
                .request(endpoint::admin::announcements::create::Request {
                    title,
                    text,
                    image_url: None,
                })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(announcement)
        })
    }

    /// Creates an announcement with given title, text, and image URL.
    ///
    /// This operation may require moderator privileges.
    fn create_announcement_with_image(
        &self,
        title: impl Into<String>,
        text: impl Into<String>,
        image_url: Url,
    ) -> BoxFuture<Result<Announcement, Error<Self::Error>>> {
        let title = title.into();
        let text = text.into();
        Box::pin(async move {
            let announcement = self
                .request(endpoint::admin::announcements::create::Request {
                    title,
                    text,
                    image_url: Some(image_url),
                })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(announcement)
        })
    }

    /// Deletes the specified announcement.
    ///
    /// This operation may require moderator privileges.
    fn delete_announcement(
        &self,
        announcement: impl EntityRef<Announcement>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let announcement_id = announcement.entity_ref();
        Box::pin(async move {
            self.request(endpoint::admin::announcements::delete::Request {
                id: announcement_id,
            })
            .await
            .map_err(Error::Client)?
            .into_result()?;
            Ok(())
        })
    }

    /// Updates the specified announcement.
    ///
    /// This method actually returns a builder, namely [`AnnouncementUpdateBuilder`].
    /// You can chain the method calls to it corresponding to the fields you want to update.
    /// Finally, calling [`update`][builder_update] method will actually perform the update.
    /// See [`AnnouncementUpdateBuilder`] for the fields that can be updated.
    ///
    /// This operation may require moderator privileges.
    ///
    /// [builder_update]: AnnouncementUpdateBuilder::update
    fn update_announcement(&self, announcement: Announcement) -> AnnouncementUpdateBuilder<&Self> {
        AnnouncementUpdateBuilder::new(self, announcement)
    }

    /// Creates a custom emoji from the given file.
    ///
    /// This operation may require moderator privileges.
    #[cfg(feature = "12-9-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-9-0")))]
    fn create_emoji(
        &self,
        file: impl EntityRef<DriveFile>,
    ) -> BoxFuture<Result<Id<Emoji>, Error<Self::Error>>> {
        let file_id = file.entity_ref();
        Box::pin(async move {
            let id = self
                .request(endpoint::admin::emoji::add::Request { file_id })
                .await
                .map_err(Error::Client)?
                .into_result()?
                .id;
            Ok(id)
        })
    }

    /// Deletes the specified emoji.
    ///
    /// This operation may require moderator privileges.
    fn delete_emoji(
        &self,
        emoji: impl EntityRef<Emoji>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let emoji_id = emoji.entity_ref();
        #[cfg(not(feature = "12-102-0"))]
        let request = endpoint::admin::emoji::remove::Request { id: emoji_id };
        #[cfg(feature = "12-102-0")]
        let request = endpoint::admin::emoji::delete::Request { id: emoji_id };
        Box::pin(async move {
            self.request(request)
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Updates the specified emoji.
    ///
    /// This method actually returns a builder, namely [`EmojiUpdateBuilder`].
    /// You can chain the method calls to it corresponding to the fields you want to update.
    /// Finally, calling [`update`][builder_update] method will actually perform the update.
    /// See [`EmojiUpdateBuilder`] for the fields that can be updated.
    ///
    /// This operation may require moderator privileges.
    ///
    /// [builder_update]: EmojiUpdateBuilder::update
    #[cfg(feature = "12-9-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-9-0")))]
    fn update_emoji(&self, emoji: Emoji) -> EmojiUpdateBuilder<&Self> {
        EmojiUpdateBuilder::new(self, emoji)
    }

    /// Copies the specified emoji.
    ///
    /// This operation may require moderator privileges.
    fn copy_emoji(
        &self,
        emoji: impl EntityRef<Emoji>,
    ) -> BoxFuture<Result<Id<Emoji>, Error<Self::Error>>> {
        let emoji_id = emoji.entity_ref();
        Box::pin(async move {
            let id = self
                .request(endpoint::admin::emoji::copy::Request { emoji_id })
                .await
                .map_err(Error::Client)?
                .into_result()?
                .id;
            Ok(id)
        })
    }

    /// Lists the emojis in the instance.
    ///
    /// This operation may require moderator privileges.
    /// Use [`meta`][`ClientExt::meta`] method if you want to get a list of custom emojis from normal users,
    fn emojis(&self) -> PagerStream<BoxPager<Self, Emoji>> {
        let pager = BackwardPager::new(self, endpoint::admin::emoji::list::Request::default());
        PagerStream::new(Box::pin(pager))
    }

    /// Searches the emojis using the given query string.
    ///
    /// This operation may require moderator privileges.
    #[cfg(feature = "12-48-0")]
    fn search_emojis(&self, query: impl Into<String>) -> PagerStream<BoxPager<Self, Emoji>> {
        let pager = BackwardPager::new(
            self,
            endpoint::admin::emoji::list::Request {
                query: Some(query.into()),
                ..Default::default()
            },
        );
        PagerStream::new(Box::pin(pager))
    }

    /// Creates an ad from the given urls.
    ///
    /// This operation may require moderator privileges.
    #[cfg(feature = "12-80-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-80-0")))]
    fn create_ad(
        &self,
        url: impl Into<String>,
        image_url: impl Into<String>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let url = url.into();
        let image_url = image_url.into();
        Box::pin(async move { self.build_ad().url(url).image_url(image_url).create().await })
    }

    /// Returns a builder for creating an ad.
    ///
    /// This method actually returns a builder, namely [`AdBuilder`].
    /// You can chain the method calls to it corresponding to the fields you want to update.
    /// Finally, calling [`create`][builder_create] method will actually perform the update.
    /// See [`AdBuilder`] for the fields that can be updated.
    ///
    /// This operation may require moderator privileges.
    ///
    /// [builder_create]: AdBuilder::create
    #[cfg(feature = "12-80-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-80-0")))]
    fn build_ad(&self) -> AdBuilder<&Self> {
        AdBuilder::new(self)
    }

    /// Deletes the specified ad.
    ///
    /// This operation may require moderator privileges.
    #[cfg(feature = "12-80-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-80-0")))]
    fn delete_ad(&self, ad: impl EntityRef<Ad>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let ad_id = ad.entity_ref();
        Box::pin(async move {
            self.request(endpoint::admin::ad::delete::Request { id: ad_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Updates the specified ad.
    ///
    /// This method actually returns a builder, namely [`AdUpdateBuilder`].
    /// You can chain the method calls to it corresponding to the fields you want to update.
    /// Finally, calling [`update`][builder_update] method will actually perform the update.
    /// See [`AdUpdateBuilder`] for the fields that can be updated.
    ///
    /// This operation may require moderator privileges.
    ///
    /// [builder_update]: AdUpdateBuilder::update
    #[cfg(feature = "12-80-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-80-0")))]
    fn update_ad(&self, ad: Ad) -> AdUpdateBuilder<&Self> {
        AdUpdateBuilder::new(self, ad)
    }

    /// Lists the ads in the instance.
    ///
    /// This operation may require moderator privileges.
    /// Use [`meta`][`ClientExt::meta`] method if you want to get a list of ads from normal users,
    #[cfg(feature = "12-80-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-80-0")))]
    fn ads(&self) -> PagerStream<BoxPager<Self, Ad>> {
        let pager = BackwardPager::new(self, endpoint::admin::ad::list::Request::default());
        PagerStream::new(Box::pin(pager))
    }

    /// Gets detailed information about the instance.
    ///
    /// This operation may require moderator privileges.
    #[cfg(feature = "12-109-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-109-0")))]
    fn admin_meta(&self) -> BoxFuture<Result<AdminMeta, Error<Self::Error>>> {
        Box::pin(async move {
            let meta = self
                .request(endpoint::admin::meta::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(meta)
        })
    }
    // }}}

    // {{{ Miscellaneous
    /// Gets information about the instance.
    fn meta(&self) -> BoxFuture<Result<Meta, Error<Self::Error>>> {
        Box::pin(async move {
            let meta = self
                .request(endpoint::meta::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(meta)
        })
    }

    /// Lists announcements of the instance.
    fn announcements(&self) -> PagerStream<BoxPager<Self, Announcement>> {
        let pager = BackwardPager::new(self, endpoint::announcements::Request::default())
            .map_ok(|v| v.into_iter().map(|f| f.announcement).collect());
        PagerStream::new(Box::pin(pager))
    }

    /// Marks all notifications as read.
    fn mark_all_notifications_as_read(&self) -> BoxFuture<Result<(), Error<Self::Error>>> {
        Box::pin(async move {
            self.request(endpoint::notifications::mark_all_as_read::Request::default())
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Marks the specified notification as read.
    #[cfg(feature = "12-77-1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-77-1")))]
    fn mark_notification_as_read(
        &self,
        notification: impl EntityRef<Notification>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let notification_id = notification.entity_ref();
        Box::pin(async move {
            self.request(endpoint::notifications::read::Request { notification_id })
                .await
                .map_err(Error::Client)?
                .into_result()?;
            Ok(())
        })
    }

    /// Creates a notification with the given text.
    #[cfg(feature = "12-27-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-27-0")))]
    fn create_notification(
        &self,
        body: impl Into<String>,
    ) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let body = body.into();
        Box::pin(async move { self.build_notification().body(body).create().await })
    }

    /// Returns a builder for creating a notification.
    ///
    /// The returned builder provides methods to customize details of the notification,
    /// and you can chain them to create a notification incrementally.
    /// Finally, calling [`create`][builder_create] method will actually create a notification.
    /// See [`NotificationBuilder`] for the provided methods.
    ///
    /// [builder_create]: NotificationBuilder::create
    #[cfg(feature = "12-27-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-27-0")))]
    fn build_notification(&self) -> NotificationBuilder<&Self> {
        NotificationBuilder::new(self)
    }
    // }}}
}

impl<C: Client + Sync> ClientExt for C {}

/// An extension trait for [`UploadFileClient`][client] that provides convenient high-level APIs.
///
/// [client]: misskey_core::UploadFileClient
pub trait UploadFileClientExt: UploadFileClient + Sync {
    /// Uploads the file from the specified local path.
    fn upload_file(
        &self,
        path: impl AsRef<Path>,
    ) -> BoxFuture<Result<DriveFile, Error<Self::Error>>> {
        let path = path.as_ref().to_owned();
        Box::pin(async move { self.build_file(path).upload().await })
    }

    /// Returns a builder for creating a file on the drive.
    ///
    /// The returned builder provides methods to customize details of the file,
    /// and you can chain them to create a file incrementally.
    /// See [`DriveFileBuilder`] for the provided methods.
    fn build_file(&self, path: impl AsRef<Path>) -> DriveFileBuilder<&Self> {
        DriveFileBuilder::with_path(self, path)
    }
}

impl<C: UploadFileClient + Sync> UploadFileClientExt for C {}
