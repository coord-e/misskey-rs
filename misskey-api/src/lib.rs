//! API bindings of [Misskey](https://github.com/syuilo/misskey), including requests/responses of [endpoints][`endpoint`] and messages on [channels][`streaming::channel`], for [misskey-rs](https://docs.rs/misskey).
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod endpoint;
pub mod model;
pub mod streaming;

#[cfg(test)]
mod test;
