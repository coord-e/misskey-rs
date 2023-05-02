//! Builder types.
//!
//! It is used by [`ClientExt`][client_ext] to incrementally construct a [`Request`][request]s.
//! Each builders has its own methods (`create`, `update`, etc.) to perform the desired operation
//! with the client it contains. You can also get the request object for reuse using a method
//! named `as_request`.
//!
//! [client_ext]: crate::ClientExt
//! [request]: misskey_core::Request

mod admin;
mod antenna;
mod clip;
mod drive;
mod me;
mod messaging;
mod misc;
mod note;
mod page;

#[cfg(feature = "12-47-0")]
mod channel;

#[cfg(feature = "12-79-0")]
mod gallery;

#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
mod user;

pub use admin::{AnnouncementUpdateBuilder, EmojiUpdateBuilder, MetaUpdateBuilder};
pub use antenna::{AntennaBuilder, AntennaUpdateBuilder};
pub use clip::{ClipBuilder, ClipUpdateBuilder};
pub use drive::{
    DriveFileBuilder, DriveFileListBuilder, DriveFileUpdateBuilder, DriveFileUrlBuilder,
    DriveFolderUpdateBuilder,
};
pub use me::MeUpdateBuilder;
pub use messaging::MessagingMessageBuilder;
pub use note::NoteBuilder;
pub use page::{PageBuilder, PageUpdateBuilder};

#[cfg(feature = "12-47-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
pub use channel::{ChannelBuilder, ChannelUpdateBuilder};

#[cfg(feature = "12-27-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-27-0")))]
pub use misc::NotificationBuilder;

#[cfg(feature = "12-79-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
pub use gallery::GalleryPostBuilder;

#[cfg(feature = "12-79-2")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-79-2")))]
pub use gallery::GalleryPostUpdateBuilder;

#[cfg(feature = "12-80-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-80-0")))]
pub use admin::{AdBuilder, AdUpdateBuilder};

// misskey-dev/misskey#7656
#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
#[cfg_attr(docsrs, doc(cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))))]
pub use user::UserListBuilder;

#[cfg(not(feature = "12-93-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-93-0"))))]
pub use admin::ServerLogListBuilder;

#[cfg(not(feature = "12-108-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-108-0"))))]
pub use me::IntoUserFields;

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
pub use admin::{DefaultPoliciesUpdateBuilder, RoleBuilder, RoleUpdateBuilder};
