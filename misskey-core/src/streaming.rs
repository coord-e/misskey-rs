//! Streaming API.

use derive_more::{Display, FromStr};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// ID of a subscribing note.
///
/// This unquestionably corresponds to `NoteId` in [misskey-api](https://docs.rs/misskey-api).
/// We have a distinct ID type here because this crate cannot depend on [misskey-api](https://docs.rs/misskey-api) for various reasons.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct SubNoteId(pub String);

/// Request to connect to the channel.
///
/// It's similar to [`Request`][`crate::Request`] but for connecting to a channel.
pub trait ConnectChannelRequest: Serialize {
    /// Type of the data we receive from the channel.
    type Incoming: DeserializeOwned + 'static;
    /// Type of the data we send to the channel.
    type Outgoing: Serialize;

    /// The name of the channel to be connected by this request.
    const NAME: &'static str;
}

impl<R: ?Sized> ConnectChannelRequest for &'_ R
where
    R: ConnectChannelRequest,
{
    type Incoming = R::Incoming;
    type Outgoing = R::Outgoing;

    const NAME: &'static str = R::NAME;
}

impl<R: ?Sized> ConnectChannelRequest for &'_ mut R
where
    R: ConnectChannelRequest,
{
    type Incoming = R::Incoming;
    type Outgoing = R::Outgoing;

    const NAME: &'static str = R::NAME;
}

impl<R: ?Sized> ConnectChannelRequest for Box<R>
where
    R: ConnectChannelRequest,
{
    type Incoming = R::Incoming;
    type Outgoing = R::Outgoing;

    const NAME: &'static str = R::NAME;
}

/// Events you receive with a subscription to the note.
///
/// This unquestionably corresponds to `NoteUpdateEvent` in [misskey-api](https://docs.rs/misskey-api).
/// We treat it abstractly here since this crate cannot depend on [misskey-api](https://docs.rs/misskey-api).
pub trait SubNoteEvent: DeserializeOwned + 'static {}

/// Events you receive from broadcast stream.
pub trait BroadcastEvent: DeserializeOwned + 'static {
    /// Name of this event in the broadcast stream.
    const TYPE: &'static str;
}
