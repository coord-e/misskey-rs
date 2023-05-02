use crate::pager::{BoxPager, OffsetPager, PagerStream};

use misskey_api::endpoint;
use misskey_api::model::{
    sort::SortOrder,
    user::{User, UserOrigin, UserSortKey},
};
use misskey_core::Client;

/// Builder for the [`users`][`crate::ClientExt::users`] method.
pub struct UserListBuilder<C> {
    client: C,
    request: endpoint::users::Request,
}

impl<C> UserListBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::users::Request::default();
        UserListBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::users::Request {
        &self.request
    }

    /// Sorts the results by the given order.
    pub fn order(&mut self, order: SortOrder<UserSortKey>) -> &mut Self {
        self.request.sort.replace(order);
        self
    }

    /// Sorts the results in ascending order by the given key.
    pub fn sort_by(&mut self, key: UserSortKey) -> &mut Self {
        self.order(SortOrder::Ascending(key))
    }

    /// Sorts the results in ascending order by number of followers.
    ///
    /// This is equivalent to `.sort_by(UserSortKey::Follower)`.
    pub fn sort_by_followers(&mut self) -> &mut Self {
        self.sort_by(UserSortKey::Follower)
    }

    /// Sorts the results in ascending order by creation date.
    ///
    /// This is equivalent to `.sort_by(UserSortKey::CreatedAt)`.
    pub fn sort_by_creation_date(&mut self) -> &mut Self {
        self.sort_by(UserSortKey::CreatedAt)
    }

    /// Sorts the results in ascending order by update date.
    ///
    /// This is equivalent to `.sort_by(UserSortKey::UpdatedAt)`.
    pub fn sort_by_update_date(&mut self) -> &mut Self {
        self.sort_by(UserSortKey::UpdatedAt)
    }

    /// Limits the listed users by its origin.
    pub fn origin(&mut self, origin: UserOrigin) -> &mut Self {
        self.request.origin.replace(origin);
        self
    }

    /// Limits the listed users to local ones.
    ///
    /// This is equivalent to `.origin(UserOrigin::Local)`.
    pub fn local(&mut self) -> &mut Self {
        self.origin(UserOrigin::Local)
    }

    /// Limits the listed users to remote ones.
    ///
    /// This is equivalent to `.origin(UserOrigin::Remote)`.
    pub fn remote(&mut self) -> &mut Self {
        self.origin(UserOrigin::Remote)
    }

    /// Limits the listed users to moderators.
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub fn moderator(&mut self) -> &mut Self {
        self.request
            .state
            .replace(endpoint::users::UserState::Moderator);
        self
    }

    /// Limits the listed users to alive ones.
    pub fn alive(&mut self) -> &mut Self {
        self.request
            .state
            .replace(endpoint::users::UserState::Alive);
        self
    }

    /// Limits the listed users to admins.
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub fn admin(&mut self) -> &mut Self {
        self.request
            .state
            .replace(endpoint::users::UserState::Admin);
        self
    }

    /// Limits the listed users to admins or moderators.
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub fn admin_or_moderator(&mut self) -> &mut Self {
        self.request
            .state
            .replace(endpoint::users::UserState::AdminOrModerator);
        self
    }

    /// Limits the host from which users are listed.
    ///
    /// To list users in the local host, use [`local`][`UserListBuilder::local`] method instead.
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    pub fn hostname(&mut self, hostname: impl Into<String>) -> &mut Self {
        self.request.hostname.replace(hostname.into());
        self
    }
}

impl<C: Client + Sync> UserListBuilder<C> {
    /// Lists the users.
    pub fn list(&self) -> PagerStream<BoxPager<C, User>> {
        let pager = OffsetPager::new(&self.client, self.request.clone());
        PagerStream::new(Box::pin(pager))
    }
}
