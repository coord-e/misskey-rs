use futures_core::{future::BoxFuture, stream::Stream};
use futures_sink::Sink;

use crate::streaming::api::{BroadcastEvent, ConnectChannelRequest, SubNoteEvent, SubNoteId};

pub trait ChannelClient<R: ConnectChannelRequest> {
    type Error: std::error::Error;
    type Stream: Stream<Item = Result<R::Incoming, Self::Error>>
        + Sink<R::Outgoing, Error = Self::Error>;

    fn connect<'a>(&mut self, request: R) -> BoxFuture<'a, Result<Self::Stream, Self::Error>>
    where
        R: 'a;
}

pub trait BroadcastClient<M: BroadcastEvent> {
    type Error: std::error::Error;
    type Stream: Stream<Item = Result<M, Self::Error>>;

    fn broadcast(&mut self) -> BoxFuture<'static, Result<Self::Stream, Self::Error>>;
}

pub trait SubNoteClient<E: SubNoteEvent> {
    type Error: std::error::Error;
    type Stream: Stream<Item = Result<E, Self::Error>>;

    fn subscribe_note<I>(&mut self, id: I) -> BoxFuture<'static, Result<Self::Stream, Self::Error>>
    where
        I: Into<SubNoteId>;
}
