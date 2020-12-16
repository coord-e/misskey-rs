use crate::Error;

use misskey_api::model::{channel::Channel, drive::DriveFile};
use misskey_api::{endpoint, EntityRef};
use misskey_core::Client;

/// Builder for the [`build_channel`][`crate::ClientExt::build_channel`] method.
pub struct ChannelBuilder<C> {
    client: C,
    request: endpoint::channels::create::Request,
}

impl<C> ChannelBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::channels::create::Request {
            name: String::default(),
            description: None,
            banner_id: None,
        };
        ChannelBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::channels::create::Request {
        &self.request
    }

    /// Sets the name of the channel.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name = name.into();
        self
    }

    /// Sets the description of the channel.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.request.description.replace(description.into());
        self
    }

    /// Sets the banner image of the channel.
    pub fn banner(&mut self, file: impl EntityRef<DriveFile>) -> &mut Self {
        self.request.banner_id.replace(file.entity_ref());
        self
    }
}

impl<C: Client> ChannelBuilder<C> {
    /// Creates the channel.
    pub async fn create(&self) -> Result<Channel, Error<C::Error>> {
        let channel = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(channel)
    }
}

/// Builder for the [`update_channel`][`crate::ClientExt::update_channel`] method.
pub struct ChannelUpdateBuilder<C> {
    client: C,
    request: endpoint::channels::update::Request,
}

impl<C> ChannelUpdateBuilder<C> {
    /// Creates a builder with the client and the channel you are going to update.
    pub fn new(client: C, channel: impl EntityRef<Channel>) -> Self {
        let channel_id = channel.entity_ref();
        let request = endpoint::channels::update::Request {
            channel_id,
            name: None,
            description: None,
            banner_id: None,
        };
        ChannelUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::channels::update::Request {
        &self.request
    }

    /// Sets the name of the channel.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name.replace(name.into());
        self
    }

    update_builder_string_option_field! {
        pub description;
    }

    update_builder_option_field! {
        #[doc_name = "banner image"]
        pub banner: impl EntityRef<DriveFile> { banner_id = banner.entity_ref() };
    }
}

impl<C: Client> ChannelUpdateBuilder<C> {
    /// Updates the channel.
    pub async fn update(&self) -> Result<Channel, Error<C::Error>> {
        let channel = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(channel)
    }
}
