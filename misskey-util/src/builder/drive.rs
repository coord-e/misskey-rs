use std::path::{Path, PathBuf};

use crate::pager::{BackwardPager, BoxPager, PagerStream};
use crate::Error;

#[cfg(feature = "12-48-0")]
use futures::stream::TryStreamExt;
use mime::Mime;
use misskey_api::model::drive::{DriveFile, DriveFolder};
#[cfg(feature = "13-10-0")]
use misskey_api::model::{drive::DriveFileSortKey, sort::SortOrder};
#[cfg(feature = "12-48-0")]
use misskey_api::streaming::channel;
use misskey_api::{endpoint, EntityRef};
#[cfg(feature = "12-48-0")]
use misskey_core::streaming::StreamingClient;
use misskey_core::{Client, UploadFileClient};
#[cfg(feature = "12-48-0")]
use ulid_crate::Ulid;
use url::Url;

/// Builder for the [`build_file_from_url`][`crate::ClientExt::build_file_from_url`] method.
pub struct DriveFileUrlBuilder<C> {
    client: C,
    #[cfg(feature = "12-48-0")]
    marker: String,
    request: endpoint::drive::files::upload_from_url::Request,
}

impl<C> DriveFileUrlBuilder<C> {
    /// Creates a builder with the client and URL of the upload source.
    pub fn with_url(client: C, url: Url) -> Self {
        #[cfg(feature = "12-48-0")]
        let marker = Ulid::new().to_string();
        let request = endpoint::drive::files::upload_from_url::Request {
            url,
            folder_id: None,
            is_sensitive: Some(false),
            force: Some(false),
            #[cfg(feature = "12-48-0")]
            comment: None,
            #[cfg(feature = "12-48-0")]
            marker: Some(marker.clone()),
        };
        DriveFileUrlBuilder {
            client,
            request,
            #[cfg(feature = "12-48-0")]
            marker,
        }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::drive::files::upload_from_url::Request {
        &self.request
    }

    /// Sets the parent folder of the file.
    pub fn folder(&mut self, folder: impl EntityRef<DriveFolder>) -> &mut Self {
        self.request.folder_id.replace(folder.entity_ref());
        self
    }

    /// Sets the comment for the file.
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    pub fn comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.request.comment.replace(comment.into());
        self
    }

    /// Sets whether the file contains NSFW content.
    pub fn sensitive(&mut self, sensitive: bool) -> &mut Self {
        self.request.is_sensitive = Some(sensitive);
        self
    }

    /// Sets whether or not to upload the file again even if the same content has already been
    /// uploaded.
    pub fn use_existing_if_uploaded(&mut self, use_existing_if_uploaded: bool) -> &mut Self {
        self.request.force = Some(!use_existing_if_uploaded);
        self
    }
}

impl<C: Client> DriveFileUrlBuilder<C> {
    /// Uploads the file.
    ///
    /// The difference between [`upload_`][alt] and this method is that the former
    /// can get the [`DriveFile`][drive_file] of the uploaded file, while the latter cannot.
    /// If you want to obtain the [`DriveFile`] of an uploaded file in v12.48.0 or later, you can
    /// use [`upload_and_wait`][wait] or download the file once on the client side
    /// and the use [`UploadFileClientExt::upload_file`][upload_file] to upload it.
    ///
    /// [alt]: DriveFileUrlBuilder::upload_
    /// [drive_file]: misskey_api::model::drive::DriveFile
    /// [wait]: DriveFileUrlBuilder::upload_and_wait
    /// [upload_file]: crate::UploadFileClientExt::upload_file
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    pub async fn upload(&self) -> Result<(), Error<C::Error>> {
        self.client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(())
    }

    /// Uploads the file.
    ///
    /// See [`upload`][alt] for the difference between them.
    ///
    /// [alt]: DriveFileUrlBuilder::upload
    #[cfg(any(docsrs, not(feature = "12-48-0")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-48-0"))))]
    pub async fn upload_(&self) -> Result<DriveFile, Error<C::Error>> {
        let file = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(file)
    }
}

