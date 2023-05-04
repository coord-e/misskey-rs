//! Asynchronous client for [Misskey](https://github.com/misskey-dev/misskey).
//!
//! We provide four components in this crate:
//!
//! - Clients that handles the connection between Misskey. As Misskey provides HTTP and WebSocket
//!   interfaces to interact with, we have [`HttpClient`] and [`WebSocketClient`] implementations
//!   correspondingly.
//! - API bindings, including requests/responses of [endpoints][`endpoint`] and messages on
//!   [channels][`streaming::channel`].
//! - Abstraction that connects API datatypes and client implementations: [`Request`][`endpoint::Request`],
//!   [`ConnectChannelRequest`][`streaming::ConnectChannelRequest`], etc.
//! - High-level API for easier handling of various functionalities: [`ClientExt`] and others.
//!
//! # Examples
//!
//! Create a note:
//!
//! ```no_run
//! use misskey::prelude::*;
//! use misskey::HttpClient;
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! let client = HttpClient::builder("https://your.instance.example/api/")
//!     .token("API_TOKEN")
//!     .build()?;
//!
//! client.create_note("Hello, Misskey").await?;
//! # Ok(())
//! # }
//! ```
//!
//! Automatically follow-back:
//!
//! ```no_run
//! use futures::stream::TryStreamExt;
//! use misskey::prelude::*;
//! use misskey::streaming::channel::main::MainStreamEvent;
//! use misskey::{HttpClient, WebSocketClient};
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! let http_client = HttpClient::builder("https://your.instance.example/api/")
//!     .token("API_TOKEN")
//!     .build()?;
//!
//! let ws_client = WebSocketClient::builder("wss://your.instance.example/streaming")
//!     .token("YOUR_API_TOKEN")
//!     .connect()
//!     .await?;
//!
//! // Connect to the main stream.
//! // The main stream is a channel that streams events about the connected account.
//! let mut stream = ws_client.main_stream().await?;
//!
//! // Wait for the next event in the main stream.
//! while let Some(event) = stream.try_next().await? {
//!     match event {
//!         // Check if the event is 'followed' and the user is not a bot
//!         MainStreamEvent::Followed(user) if !user.is_bot => {
//!             println!("followed from @{}", user.username);
//!
//!             // Follow back `user` if you haven't already.
//!             if !http_client.is_following(&user).await? {
//!                 http_client.follow(&user).await?;
//!             }
//!         }
//!         // other events are just ignored here
//!         _ => {}
//!    }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! See the [example](https://github.com/coord-e/misskey-rs/tree/develop/example) directory for more examples.
//!
//! # Feature flags
//!
//! - `http-client`: Enables the HTTP client which is capable for uploading files.
//!   Enabled by default.
//! - `websocket-client`: Enables the WebSocket client which is capable for streaming.
//!   Enabled by default.
//! - `tokio-runtime`: Use the [tokio](https://tokio.rs) v1.0 runtime in the WebSocket client.
//!   Enabled by default.
//! - `async-std-runtime`: Use the [async-std](https://async.rs) runtime in the WebSocket client.
//! - `aid`: Assume that the `aid` ID generation method is used in the targeted Misskey instance.
//!   Enabled by default.
//! - `meid`: Assume that the `meid` ID generation method is used in the targeted Misskey instance.
//! - `ulid`: Assume that the `ulid` ID generation method is used in the targeted Misskey instance.
//! - `objectid`: Assume that the `objectid` ID generation method is used in the targeted Misskey instance.
//! - and version flags, as described in [version flags section](#specifying-misskey-version).
//!
//! ## Specifying Misskey version
//!
//! We have a set of feature flags to specify the targeted Misskey version.
//! For example, to compile for Misskey v12.33.0 with WebSocket client on async-std runtime, add
//! the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies.misskey]
//! version = "0.2"
//! default-features = false
//! features = ["12-31-0", "websocket-client", "async-std-runtime", "aid"]
//! ```
//!
//! | Feature                    | Supported Misskey versions (inclusive) | Tested Misskey version |
//! | -------------------------- | -------------------------------------- | ---------------------- |
//! | `13-3-0`                   | v13.3.0 ~ v13.3.1                      | v13.3.0                |
//! | `13-2-4`                   | v13.2.4 ~ v13.2.6                      | v13.2.5                |
//! | `13-2-3`                   | v13.2.3                                | v13.2.3                |
//! | `13-1-1`                   | v13.1.1 ~ v13.2.2                      | v13.1.1                |
//! | `13-1-0`                   | v13.1.0                                | v13.1.0                |
//! | `13-0-0`                   | v13.0.0                                | v13.0.0                |
//! | `12-112-3`                 | v12.112.3 ~ v12.119.2                  | v12.112.3              |
//! | `12-112-0`                 | v12.112.0 ~ v12.112.2                  | v12.112.2              |
//! | `12-111-0`                 | v12.111.0 ~ v12.111.1                  | v12.111.0              |
//! | `12-109-0`                 | v12.109.0 ~ v12.110.1                  | v12.109.2              |
//! | `12-108-0`                 | v12.108.0 ~ v12.108.1                  | v12.108.1              |
//! | `12-107-0`                 | v12.107.0                              | v12.107.0              |
//! | `12-106-0`                 | v12.106.0 ~ v12.106.3                  | v12.106.0              |
//! | `12-105-0`                 | v12.105.0                              | v12.105.0              |
//! | `12-104-0`                 | v12.104.0                              | v12.104.0              |
//! | `12-102-0`                 | v12.102.0 ~ v12.103.1                  | v12.102.0              |
//! | `12-99-0`                  | v12.99.0 ~ v12.101.1                   | v12.99.1               |
//! | `12-98-0`                  | v12.98.0                               | v12.98.0               |
//! | `12-96-0`                  | v12.96.0 ~ v12.97.1                    | v12.97.0               |
//! | `12-95-0`                  | v12.95.0                               | v12.95.0               |
//! | `12-93-0`                  | v12.93.0 ~ v12.94.1                    | v12.93.0               |
//! | `12-92-0`                  | v12.92.0                               | v12.92.0               |
//! | `12-91-0`                  | v12.91.0                               | v12.91.0               |
//! | `12-89-1`                  | v12.89.1 ~ v12.90.1                    | v12.89.1               |
//! | `12-89-0`                  | v12.89.0                               | v12.89.0               |
//! | `12-88-0`                  | v12.88.0                               | v12.88.0               |
//! | `12-82-0`                  | v12.82.0 ~ v12.87.0                    | v12.82.0               |
//! | `12-81-0`                  | v12.81.0 ~ v12.81.2                    | v12.81.0               |
//! | `12-80-0`                  | v12.80.0 ~ v12.80.3                    | v12.80.0               |
//! | `12-79-2`                  | v12.79.2 ~ v12.79.3                    | v12.79.2               |
//! | `12-79-0`                  | v12.79.0 ~ v12.79.1                    | v12.79.0               |
//! | `12-77-1`                  | v12.77.1 ~ v12.78.0                    | v12.77.1               |
//! | `12-77-0`                  | v12.77.0                               | v12.77.0               |
//! | `12-75-0`                  | v12.75.0 ~ v12.76.1                    | v12.75.0               |
//! | `12-71-0`                  | v12.71.0 ~ v12.74.1                    | v12.71.0               |
//! | `12-70-0`                  | v12.70.0                               | v12.70.0               |
//! | `12-69-0`                  | v12.69.0                               | v12.69.0               |
//! | `12-67-0`                  | v12.67.0 ~ v12.68.0                    | v12.68.0               |
//! | `12-66-0`                  | v12.66.0                               | v12.66.0               |
//! | `12-65-4`                  | v12.65.4 ~ v12.65.7                    | v12.65.4               |
//! | `12-63-0`                  | v12.63.0 ~ v12.65.3                    | v12.63.0               |
//! | `12-62-2`                  | v12.62.2                               | v12.62.2               |
//! | `12-62-0`                  | v12.62.0 ~ v12.62.1                    | v12.62.0               |
//! | `12-61-0`                  | v12.61.0 ~ v12.61.1                    | v12.61.0               |
//! | `12-60-0`                  | v12.60.0 ~ v12.60.1                    | v12.60.0               |
//! | `12-58-0`                  | v12.58.0 ~ v12.59.0                    | v12.58.0               |
//! | `12-57-0`                  | v12.57.0 ~ v12.57.4                    | v12.57.1               |
//! | `12-55-0`                  | v12.55.0 ~ v12.56.0                    | v12.55.0               |
//! | `12-51-0`                  | v12.51.0 ~ v12.54.0                    | v12.51.0               |
//! | `12-49-0`                  | v12.49.0 ~ v12.50.0                    | v12.49.0               |
//! | `12-48-0`                  | v12.48.0 ~ v12.48.3                    | v12.48.0               |
//! | `12-47-0`                  | v12.47.0 ~ v12.47.1                    | v12.47.1               |
//! | `12-42-0`                  | v12.42.0 ~ v12.46.0                    | v12.42.0               |
//! | `12-39-0`                  | v12.39.0 ~ v12.41.3                    | v12.39.0               |
//! | `12-37-0`                  | v12.37.0 ~ v12.38.1                    | v12.37.0               |
//! | `12-31-0`                  | v12.31.0 ~ v12.36.1                    | v12.31.0               |
//! | `12-29-0`                  | v12.29.0 ~ v12.30.0                    | v12.29.0               |
//! | `12-28-0`                  | v12.28.0                               | v12.28.0               |
//! | `12-27-0`                  | v12.27.0 ~ v12.27.1                    | v12.27.0               |
//! | `12-19-0`                  | v12.19.0 ~ v12.26.0                    | v12.20.0               |
//! | `12-13-0`                  | v12.13.0 ~ v12.18.1                    | v12.13.0               |
//! | `12-10-0`                  | v12.10.0 ~ v12.12.0                    | v12.10.0               |
//! | `12-9-0`                   | v12.9.0                                | v12.9.0                |
//! | `12-8-0`                   | v12.8.0                                | v12.8.0                |
//! | `12-5-0`                   | v12.5.0 ~ v12.7.1                      | v12.5.0                |
//! | (no version flag enabled)  | v12.0.0 ~ v12.4.1                      | v12.0.0                |
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]

