use crate::Error;

use misskey_api::endpoint;
use misskey_api::model::clip::Clip;
use misskey_core::Client;

/// Builder for the [`build_clip`][`crate::ClientExt::build_clip`] method.
pub struct ClipBuilder<C> {
    client: C,
    request: endpoint::clips::create::Request,
}

#[cfg(feature = "12-57-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
impl<C> ClipBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::clips::create::Request {
            name: String::default(),
            is_public: Some(false),
            description: None,
        };
        ClipBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::clips::create::Request {
        &self.request
    }

    /// Sets the name of the clip.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name = name.into();
        self
    }

    /// Sets whether the clip is public or not.
    pub fn public(&mut self, public: bool) -> &mut Self {
        self.request.is_public.replace(public);
        self
    }

    /// Sets the description of the clip.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.request.description.replace(description.into());
        self
    }
}

impl<C: Client> ClipBuilder<C> {
    /// Creates the clip.
    pub async fn create(&self) -> Result<Clip, Error<C::Error>> {
        let clip = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(clip)
    }
}

/// Builder for the [`update_clip`][`crate::ClientExt::update_clip`] method.
pub struct ClipUpdateBuilder<C> {
    client: C,
    request: endpoint::clips::update::Request,
}

#[cfg(feature = "12-57-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
impl<C> ClipUpdateBuilder<C> {
    /// Creates a builder with the client and the clip you are going to update.
    pub fn new(client: C, clip: Clip) -> Self {
        let Clip {
            id,
            name,
            is_public,
            description,
            ..
        } = clip;
        let request = endpoint::clips::update::Request {
            clip_id: id,
            name,
            is_public,
            description,
        };
        ClipUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::clips::update::Request {
        &self.request
    }

    /// Sets the name of the clip.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name = name.into();
        self
    }

    /// Sets whether the clip is public or not.
    pub fn public(&mut self, public: bool) -> &mut Self {
        self.request.is_public = public;
        self
    }

    /// Sets the description of the clip.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.request.description.replace(description.into());
        self
    }
}

impl<C: Client> ClipUpdateBuilder<C> {
    /// Updates the clip.
    pub async fn update(&self) -> Result<Clip, Error<C::Error>> {
        let clip = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(clip)
    }
}
