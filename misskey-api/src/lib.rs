//! API bindings of [Misskey](https://github.com/misskey-dev/misskey), including requests/responses of [endpoints][`endpoint`] and messages on [channels][`streaming::channel`], for [misskey-rs](https://docs.rs/misskey).
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod endpoint;
pub mod model;
pub mod streaming;

pub(crate) mod serde;

mod entity;
mod pagination;

pub use entity::{Entity, EntityRef};
pub use pagination::{OffsetPaginationRequest, PaginationItem, PaginationRequest};

#[cfg(test)]
mod test;
