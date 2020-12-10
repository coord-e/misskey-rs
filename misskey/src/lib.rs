//! Asynchronous client for [Misskey](https://github.com/syuilo/misskey).
//!
//! We provide three components in this crate:
//!
//! - Clients that handles the connection between Misskey. As Misskey provides HTTP and WebSocket
//!   interfaces to interact with, we have [`HttpClient`] and [`WebSocketClient`] implementations
//!   correspondingly.
//! - API bindings, including requests/responses of [endpoints][`endpoint`] and messages on
//!   [channels][`streaming::channel`].
//! - Abstraction that connects API datatypes and client implementations: [`Request`][`endpoint::Request`],
//!   [`ConnectChannelRequest`][`streaming::ConnectChannelRequest`], etc.
//!
//! # Examples
//!
//! Create a note:
//!
//! ```no_run
//! use misskey::{Client, HttpClient};
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! let client = HttpClient::builder("https://your.instance.example/api/".parse()?)
//!     .token("API_TOKEN".to_string())
//!     .build()?;
//!
//! client
//!     .request(
//!         // Each endpoint implementation has a corresponding `Request` type.
//!         // We can dispatch an API call by passing `Request` to `Client::request` method.
//!         misskey::endpoint::notes::create::Request::builder()
//!             .text("Hello, Misskey")
//!             .build(),
//!     )
//!     .await?
//!     .into_result()?;
//! # Ok(())
//! # }
//! ```
//!
//! Automatically follow-back:
//!
//! ```no_run
//! use futures::stream::StreamExt;
//! use misskey::streaming::channel::main::{self, MainStreamEvent};
//! use misskey::{Client, WebSocketClient};
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! let client = WebSocketClient::builder("wss://your.instance.example/streaming".parse()?)
//!     .token("YOUR_API_TOKEN")
//!     .connect()
//!     .await?;
//!
//! // Connect to the main stream.
//! // The main stream is a channel that streams events about the connected account.
//! let mut stream = client.channel(main::Request::default()).await?;
//!
//! loop {
//!     // Wait for the next event using `next` method from `StreamExt`.
//!     let event = stream.next().await.unwrap()?;
//!
//!     match event {
//!         MainStreamEvent::Followed(user) if !user.is_bot => {
//!             println!("followed from @{}", user.username);
//!
//!             client
//!                 .request(misskey::endpoint::following::create::Request { user_id: user.id })
//!                 .await?
//!                 .into_result()?;
//!         }
//!         _ => {}
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! See the [example](https://github.com/coord-e/misskey-rs/tree/develop/example) directory for more examples and detailed explanations.
//!
//! # Feature flags
//!
//! - `http-client`: Enables the HTTP client which is capable for uploading files.
//!   Enabled by default.
//! - `websocket-client`: Enables the WebSocket client which is capable for streaming.
//!   Enabled by default.
//! - `tokio-runtime`: Use the [tokio](https://tokio.rs) runtime in the WebSocket client.
//!   Enabled by default.
//! - `async-std-runtime`: Use the [async-std](https://async.rs) runtime in the WebSocket client.
//! - and version flags, as described in [version flags section](#specifying-misskey-version).
//!
//! ## Specifying Misskey version
//!
//! We have a set of feature flags to specify the targeted Misskey version.
//! The latest one (`12-47-0`) is enabled as a default. You can opt-in to compile for prior
//! Misskey version by using `default-features = false` and the corresponding feature flag.
//!
//! For example, to compile for Misskey v12.33.0 with WebSocket client on async-std runtime, add
//! the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies.misskey]
//! version = "0.1"
//! default-features = false
//! features = ["12-31-0", "websocket-client", "async-std-runtime"]
//! ```
//!
//! | Feature                    | Supported Misskey versions (inclusive) | Tested Misskey version |
//! | -------------------------- | -------------------------------------- | ---------------------- |
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
    //! Get your own information from `/api/i`:
    //!
    //! ```no_run
    //! # use misskey::{Client, HttpClient};
    //! # #[tokio::main]
    //! # async fn main() -> anyhow::Result<()> {
    //! # let client = HttpClient::new("http://your.instance.example/api/".parse()?, Some("API_TOKEN".to_string()))?;
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
    //! # let client = WebSocketClient::builder("ws://your.instance.example/streaming".parse()?)
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
    //! # let client = WebSocketClient::builder("ws://your.instance.example/streaming".parse()?)
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
    //! use misskey::streaming::emoji::EmojiAddedEvent;
    //! # use misskey::WebSocketClient;
    //! # #[tokio::main]
    //! # async fn main() -> anyhow::Result<()> {
    //! # let client = WebSocketClient::builder("ws://your.instance.example/streaming".parse()?)
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
    pub use misskey_core::model::*;
}

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
