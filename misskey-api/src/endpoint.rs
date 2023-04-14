//! API endpoints.
//!
//! Each endpoint is implemented under modules named by replacing `/` with `::` and `-` with `_` in the endpoint name.
//! For example, `notes/local-timeline` is implemented under [`notes::local_timeline`] and
//! `drive/files/create` is implemented under [`drive::files::create`].
//!
//! All request types implements [`Request`][`misskey_core::Request`].
//! We dispatch it actually and get the [response][`misskey_core::Request::Response`]
//! using [`Client::request`][`misskey_core::Client::request`].

macro_rules! impl_pagination {
    ($name:ident, $item:ty) => {
        impl crate::PaginationRequest for $name {
            type Item = $item;

            fn set_since_id(&mut self, since_id: <$item as crate::PaginationItem>::Id) {
                self.since_id.replace(since_id);
            }
            fn set_until_id(&mut self, until_id: <$item as crate::PaginationItem>::Id) {
                self.until_id.replace(until_id);
            }
            fn set_limit(&mut self, limit: u8) {
                self.limit.replace(limit);
            }
        }
    };
}

macro_rules! impl_offset_pagination {
    ($name:ident, $item:ty) => {
        impl crate::OffsetPaginationRequest for $name {
            type Item = $item;
            fn set_offset(&mut self, offset: u64) {
                self.offset.replace(offset);
            }
            fn set_limit(&mut self, limit: u8) {
                self.limit.replace(limit);
            }
        }
    };
}

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
pub mod pages;
pub mod pinned_users;
pub mod username;
pub mod users;

#[cfg(feature = "12-47-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
pub mod channels;

#[cfg(feature = "12-65-4")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-65-4")))]
pub mod get_online_users_count;

#[cfg(feature = "12-66-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-66-0")))]
pub mod server_info;

#[cfg(feature = "12-67-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-67-0")))]
pub mod ping;

#[cfg(feature = "12-79-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
pub mod gallery;

#[cfg(feature = "12-92-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-92-0")))]
pub mod email_address;

// misskey-dev/misskey#8308
#[cfg(any(not(feature = "12-106-0"), feature = "12-107-0"))]
#[cfg_attr(docsrs, doc(cfg(any(not(feature = "12-106-0"), feature = "12-107-0"))))]
pub mod stats;