pub mod endpoint {
    //! API endpoints.
    //!
    //! Each endpoint is implemented under modules named by replacing `/` with `::` and `-` with `_` in the endpoint name.
    //! For example, `notes/local-timeline` is implemented under [`notes::local_timeline`] and
    //! `drive/files/create` is implemented under [`drive::files::create`].
    //!
    //! All request types implement [`Request`].
    //! We dispatch it actually and get the [response][`Request::Response`]
    //! using [`Client::request`][`crate::Client::request`].
    //!
    //! # Example
    //!
    //! Create a note using `/api/notes/create`:
    //!
    //! ```no_run
    //! # use misskey::{Client, HttpClient};
    //! # #[tokio::main]
    //! # async fn main() -> anyhow::Result<()> {
    //! # let client = HttpClient::with_token("http://your.instance.example/api/".parse()?, "API_TOKEN")?;
    //! client
    //!     .request(
    //!         // Each endpoint implementation has a corresponding `Request` type.
    //!         // We can dispatch an API call by passing `Request` to `Client::request` method.
    //!         // Here, we build a `Request` to `notes/create` using a `Request::builder()`.
    //!         misskey::endpoint::notes::create::Request::builder()
    //!             .text("Hello, Misskey")
    //!             .build(),
    //!     )
    //!     // Asynchronously wait for the response.
    //!     // `Client::request` method returns `Result<ApiResult<T>>`.
    //!     // The returned `Result` may contain an error happened on our side
    //!     // (e.g. networking failure or deserialization error)
    //!     .await?
    //!     // Convert `ApiResult<T>` to `Result<T, ApiError>` using `ApiResult::into_result`.
    //!     // `ApiError` is an error which is returned from Misskey API.
    //!     .into_result()?;
    //! # Ok(())
    //! # }
    //! ```
    //!
    //! Get your own information from `/api/i`:
    //!
    //! ```no_run
    //! # use misskey::{Client, HttpClient};
    //! # #[tokio::main]
    //! # async fn main() -> anyhow::Result<()> {
    //! # let client = HttpClient::with_token("http://your.instance.example/api/".parse()?, "API_TOKEN")?;
    //!     let me = client
    //!         .request(misskey::endpoint::i::Request::default())
    //!         .await?
    //!         .into_result()?;
    //!     println!("{:?}", me);
    //! # Ok(())
    //! # }
    //! ```

