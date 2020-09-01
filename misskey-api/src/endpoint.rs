pub mod admin;
pub mod announcements;
pub mod antennas;
pub mod blocking;
pub mod charts;
pub mod clips;
pub mod drive;
#[allow(clippy::module_inception)]
pub mod endpoint;
pub mod endpoints;
pub mod following;
pub mod i;
pub mod messaging;
pub mod meta;
pub mod mute;
pub mod notes;
pub mod notifications;
pub mod pinned_users;
pub mod stats;
pub mod username;
pub mod users;

#[cfg(feature = "12-47-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
pub mod channels;
