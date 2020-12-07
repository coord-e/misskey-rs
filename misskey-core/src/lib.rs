//! Core traits and types in [misskey-rs](https://docs.rs/misskey).
#![warn(missing_docs)]

mod api;
mod client;
pub mod model;
pub mod streaming;

pub use api::*;
pub use client::Client;
