//! Streaming API.

use std::convert::Infallible;
use std::fmt::{self, Display};
use std::str::FromStr;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// ID of a subscribing note.
///
/// This unquestionably corresponds to `NoteId` in [misskey-api](https://docs.rs/misskey-api).
/// We have a distinct ID type here because this crate cannot depend on [misskey-api](https://docs.rs/misskey-api) for various reasons.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(transparent)]
pub struct SubNoteId(pub String);

impl FromStr for SubNoteId {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<SubNoteId, Infallible> {
        Ok(SubNoteId(s.to_string()))
    }
}

impl Display for SubNoteId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

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
