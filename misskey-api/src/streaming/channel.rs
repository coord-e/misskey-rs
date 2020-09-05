pub mod admin;
pub mod antenna;
pub mod drive;
pub mod global_timeline;
pub mod hashtag;
pub mod home_timeline;
pub mod hybrid_timeline;
pub mod local_timeline;
pub mod main;
pub mod messaging;
pub mod messaging_index;
pub mod queue_stats;
pub mod server_stats;
pub mod user_list;

#[allow(clippy::module_inception)]
#[cfg(feature = "12-47-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
pub mod channel;
