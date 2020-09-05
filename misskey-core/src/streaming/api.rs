use derive_more::{Display, FromStr};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct SubNoteId(pub String);

pub trait ConnectChannelRequest: Serialize {
    type Incoming: DeserializeOwned + 'static;
    type Outgoing: Serialize;

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

pub trait SubNoteEvent: DeserializeOwned + 'static {}

pub trait BroadcastEvent: DeserializeOwned + 'static {
    const TYPE: &'static str;
}
