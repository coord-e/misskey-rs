use crate::Error;

use misskey_api::endpoint;
use misskey_api::model::flash::Flash;
use misskey_core::Client;

/// Builder for the [`build_play`][`crate::ClientExt::build_play`] method.
pub struct FlashBuilder<C> {
    client: C,
    request: endpoint::flash::create::Request,
}

impl<C> FlashBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::flash::create::Request::default();
        FlashBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::flash::create::Request {
        &self.request
    }

    /// Sets the title of the Play.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.request.title = title.into();
        self
    }

    /// Sets the summary of the Play.
    pub fn summary(&mut self, summary: impl Into<String>) -> &mut Self {
        self.request.summary = summary.into();
        self
    }

    /// Sets the script of the Play.
    pub fn script(&mut self, script: impl Into<String>) -> &mut Self {
        self.request.script = script.into();
        self
    }

    /// Sets the permissions of the Play.
    pub fn permissions(
        &mut self,
        permissions: impl IntoIterator<Item = impl Into<String>>,
    ) -> &mut Self {
        self.request.permissions = permissions.into_iter().map(Into::into).collect();
        self
    }
}

impl<C: Client> FlashBuilder<C> {
    /// Creates the Play.
    pub async fn create(&self) -> Result<Flash, Error<C::Error>> {
        let flash = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(flash)
    }
}

/// Builder for the [`update_play`][`crate::ClientExt::update_play`] method.
pub struct FlashUpdateBuilder<C> {
    client: C,
    request: endpoint::flash::update::Request,
}

impl<C> FlashUpdateBuilder<C> {
    /// Creates a builder with the client and the Play you are going to update.
    pub fn new(client: C, flash: Flash) -> Self {
        let Flash {
            id,
            title,
            summary,
            script,
            ..
        } = flash;
        let request = endpoint::flash::update::Request {
            flash_id: id,
            title,
            summary,
            script,
            permissions: Vec::default(),
        };
        FlashUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::flash::update::Request {
        &self.request
    }

    /// Sets the title of the Play.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.request.title = title.into();
        self
    }

    /// Sets the summary of the Play.
    pub fn summary(&mut self, summary: impl Into<String>) -> &mut Self {
        self.request.summary = summary.into();
        self
    }

    /// Sets the script of the Play.
    pub fn script(&mut self, script: impl Into<String>) -> &mut Self {
        self.request.script = script.into();
        self
    }

    /// Sets the permissions of the Play.
    pub fn permissions(
        &mut self,
        permissions: impl IntoIterator<Item = impl Into<String>>,
    ) -> &mut Self {
        self.request.permissions = permissions.into_iter().map(Into::into).collect();
        self
    }
}

impl<C: Client> FlashUpdateBuilder<C> {
    /// Updates the Play.
    pub async fn update(&self) -> Result<(), Error<C::Error>> {
        self.client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(())
    }
}
