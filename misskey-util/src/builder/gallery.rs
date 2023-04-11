use crate::Error;

use misskey_api::model::{drive::DriveFile, gallery::GalleryPost};
use misskey_api::{endpoint, EntityRef};
use misskey_core::Client;

/// Builder for the [`build_gallery_post`][`crate::ClientExt::build_gallery_post`] method.
pub struct GalleryPostBuilder<C> {
    client: C,
    request: endpoint::gallery::posts::create::Request,
}

impl<C> GalleryPostBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::gallery::posts::create::Request {
            title: String::default(),
            description: None,
            file_ids: Vec::default(),
            is_sensitive: None,
        };
        GalleryPostBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::gallery::posts::create::Request {
        &self.request
    }

    /// Sets the title of the post.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.request.title = title.into();
        self
    }

    /// Sets the description of the post.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.request.description.replace(description.into());
        self
    }

    /// Sets the files of the post.
    pub fn files(
        &mut self,
        files: impl IntoIterator<Item = impl EntityRef<DriveFile>>,
    ) -> &mut Self {
        let ids = files.into_iter().map(|file| file.entity_ref());
        self.request.file_ids = ids.collect();
        self
    }

    /// Adds a file to the post.
    pub fn add_file(&mut self, file: impl EntityRef<DriveFile>) -> &mut Self {
        self.request.file_ids.push(file.entity_ref());
        self
    }

    /// Sets whether the post contains NSFW content.
    pub fn sensitive(&mut self, sensitive: bool) -> &mut Self {
        self.request.is_sensitive = Some(sensitive);
        self
    }
}

impl<C: Client> GalleryPostBuilder<C> {
    /// Creates the post.
    pub async fn create(&self) -> Result<GalleryPost, Error<C::Error>> {
        let post = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(post)
    }
}
