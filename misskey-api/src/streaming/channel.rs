use serde::Serialize;

/// A type with no possible values.
///
/// This is used to indicate that [`ConnectChannelRequest::Outgoing`][`misskey_core::streaming::ConnectChannelRequest`] does not exist,
/// that is, we do not send messages through that channel.
///
/// In the future when [`!`][never] is stabilized, this may be an alias for [`!`][never].
///
/// [never]: https://doc.rust-lang.org/nightly/std/primitive.never.html
#[derive(Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum NoOutgoing {}

pub mod admin;
pub mod antenna;
pub mod drive;
pub mod global_timeline;
pub mod hashtag;
pub mod home_timeline;
pub mod hybrid_timeline;
pub mod local_timeline;
pub mod main;
pub mod queue_stats;
pub mod server_stats;
pub mod user_list;

#[allow(clippy::module_inception)]
#[cfg(feature = "12-47-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
pub mod channel;

#[cfg(not(feature = "13-7-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "13-7-0"))))]
pub mod messaging;

#[cfg(not(feature = "13-7-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "13-7-0"))))]
pub mod messaging_index;

#[cfg(feature = "13-11-3")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-11-3")))]
pub mod role_timeline;
