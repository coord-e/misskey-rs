use crate::builder::{MeUpdateBuilder, NoteBuilder, UserListBuilder};
use crate::pager::{BackwardPager, BoxPager, ForwardPager, OffsetPager, PagerStream};
use crate::Error;
use crate::{TimelineCursor, TimelineRange};

use chrono::Utc;
use futures::{future::BoxFuture, stream::TryStreamExt};
use mime::Mime;
use misskey_api::model::{
    channel::Channel,
    following::FollowRequest,
    id::Id,
    note::{Note, Reaction, Tag},
    notification::Notification,
    query::Query,
    user::{User, UserRelation},
    user_group::{UserGroup, UserGroupInvitation},
    user_list::UserList,
};
use misskey_api::{endpoint, EntityRef};
use misskey_core::Client;

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
            /// # let user = client.users().list().try_next().await?.unwrap();
            /// # let channel = client.create_channel("test").await?;
            /// # let list = client.create_user_list("test").await?;
            /// use futures::stream::TryStreamExt;
            ///
            #[doc = "// `notes` variable here is a `Stream` to enumerate all " $timeline " notes."]
            #[doc = "let mut notes = client." $timeline "_notes(" $("&" $argname ", ")* "..);"]
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
            /// # let user = client.users().list().try_next().await?.unwrap();
            /// # let channel = client.create_channel("test").await?;
            /// # let list = client.create_user_list("test").await?;
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
            /// # let user = client.users().list().try_next().await?.unwrap();
            /// # let channel = client.create_channel("test").await?;
            /// # let list = client.create_user_list("test").await?;
            /// use futures::stream::TryStreamExt;
            /// use chrono::Utc;
            ///
            /// let time = Utc::today().and_hms(0, 0, 0);
            ///
            #[doc = "// `notes_since` is a `Stream` to enumerate the " $timeline " notes since `time` in reverse order."]
            #[doc = "let mut notes_since = client." $timeline "_notes_since(" $("&" $argname ", ")* "time);"]
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

    /// Mutes the specified user.
    fn mute(&self, user: impl EntityRef<User>) -> BoxFuture<Result<(), Error<Self::Error>>> {
        let user_id = user.entity_ref();
        Box::pin(async move {
            self.request(endpoint::mute::create::Request { user_id })
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
    /// use futures::stream::TryStreamExt;
    ///
    /// // In this example, we will fetch all the followers and follow them.
    /// // First, obtain your information to pass to `.follwers` method.
    /// let me = client.me().await?;
    ///
    /// // `follwers` variable here is a `Stream` to enumerate all the followers of `me`.
    /// let mut followers = client.followers(&me);
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
    /// use futures::stream::TryStreamExt;
    ///
    /// // `notifications` here is a `Stream` to enumerate all the notifications.
    /// let mut notifications = client.notifications();
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    fn users(&self) -> UserListBuilder<&Self> {
        UserListBuilder::new(self)
    }

    /// Lists the recommended users of the instance.
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
}

impl<C: Client + Sync> ClientExt for C {}
