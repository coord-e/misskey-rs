pub mod active_users;
pub mod drive;
pub mod federation;
pub mod hashtag;
pub mod instance;
pub mod notes;
pub mod user;
pub mod users;

#[cfg(not(feature = "12-104-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-104-0"))))]
pub mod network;

#[cfg(feature = "12-104-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-104-0")))]
pub mod ap_request;
