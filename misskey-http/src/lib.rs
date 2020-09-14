//! Asynchronous HTTP-based client implementation for Misskey.
#![warn(missing_docs)]

mod client;
mod error;

pub use client::{builder::HttpClientBuilder, HttpClient};
pub use error::Error;