#[cfg(feature = "12-48-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
impl<C: Client> DriveFileUrlBuilder<C>
where
    C: StreamingClient<Error = <C as Client>::Error>,
{
    /// Uploads the file and wait for a message of completion from the server.
    ///
    /// Unlike [`upload`][upload], this method returns [`DriveFile`][drive_file].
    ///
    /// [upload]: DriveFileUrlBuilder::upload
    /// [drive_file]: misskey_api::model::drive::DriveFile
    ///
    /// # Note on the use of main stream
    ///
    /// This method is implemented by waiting for the upload completion event in the main stream.
    /// However, it is currently not possible to have multiple connections to the main stream from
    /// the same client. Therefore, when you use this method, you must not not be connected to the
    /// main stream elsewhere. Likewise, you will not be able to connect to the main stream until
    /// this method is completed.
    pub async fn upload_and_wait(&self) -> Result<DriveFile, Error<<C as Client>::Error>> {
        let expected_marker = self.marker.clone();
        self.client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;

        use channel::main::{self, MainStreamEvent};
        let stream = self
            .client
            .channel(main::Request::default())
            .await
            .map_err(Error::Client)?
            .map_err(Error::Client)
            .try_filter_map(|event| async {
                match event {
                    MainStreamEvent::UrlUploadFinished {
                        marker: Some(marker),
                        file,
                    } if marker == expected_marker => Ok(Some(file)),
                    _ => Ok(None),
                }
            });
        futures::pin_mut!(stream);
        let file = stream.try_next().await?.unwrap();
        Ok(file)
    }
}

/// Builder for the [`build_file`][`crate::UploadFileClientExt::build_file`] method.
pub struct DriveFileBuilder<C> {
    client: C,
    path: PathBuf,
    type_: Mime,
    request: endpoint::drive::files::create::Request,
}

impl<C> DriveFileBuilder<C> {
    /// Creates a builder with the client and path to the file.
    pub fn with_path(client: C, path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_owned();
        let request = endpoint::drive::files::create::Request {
            name: path.file_name().map(|s| s.to_string_lossy().into_owned()),
            #[cfg(feature = "12-102-0")]
            comment: None,
            folder_id: None,
            is_sensitive: Some(false),
            force: Some(false),
        };
        let type_ = mime_guess::from_path(&path).first_or_octet_stream();
        DriveFileBuilder {
            client,
            type_,
            path,
            request,
        }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::drive::files::create::Request {
        &self.request
    }

    /// Sets the parent folder of the file.
    pub fn folder(&mut self, folder: impl EntityRef<DriveFolder>) -> &mut Self {
        self.request.folder_id.replace(folder.entity_ref());
        self
    }

    /// Sets the mime type of the file.
    pub fn type_(&mut self, type_: Mime) -> &mut Self {
        self.type_ = type_;
        self
    }

    /// Sets the name of the file.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name.replace(name.into());
        self
    }

    #[cfg(feature = "12-102-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-102-0")))]
    /// Sets the comment of the file.
    pub fn comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.request.comment.replace(comment.into());
        self
    }

    /// Sets whether the file contains NSFW content.
    pub fn sensitive(&mut self, sensitive: bool) -> &mut Self {
        self.request.is_sensitive = Some(sensitive);
        self
    }

    /// Sets whether or not to upload the file again even if the same content has already been
    /// uploaded.
    pub fn use_existing_if_uploaded(&mut self, use_existing_if_uploaded: bool) -> &mut Self {
        self.request.force = Some(!use_existing_if_uploaded);
        self
    }
}

impl<C: UploadFileClient> DriveFileBuilder<C> {
    /// Uploads the file.
    pub async fn upload(&self) -> Result<DriveFile, Error<C::Error>> {
        let fs_file = std::fs::File::open(&self.path)?;
        let file = self
            .client
            .request_with_file(
                &self.request,
                self.type_.clone(),
                self.request.name.clone().unwrap_or_default(),
                fs_file,
            )
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(file)
    }
}

/// Builder for the [`update_file`][`crate::ClientExt::update_file`] method.
pub struct DriveFileUpdateBuilder<C> {
    client: C,
    request: endpoint::drive::files::update::Request,
}

impl<C> DriveFileUpdateBuilder<C> {
    /// Creates a builder with the client and the file you are going to update.
    pub fn new(client: C, file: impl EntityRef<DriveFile>) -> Self {
        let request = endpoint::drive::files::update::Request {
            file_id: file.entity_ref(),
            folder_id: None,
            name: None,
            is_sensitive: None,
            #[cfg(feature = "12-82-0")]
            comment: None,
        };
        DriveFileUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::drive::files::update::Request {
        &self.request
    }

    /// Sets the parent folder of the file.
    pub fn set_folder(&mut self, folder: impl EntityRef<DriveFolder>) -> &mut Self {
        self.request.folder_id.replace(Some(folder.entity_ref()));
        self
    }

    /// Deletes the parent folder of the file.
    pub fn delete_folder(&mut self) -> &mut Self {
        self.request.folder_id.replace(None);
        self
    }

    /// Sets the name of the file.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name.replace(name.into());
        self
    }

    /// Sets whether the file contains NSFW content.
    pub fn sensitive(&mut self, sensitive: bool) -> &mut Self {
        self.request.is_sensitive = Some(sensitive);
        self
    }

    #[cfg(feature = "12-82-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-82-0")))]
    /// Sets the comment of the file.
    pub fn comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.request.comment.replace(comment.into());
        self
    }
}

