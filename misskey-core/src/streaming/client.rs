use futures_core::stream::Stream;
use futures_sink::Sink;

use crate::streaming::api::{BroadcastEvent, ConnectChannelRequest, SubNoteEvent, SubNoteId};

#[async_trait::async_trait]
pub trait ChannelClient<R: ConnectChannelRequest + Send> {
    type Error: std::error::Error;
    type Stream: Stream<Item = Result<R::Incoming, Self::Error>>
        + Sink<R::Outgoing, Error = Self::Error>;

    async fn connect<'a>(&mut self, request: R) -> Result<Self::Stream, Self::Error>
    where
        R: 'a;
}

#[async_trait::async_trait]
pub trait BroadcastClient<M: BroadcastEvent> {
    type Error: std::error::Error;
    type Stream: Stream<Item = Result<M, Self::Error>>;

    async fn broadcast<'a>(&mut self) -> Result<Self::Stream, Self::Error>
    where
        M: 'a;
}

#[async_trait::async_trait]
pub trait SubNoteClient<E: SubNoteEvent> {
    type Error: std::error::Error;
    type Stream: Stream<Item = Result<E, Self::Error>>;

    async fn subscribe_note<'a, I>(&mut self, id: I) -> Result<Self::Stream, Self::Error>
    where
        I: Into<SubNoteId> + Send,
        E: 'a;
}
