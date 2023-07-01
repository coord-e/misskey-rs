use crate::Error;

use misskey_api::model::user_list::UserList;
use misskey_api::{endpoint, EntityRef};
use misskey_core::Client;

/// Builder for the [`update_user_list`][`crate::ClientExt::update_user_list`] method.
pub struct UserListUpdateBuilder<C> {
    client: C,
    request: endpoint::users::lists::update::Request,
}

impl<C> UserListUpdateBuilder<C> {
    /// Creates a builder with the client and the list you are going to update.
    pub fn new(client: C, list: impl EntityRef<UserList>) -> Self {
        let request = endpoint::users::lists::update::Request {
            list_id: list.entity_ref(),
            #[cfg(not(feature = "13-13-0"))]
            name: String::default(),
            #[cfg(feature = "13-13-0")]
            name: None,
            #[cfg(feature = "13-13-0")]
            is_public: None,
        };
        UserListUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::users::lists::update::Request {
        &self.request
    }

    #[cfg(not(feature = "13-13-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-13-0"))))]
    /// Sets the name of the user list.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name = name.into();
        self
    }

    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    /// Sets the name of the user list.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name.replace(name.into());
        self
    }

    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    /// Sets whether the user list is public or not.
    pub fn public(&mut self, is_public: bool) -> &mut Self {
        self.request.is_public.replace(is_public);
        self
    }
}

impl<C: Client> UserListUpdateBuilder<C> {
    /// Updates the clip.
    pub async fn update(&self) -> Result<UserList, Error<C::Error>> {
        let list = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(list)
    }
}