    // Because the `docsrs` cfg flag does not propagate to `misskey-api`, we're dealing with this
    // by specifying `no_inline` as a workaround.
    #[doc(no_inline)]
    pub use misskey_api::endpoint::*;
    pub use misskey_api::{OffsetPaginationRequest, PaginationRequest};
    pub use misskey_core::{Request, UploadFileRequest};
}

pub mod streaming {
    //! Streaming API.
    //!
    //! # Example
    //!
    //! Stream the notes in the local timeline:
    //!
    //! ```no_run
    //! use futures::stream::StreamExt;
    //! use misskey::streaming::channel::local_timeline::{self, LocalTimelineEvent};
    //! # use misskey::WebSocketClient;
    //! # #[tokio::main]
    //! # async fn main() -> anyhow::Result<()> {
    //! # let client = WebSocketClient::builder("wss://your.instance.example/streaming")
    //! #     .token("API_TOKEN")
    //! #     .connect()
    //! #     .await?;
    //!
    //! // Connect to the local timeline channel.
    //! let mut stream = client.channel(local_timeline::Request::default()).await?;
    //!
    //! loop {
    //!     // Wait for the next note using `next` method from `StreamExt`.
    //!     let LocalTimelineEvent::Note(note) = stream.next().await.unwrap()?;
    //!     println!("{:?}", note);
    //! }
    //! # Ok(())
    //! # }
    //! ```
    //!
    //! Capture the note:
    //!
    //! ```no_run
    //! use futures::stream::StreamExt;
    //! use misskey::model::{note::Note, id::Id};
    //! use misskey::streaming::note::NoteUpdateEvent;
    //! # use misskey::WebSocketClient;
    //! # #[tokio::main]
    //! # async fn main() -> anyhow::Result<()> {
    //! # let client = WebSocketClient::builder("wss://your.instance.example/streaming")
    //! #     .token("API_TOKEN")
    //! #     .connect()
    //! #     .await?;
    //!
    //! let mut stream = client.subnote("NOTE_ID_TO_WATCH").await?;
    //!
    //! loop {
    //!     // Wait for the event note using `next` method from `StreamExt`.
    //!     let event = stream.next().await.unwrap()?;
    //!
    //!     match event {
    //!        NoteUpdateEvent::Reacted { reaction, user_id } => {
    //!           println!("{:?} added {:?}", user_id, reaction);
    //!        }
    //!        NoteUpdateEvent::Unreacted { reaction, user_id } => {
    //!           println!("{:?} removed {:?}", user_id, reaction);
    //!        }
    //!        NoteUpdateEvent::Deleted { deleted_at } => {
    //!           println!("deleted at {:?}", deleted_at);
    //!        }
    //!        NoteUpdateEvent::PollVoted { choice, user_id } => {
    //!           println!("{:?} voted to {}", user_id, choice);
    //!        }
    //!     }
    //! }
    //! # Ok(())
    //! # }
    //! ```
    //!
    //! Monitor newly added emojis:
    //!
    //! ```no_run
    //! use futures::stream::StreamExt;
    //! use misskey::streaming::broadcast::emoji_added::EmojiAddedEvent;
    //! # use misskey::WebSocketClient;
    //! # #[tokio::main]
    //! # async fn main() -> anyhow::Result<()> {
    //! # let client = WebSocketClient::builder("wss://your.instance.example/streaming")
    //! #     .token("API_TOKEN")
    //! #     .connect()
    //! #     .await?;
    //!
    //! // Connect to the broadcast stream.
    //! let mut stream = client.broadcast::<EmojiAddedEvent>().await?;
    //!
    //! loop {
    //!     let emoji = stream.next().await.unwrap()?.emoji;
    //!     println!("Emoji {} is added", emoji.name);
    //! }
    //! # Ok(())
    //! # }
    //! ```

