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
mod channel;
mod clip;
mod drive;
mod me;
mod messaging;
mod note;
mod user;

pub use admin::{
    AnnouncementUpdateBuilder, EmojiUpdateBuilder, MetaUpdateBuilder, ServerLogListBuilder,
};
pub use antenna::{AntennaBuilder, AntennaUpdateBuilder};
pub use channel::{ChannelBuilder, ChannelUpdateBuilder};
pub use clip::{ClipBuilder, ClipUpdateBuilder};
pub use drive::{
    DriveFileBuilder, DriveFileListBuilder, DriveFileUpdateBuilder, DriveFileUrlBuilder,
    DriveFolderUpdateBuilder,
};
pub use me::{IntoUserFields, MeUpdateBuilder};
pub use messaging::MessagingMessageBuilder;
pub use note::NoteBuilder;
pub use user::UserListBuilder;