impl<C: Client> DriveFileUpdateBuilder<C> {
    /// Updates the file.
    pub async fn update(&self) -> Result<DriveFile, Error<C::Error>> {
        let file = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(file)
    }
}

/// Builder for the [`update_folder`][`crate::ClientExt::update_folder`] method.
pub struct DriveFolderUpdateBuilder<C> {
    client: C,
    request: endpoint::drive::folders::update::Request,
}

impl<C> DriveFolderUpdateBuilder<C> {
    /// Creates a builder with the client and the folder you are going to update.
    pub fn new(client: C, folder: impl EntityRef<DriveFolder>) -> Self {
        let request = endpoint::drive::folders::update::Request {
            folder_id: folder.entity_ref(),
            parent_id: None,
            name: None,
        };
        DriveFolderUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::drive::folders::update::Request {
        &self.request
    }

    update_builder_option_field! {
        #[doc_name = "parent folder of the folder"]
        pub parent: impl EntityRef<DriveFolder> { parent_id = parent.entity_ref() };
    }

    /// Sets the name of the folder.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name.replace(name.into());
        self
    }
}

impl<C: Client> DriveFolderUpdateBuilder<C> {
    /// Updates the folder.
    pub async fn update(&self) -> Result<DriveFolder, Error<C::Error>> {
        let folder = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(folder)
    }
}

/// Builder for the [`files`][`crate::ClientExt::files`] method.
pub struct DriveFileListBuilder<C> {
    client: C,
    request: endpoint::drive::files::Request,
}

impl<C> DriveFileListBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::drive::files::Request::default();
        DriveFileListBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::drive::files::Request {
        &self.request
    }

    /// Limits the listed files to those of the specified MIME type.
    pub fn type_(&mut self, type_: Mime) -> &mut Self {
        self.request.type_.replace(type_);
        self
    }

    /// Specifies the folder to list the files.
    pub fn folder(&mut self, folder: impl EntityRef<DriveFolder>) -> &mut Self {
        self.request.folder_id.replace(folder.entity_ref());
        self
    }

    /// Sorts the results by the given order.
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub fn order(&mut self, order: SortOrder<DriveFileSortKey>) -> &mut Self {
        self.request.sort.replace(order);
        self
    }

    /// Sorts the results in ascending order by the given key.
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub fn sort_by(&mut self, key: DriveFileSortKey) -> &mut Self {
        self.order(SortOrder::Ascending(key))
    }

    /// Sorts the results in ascending order by creation date.
    ///
    /// This is equivalent to `.sort_by(DriveFileSortKey::CreatedAt)`.
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub fn sort_by_creation_date(&mut self) -> &mut Self {
        self.sort_by(DriveFileSortKey::CreatedAt)
    }

    /// Sorts the results in ascending order by number of followers.
    ///
    /// This is equivalent to `.sort_by(DriveFileSortKey::Name)`.
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub fn sort_by_name(&mut self) -> &mut Self {
        self.sort_by(DriveFileSortKey::Name)
    }

    /// Sorts the results in ascending order by update date.
    ///
    /// This is equivalent to `.sort_by(DriveFileSortKey::Size)`.
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub fn sort_by_size(&mut self) -> &mut Self {
        self.sort_by(DriveFileSortKey::Size)
    }

    /// Sorts the results in descending order by the given key.
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub fn sort_desc_by(&mut self, key: DriveFileSortKey) -> &mut Self {
        self.order(SortOrder::Descending(key))
    }

    /// Sorts the results in descending order by creation date.
    ///
    /// This is equivalent to `.sort_desc_by(DriveFileSortKey::CreatedAt)`.
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub fn sort_desc_by_creation_date(&mut self) -> &mut Self {
        self.sort_desc_by(DriveFileSortKey::CreatedAt)
    }

    /// Sorts the results in descending order by number of followers.
    ///
    /// This is equivalent to `.sort_desc_by(DriveFileSortKey::Name)`.
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub fn sort_desc_by_name(&mut self) -> &mut Self {
        self.sort_desc_by(DriveFileSortKey::Name)
    }

    /// Sorts the results in descending order by update date.
    ///
    /// This is equivalent to `.sort_desc_by(DriveFileSortKey::Size)`.
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub fn sort_desc_by_size(&mut self) -> &mut Self {
        self.sort_desc_by(DriveFileSortKey::Size)
    }
}

impl<C: Client + Sync> DriveFileListBuilder<C> {
    /// Lists the files.
    pub fn list(&self) -> PagerStream<BoxPager<C, DriveFile>> {
        let pager = BackwardPager::new(&self.client, self.request.clone());
        PagerStream::new(Box::pin(pager))
    }
}
