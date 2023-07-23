use crate::Error;

#[cfg(all(feature = "12-10-0", not(feature = "13-7-0")))]
use misskey_api::model::user_group::UserGroup;
use misskey_api::model::{
    antenna::{Antenna, AntennaSource},
    query::Query,
    user_list::UserList,
};
use misskey_api::{endpoint, EntityRef};
use misskey_core::Client;

/// Builder for the [`build_antenna`][`crate::ClientExt::build_antenna`] method.
pub struct AntennaBuilder<C> {
    client: C,
    request: endpoint::antennas::create::Request,
}

impl<C> AntennaBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::antennas::create::Request {
            name: String::default(),
            src: AntennaSource::All,
            user_list_id: None,
            #[cfg(all(feature = "12-10-0", not(feature = "13-7-0")))]
            user_group_id: None,
            keywords: Query::default(),
            #[cfg(feature = "12-19-0")]
            exclude_keywords: Query::default(),
            users: Vec::default(),
            case_sensitive: false,
            with_replies: true,
            with_file: false,
            notify: false,
        };
        AntennaBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::antennas::create::Request {
        &self.request
    }

    /// Sets the name of the antenna.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name = name.into();
        self
    }

    /// Makes the antenna watch for all notes.
    pub fn all(&mut self) -> &mut Self {
        self.request.src = AntennaSource::All;
        self
    }

    /// Makes the antenna watch for notes in the home timeline.
    pub fn home(&mut self) -> &mut Self {
        self.request.src = AntennaSource::Home;
        self
    }

    /// Makes the antenna watch for notes by the specified users.
    pub fn users(&mut self, users: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.request.src = AntennaSource::Users;
        self.request.users = users.into_iter().map(Into::into).collect();
        self
    }

    /// Makes the antenna watch for notes by users in the specified user list.
    pub fn user_list(&mut self, user_list: impl EntityRef<UserList>) -> &mut Self {
        self.request.src = AntennaSource::List;
        self.request.user_list_id.replace(user_list.entity_ref());
        self
    }

    /// Makes the antenna watch for notes by users in the specified user group.
    #[cfg(all(feature = "12-10-0", not(feature = "13-7-0")))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "12-10-0", not(feature = "13-7-0")))))]
    pub fn user_group(&mut self, user_group: impl EntityRef<UserGroup>) -> &mut Self {
        self.request.src = AntennaSource::Group;
        self.request.user_group_id.replace(user_group.entity_ref());
        self
    }

    /// Sets the query that specifies which notes to include in the antenna.
    pub fn include(&mut self, query: impl Into<Query<String>>) -> &mut Self {
        self.request.keywords = query.into();
        self
    }

    /// Sets the query that specifies which notes to exclude in the antenna.
    #[cfg(feature = "12-19-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-19-0")))]
    pub fn exclude(&mut self, query: impl Into<Query<String>>) -> &mut Self {
        self.request.exclude_keywords = query.into();
        self
    }

    /// Sets whether or not the antenna is case sensitive.
    pub fn case_sensitive(&mut self, case_sensitive: bool) -> &mut Self {
        self.request.case_sensitive = case_sensitive;
        self
    }

    /// Sets whether or not the antenna would exclude the reply notes.
    pub fn exclude_replies(&mut self, exclude_replies: bool) -> &mut Self {
        self.request.with_replies = !exclude_replies;
        self
    }

    /// Sets whether or not the antenna would only target notes with attached files.
    pub fn with_files_only(&mut self, with_files_only: bool) -> &mut Self {
        self.request.with_file = with_files_only;
        self
    }

    /// Sets whether or not to be notified when a note is hit by the antenna.
    pub fn notify(&mut self, notify: bool) -> &mut Self {
        self.request.notify = notify;
        self
    }
}

impl<C: Client> AntennaBuilder<C> {
    /// Creates the antenna.
    pub async fn create(&self) -> Result<Antenna, Error<C::Error>> {
        let response = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(response)
    }
}

