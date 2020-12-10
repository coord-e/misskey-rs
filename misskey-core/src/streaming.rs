//! Streaming API.

use std::pin::Pin;

use futures_core::{
    future::BoxFuture,
    stream::{BoxStream, Stream},
};
use futures_sink::Sink;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Trait for [`Stream`] + [`Sink`].
///
/// We need this for [`BoxStreamSink`] because trait objects can only have a single base trait. ([reference])
///
/// [reference]: https://doc.rust-lang.org/reference/types/trait-object.html
pub trait StreamSink<I, O, E>: Stream<Item = Result<I, E>> + Sink<O, Error = E> {}
impl<I, O, E, S: ?Sized> StreamSink<I, O, E> for S where
    S: Sink<O, Error = E> + Stream<Item = Result<I, E>>
{
}

/// An owned dynamically typed [`Stream`] + [`Sink`] for use in cases where we can't statically
/// type the result.
pub type BoxStreamSink<'a, I, O, E> = Pin<Box<dyn StreamSink<I, O, E> + 'a + Send>>;

/// Stream for the [`StreamingClient::subnote`] method.
pub type SubNoteStream<'a, T, E> = BoxStream<'a, Result<T, E>>;
/// Stream for the [`StreamingClient::channel`] method.
pub type ChannelStream<'a, T, E> = BoxStreamSink<
    'a,
    <T as ConnectChannelRequest>::Incoming,
    <T as ConnectChannelRequest>::Outgoing,
    E,
>;
/// Stream for the [`StreamingClient::broadcast`] method.
pub type BroadcastStream<'a, T, E> = BoxStream<'a, Result<T, E>>;

/// Abstraction over API clients with streaming connections.
pub trait StreamingClient {
    /// The error type produced by the client when an error occurs.
    type Error: std::error::Error;

    /// Captures the note specified by `note_id`.
    fn subnote<E: SubNoteEvent>(
        &self,
        note_id: String,
    ) -> BoxFuture<Result<SubNoteStream<E, Self::Error>, Self::Error>>;

    /// Connects to the channel using `request`.
    fn channel<R: ConnectChannelRequest>(
        &self,
        request: R,
    ) -> BoxFuture<Result<ChannelStream<R, Self::Error>, Self::Error>>;

    /// Receive messages from the broadcast stream.
    fn broadcast<E: BroadcastEvent>(
        &self,
    ) -> BoxFuture<Result<BroadcastStream<E, Self::Error>, Self::Error>>;
}

impl<C: ?Sized> StreamingClient for &C
where
    C: StreamingClient,
{
    type Error = C::Error;

    fn subnote<E: SubNoteEvent>(
        &self,
        note_id: String,
    ) -> BoxFuture<Result<SubNoteStream<E, Self::Error>, Self::Error>> {
        C::subnote(self, note_id)
    }

    fn channel<R: ConnectChannelRequest>(
        &self,
        request: R,
    ) -> BoxFuture<Result<ChannelStream<R, Self::Error>, Self::Error>> {
        C::channel(self, request)
    }

    fn broadcast<E: BroadcastEvent>(
        &self,
    ) -> BoxFuture<Result<BroadcastStream<E, Self::Error>, Self::Error>> {
        C::broadcast(self)
    }
}

impl<C: ?Sized> StreamingClient for &mut C
where
    C: StreamingClient,
{
    type Error = C::Error;

    fn subnote<E: SubNoteEvent>(
        &self,
        note_id: String,
    ) -> BoxFuture<Result<SubNoteStream<E, Self::Error>, Self::Error>> {
        C::subnote(self, note_id)
    }

    fn channel<R: ConnectChannelRequest>(
        &self,
        request: R,
    ) -> BoxFuture<Result<ChannelStream<R, Self::Error>, Self::Error>> {
        C::channel(self, request)
    }

    fn broadcast<E: BroadcastEvent>(
        &self,
    ) -> BoxFuture<Result<BroadcastStream<E, Self::Error>, Self::Error>> {
        C::broadcast(self)
    }
}

impl<C: ?Sized> StreamingClient for Box<C>
where
    C: StreamingClient,
{
    type Error = C::Error;

    fn subnote<E: SubNoteEvent>(
        &self,
        note_id: String,
    ) -> BoxFuture<Result<SubNoteStream<E, Self::Error>, Self::Error>> {
        C::subnote(self, note_id)
    }

    fn channel<R: ConnectChannelRequest>(
        &self,
        request: R,
    ) -> BoxFuture<Result<ChannelStream<R, Self::Error>, Self::Error>> {
        C::channel(self, request)
    }

    fn broadcast<E: BroadcastEvent>(
        &self,
    ) -> BoxFuture<Result<BroadcastStream<E, Self::Error>, Self::Error>> {
        C::broadcast(self)
    }
}

/// Request to connect to the channel.
///
/// It's similar to `Request` but for connecting to a channel.
pub trait ConnectChannelRequest: Serialize {
    /// Type of the data we receive from the channel.
    type Incoming: DeserializeOwned + 'static;
    /// Type of the data we send to the channel.
    type Outgoing: Serialize + 'static;

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
/// We treat it abstractly here since [misskey-core](https://docs.rs/misskey-core) cannot depend on [misskey-api](https://docs.rs/misskey-api).
pub trait SubNoteEvent: DeserializeOwned + 'static {}

/// Events you receive from broadcast stream.
pub trait BroadcastEvent: DeserializeOwned + 'static {
    /// Name of this event in the broadcast stream.
    const TYPE: &'static str;
}
