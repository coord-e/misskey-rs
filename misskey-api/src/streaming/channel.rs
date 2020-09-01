use std::marker::PhantomData;

use derive_more::{Display, FromStr};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod global_timeline;
pub mod home_timeline;
pub mod hybrid_timeline;
pub mod local_timeline;
pub mod main;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct ChannelId(Uuid);

impl ChannelId {
    fn new() -> Self {
        ChannelId(Uuid::new_v4())
    }
}

pub trait Channel: DeserializeOwned {
    const NAME: &'static str;
}

#[derive(Debug, Clone)]
pub struct ConnectRequest<C> {
    id: ChannelId,
    _marker: PhantomData<fn() -> C>,
}

impl<C> Serialize for ConnectRequest<C>
where
    C: Channel,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("ConnectRequest", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("channel", C::NAME)?;
        state.end()
    }
}

impl<C> ConnectRequest<C> {
    pub fn new() -> Self {
        ConnectRequest {
            id: ChannelId::new(),
            _marker: PhantomData,
        }
    }
}

impl<C> misskey_core::streaming::Request for ConnectRequest<C>
where
    C: Channel,
{
    const TYPE: &'static str = "connect";
}

impl<C> misskey_core::streaming::SubscribeRequest for ConnectRequest<C>
where
    C: Channel,
{
    type Message = ChannelMessage<C>;
    type Unsubscribe = DisconnectRequest;

    type Id = ChannelId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct DisconnectRequest {
    id: ChannelId,
}

impl misskey_core::streaming::Request for DisconnectRequest {
    const TYPE: &'static str = "disconnect";
}

impl misskey_core::streaming::UnsubscribeRequest for DisconnectRequest {
    type Id = ChannelId;
    fn from_id(id: Self::Id) -> Self {
        DisconnectRequest { id }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(bound = "C: Channel")]
pub struct ChannelMessage<C> {
    id: ChannelId,
    #[serde(flatten)]
    message: C,
}

impl<C> misskey_core::streaming::Message for ChannelMessage<C>
where
    C: Channel,
{
    const TYPE: &'static str = "channel";
}

impl<C> misskey_core::streaming::SubscriptionMessage for ChannelMessage<C> where C: Channel {}
