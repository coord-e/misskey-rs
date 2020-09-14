//! Asynchronous client for [Misskey](https://github.com/syuilo/misskey).
//!
//! We provide three components in this crate:
//!
//! - Clients that handles the connection between Misskey. As Misskey provides HTTP and WebSocket
//!   interfaces to interact with, we have [`HttpClient`] and [`WebSocketClient`] implementations
//!   correspondingly.
//! - API data types, including requests/responses of [endpoints][`endpoint`] and messages on
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
//! let client = HttpClient::new("https://your.misskey.instance/api/".parse()?, Some("API_TOKEN".to_string()))?;
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
//! use misskey::{Client, WebSocketClientBuilder};
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! let client = WebSocketClientBuilder::new("wss://your.misskey.instance/streaming".parse()?)
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
//! The latest one (`12-47-2`) is enabled as a default. You can opt-in to compile for prior
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
//! | Feature                    | Supported Misskey versions (inclusive) |
//! | -------------------------- | -------------------------------------- |
//! | `12-47-2`                  | v12.47.2 ~                             |
//! | `12-47-0`                  | v12.47.0 ~ v12.47.1                    |
//! | `12-39-0`                  | v12.39.0 ~ v12.46.0                    |
//! | `12-37-0`                  | v12.37.0 ~ v12.38.1                    |
//! | `12-31-0`                  | v12.31.0 ~ v12.36.1                    |
//! | `12-29-0`                  | v12.29.0 ~ v12.30.0                    |
//! | `12-28-0`                  | v12.28.0                               |
//! | `12-27-0`                  | v12.27.0 ~ v12.27.1                    |
//! | `12-19-0`                  | v12.19.0 ~ v12.26.0                    |
//! | `12-13-0`                  | v12.13.0 ~ v12.18.1                    |
//! | `12-10-0`                  | v12.10.0 ~ v12.12.0                    |
//! | `12-9-0`                   | v12.9.0                                |
//! | `12-8-0`                   | v12.8.0                                |
//! | `12-5-0`                   | v12.5.0 ~ v12.7.1                      |
//! | (no version flag enabled)  | v12.0.0 ~ v12.4.1                      |
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
    //! # let client = HttpClient::new("http://your.misskey.instance/api/".parse()?, Some("API_TOKEN".to_string()))?;
    //!     let me = client
    //!         .request(misskey::endpoint::i::Request::default())
    //!         .await?
    //!         .into_result()?;
    //!     println!("{:?}", me);
    //! # Ok(())
    //! # }
    //! ```

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
    //! # use misskey::WebSocketClientBuilder;
    //! # #[tokio::main]
    //! # async fn main() -> anyhow::Result<()> {
    //! # let client = WebSocketClientBuilder::new("ws://your.misskey.instance/streaming".parse()?)
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
    //! use misskey::model::note::NoteId;
    //! use misskey::streaming::note::NoteUpdateEvent;
    //! # use misskey::WebSocketClientBuilder;
    //! # #[tokio::main]
    //! # async fn main() -> anyhow::Result<()> {
    //! # let client = WebSocketClientBuilder::new("ws://your.misskey.instance/streaming".parse()?)
    //! #     .token("API_TOKEN")
    //! #     .connect()
    //! #     .await?;
    //!
    //! let note_id = NoteId("NOTE_ID_TO_WATCH".to_string());
    //! let mut stream = client.subscribe_note(note_id).await?;
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
    //! # use misskey::WebSocketClientBuilder;
    //! # #[tokio::main]
    //! # async fn main() -> anyhow::Result<()> {
    //! # let client = WebSocketClientBuilder::new("ws://your.misskey.instance/streaming".parse()?)
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

    pub use misskey_api::streaming::*;
    pub use misskey_core::streaming::*;
}

pub mod model {
    //! Object types used in API.

    pub use misskey_api::model::*;
    pub use misskey_core::model::*;
}

pub use misskey_core::Client;

#[cfg(feature = "http-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "http-client")))]
pub mod http {
    //! Asynchronous HTTP-based client.

    pub use misskey_http::*;
}

#[cfg(feature = "http-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "http-client")))]
pub use http::{HttpClient, HttpClientBuilder};

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
pub use websocket::{WebSocketClient, WebSocketClientBuilder};
