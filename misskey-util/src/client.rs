use crate::builder::{MeUpdateBuilder, UserListBuilder};
use crate::pager::{BackwardPager, BoxPager, OffsetPager, PagerStream};
use crate::Error;

use futures::{future::BoxFuture, stream::TryStreamExt};
use misskey_api::model::{
    following::FollowRequest,
    id::Id,
    note::Note,
    notification::Notification,
    user::{User, UserRelation},
    user_group::{UserGroup, UserGroupInvitation},
};
use misskey_api::{endpoint, EntityRef};
use misskey_core::Client;

/// An extension trait for [`Client`][client] that provides convenient high-level APIs.
///
/// [client]: misskey_core::Client
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
    /// This method returns a [`Stream`][stream] that uses pagination to fetch all entries.
    /// You can use methods from [`TryStreamExt`][try_stream_ext] or [`StreamExt`][stream_ext]
    /// to work with this.
    ///
    /// [stream]: futures::stream::Stream
    /// [try_stream_ext]: futures::stream::TryStreamExt
    /// [stream_ext]: futures::stream::StreamExt
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
    ///
    /// This method returns a [`Stream`][stream] that uses pagination to fetch all entries.
    /// You can use methods from [`TryStreamExt`][try_stream_ext] or [`StreamExt`][stream_ext]
    /// to work with this.
    ///
    /// [stream]: futures::stream::Stream
    /// [try_stream_ext]: futures::stream::TryStreamExt
    /// [stream_ext]: futures::stream::StreamExt
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
    ///
    /// This method returns a [`Stream`][stream] that uses pagination to fetch all entries.
    /// You can use methods from [`TryStreamExt`][try_stream_ext] or [`StreamExt`][stream_ext]
    /// to work with this.
    ///
    /// [stream]: futures::stream::Stream
    /// [try_stream_ext]: futures::stream::TryStreamExt
    /// [stream_ext]: futures::stream::StreamExt
    fn muting_users(&self) -> PagerStream<BoxPager<Self, User>> {
        let pager = BackwardPager::new(self, endpoint::mute::list::Request::default())
            .map_ok(|v| v.into_iter().map(|m| m.mutee).collect());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the users blocked by the user logged in with this client.
    ///
    /// This method returns a [`Stream`][stream] that uses pagination to fetch all entries.
    /// You can use methods from [`TryStreamExt`][try_stream_ext] or [`StreamExt`][stream_ext]
    /// to work with this.
    ///
    /// [stream]: futures::stream::Stream
    /// [try_stream_ext]: futures::stream::TryStreamExt
    /// [stream_ext]: futures::stream::StreamExt
    fn blocking_users(&self) -> PagerStream<BoxPager<Self, User>> {
        let pager = BackwardPager::new(self, endpoint::blocking::list::Request::default())
            .map_ok(|v| v.into_iter().map(|b| b.blockee).collect());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the notes favorited by the user logged in with this client.
    ///
    /// This method returns a [`Stream`][stream] that uses pagination to fetch all entries.
    /// You can use methods from [`TryStreamExt`][try_stream_ext] or [`StreamExt`][stream_ext]
    /// to work with this.
    ///
    /// [stream]: futures::stream::Stream
    /// [try_stream_ext]: futures::stream::TryStreamExt
    /// [stream_ext]: futures::stream::StreamExt
    fn favorited_notes(&self) -> PagerStream<BoxPager<Self, Note>> {
        let pager = BackwardPager::new(self, endpoint::i::favorites::Request::default())
            .map_ok(|v| v.into_iter().map(|f| f.note).collect());
        PagerStream::new(Box::pin(pager))
    }

    /// Lists the notifications to the user logged in with this client.
    ///
    /// This method returns a [`Stream`][stream] that uses pagination to fetch all entries.
    /// You can use methods from [`TryStreamExt`][try_stream_ext] or [`StreamExt`][stream_ext]
    /// to work with this.
    ///
    /// [stream]: futures::stream::Stream
    /// [try_stream_ext]: futures::stream::TryStreamExt
    /// [stream_ext]: futures::stream::StreamExt
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
    ///
    /// This method returns a [`Stream`][stream] that uses pagination to fetch all entries.
    /// You can use methods from [`TryStreamExt`][try_stream_ext] or [`StreamExt`][stream_ext]
    /// to work with this.
    ///
    /// [stream]: futures::stream::Stream
    /// [try_stream_ext]: futures::stream::TryStreamExt
    /// [stream_ext]: futures::stream::StreamExt
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
    /// This method returns a [`Stream`][stream] that uses pagination to fetch all entries.
    /// You can use methods from [`TryStreamExt`][try_stream_ext] or [`StreamExt`][stream_ext]
    /// to work with this.
    ///
    /// [stream]: futures::stream::Stream
    /// [try_stream_ext]: futures::stream::TryStreamExt
    /// [stream_ext]: futures::stream::StreamExt
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
}

impl<C: Client + Sync> ClientExt for C {}