/// Builder for the [`update_antenna`][`crate::ClientExt::update_antenna`] method.
#[derive(Debug, Clone)]
pub struct AntennaUpdateBuilder<C> {
    client: C,
    request: endpoint::antennas::update::Request,
}

impl<C> AntennaUpdateBuilder<C> {
    /// Creates a builder with the client and the antenna you are going to update.
    pub fn new(client: C, antenna: Antenna) -> Self {
        let Antenna {
            id,
            name,
            case_sensitive,
            #[cfg(feature = "12-19-0")]
            exclude_keywords,
            keywords,
            src,
            #[cfg(all(feature = "12-10-0", not(feature = "13-7-0")))]
            user_group_id,
            user_list_id,
            users,
            notify,
            with_file,
            with_replies,
            ..
        } = antenna;
        let request = endpoint::antennas::update::Request {
            antenna_id: id,
            name,
            src,
            user_list_id,
            #[cfg(all(feature = "12-10-0", not(feature = "13-7-0")))]
            user_group_id,
            keywords,
            #[cfg(feature = "12-19-0")]
            exclude_keywords,
            users,
            case_sensitive,
            with_replies,
            with_file,
            notify,
        };
        AntennaUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::antennas::update::Request {
        &self.request
    }

    /// Sets the name of the antenna.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name = name.into();
        self
    }

    /// Makes the antenna watch for all notes.
    pub fn all(&mut self) -> &mut Self {
        self.request.src = AntennaSource::All;
        self
    }

    /// Makes the antenna watch for notes in the home timeline.
    pub fn home(&mut self) -> &mut Self {
        self.request.src = AntennaSource::Home;
        self
    }

    /// Makes the antenna watch for notes by the specified users.
    pub fn users(&mut self, users: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.request.src = AntennaSource::Users;
        self.request.users = users.into_iter().map(Into::into).collect();
        self
    }

    /// Makes the antenna watch for notes by users in the specified user list.
    pub fn user_list(&mut self, user_list: impl EntityRef<UserList>) -> &mut Self {
        self.request.src = AntennaSource::List;
        self.request.user_list_id.replace(user_list.entity_ref());
        self
    }

    /// Makes the antenna watch for notes by users in the specified user group.
    #[cfg(all(feature = "12-10-0", not(feature = "13-7-0")))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "12-10-0", not(feature = "13-7-0")))))]
    pub fn user_group(&mut self, user_group: impl EntityRef<UserGroup>) -> &mut Self {
        self.request.src = AntennaSource::Group;
        self.request.user_group_id.replace(user_group.entity_ref());
        self
    }

    /// Sets the query that specifies which notes to include in the antenna.
    pub fn include(&mut self, query: impl Into<Query<String>>) -> &mut Self {
        self.request.keywords = query.into();
        self
    }

    /// Sets the query that specifies which notes to exclude in the antenna.
    #[cfg(feature = "12-19-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-19-0")))]
    pub fn exclude(&mut self, query: impl Into<Query<String>>) -> &mut Self {
        self.request.exclude_keywords = query.into();
        self
    }

    /// Sets whether or not the antenna is case sensitive.
    pub fn case_sensitive(&mut self, case_sensitive: bool) -> &mut Self {
        self.request.case_sensitive = case_sensitive;
        self
    }

    /// Sets whether or not the antenna would exclude the reply notes.
    pub fn exclude_replies(&mut self, exclude_replies: bool) -> &mut Self {
        self.request.with_replies = !exclude_replies;
        self
    }

    /// Sets whether or not the antenna would only target notes with attached files.
    pub fn with_files_only(&mut self, with_files_only: bool) -> &mut Self {
        self.request.with_file = with_files_only;
        self
    }

    /// Sets whether or not to be notified when a note is hit by the antenna.
    pub fn notify(&mut self, notify: bool) -> &mut Self {
        self.request.notify = notify;
        self
    }
}

impl<C: Client> AntennaUpdateBuilder<C> {
    /// Updates the antenna.
    pub async fn update(&self) -> Result<Antenna, Error<C::Error>> {
        let response = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(response)
    }
}