    // Because the `docsrs` cfg flag does not propagate to `misskey-api`, we're dealing with this
    // by specifying `no_inline` as a workaround.
    #[doc(no_inline)]
    pub use misskey_api::streaming::*;
    pub use misskey_core::streaming::*;
}

pub mod model {
    //! Object types used in API.

    pub use misskey_api::model::*;
    pub use misskey_api::{Entity, EntityRef, PaginationItem};
    pub use misskey_core::model::*;
}

pub use misskey_core::streaming::StreamingClient;
pub use misskey_core::{Client, UploadFileClient};

#[cfg(feature = "http-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "http-client")))]
pub mod http {
    //! Asynchronous HTTP-based client.

    pub use misskey_http::*;
}

#[cfg(feature = "http-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "http-client")))]
pub use http::HttpClient;

#[cfg(feature = "websocket-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "websocket-client")))]
pub mod websocket {
    //! Asynchronous WebSocket-based client.
    //!
    //! The underlying async runtime is determined by the feature flags.
    //! The [tokio](https://tokio.rs) runtime is enabled by default. For details, see the [feature flags section](../index.html#feature-flags).

    pub use misskey_websocket::*;
}

#[cfg(feature = "websocket-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "websocket-client")))]
pub use websocket::WebSocketClient;

pub use misskey_util::{builder, pager, Error, TimelineCursor, TimelineRange};
pub use misskey_util::{ClientExt, StreamingClientExt, UploadFileClientExt};

/// Prelude for crates using `misskey-rs`.
///
/// This module provides a set of useful re-exports, including helper traits.
/// The standard usage of this module is to import its entire contents as follows:
///
/// ```
/// use misskey::prelude::*;
/// ```
pub mod prelude {
    pub use crate::Client;
    #[doc(no_inline)]
    pub use crate::ClientExt as _;
    pub use crate::StreamingClient;
    #[doc(no_inline)]
    pub use crate::StreamingClientExt as _;
    pub use crate::UploadFileClient;
    #[doc(no_inline)]
    pub use crate::UploadFileClientExt as _;
}
