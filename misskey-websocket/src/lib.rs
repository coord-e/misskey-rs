mod broker;
mod channel;
mod client;
pub mod error;
mod model;

pub use client::{builder::WebSocketClientBuilder, stream, WebSocketClient};
pub use model::{
    message::{channel::MainStreamEvent, note_updated::NoteUpdateEvent},
    request::TimelineType,
};
