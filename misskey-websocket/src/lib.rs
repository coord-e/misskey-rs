//! Asynchronous WebSocket-based client implementation for Misskey.
//!
//! # Feature flags
//!
//! - `tokio-runtime`: Use the [tokio](https://tokio.rs) runtime. Enabled by default.
//! - `async-std-runtime`: Use the [async-std](https://async.rs) runtime.
#![warn(missing_docs)]

mod broker;
mod channel;
mod client;
mod error;
mod model;

pub use broker::{ReconnectCondition, ReconnectConfig};
pub use client::{builder::WebSocketClientBuilder, stream, WebSocketClient};
pub use error::Error;
