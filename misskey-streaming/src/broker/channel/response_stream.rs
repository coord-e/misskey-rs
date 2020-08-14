use std::pin::Pin;
use std::task::{Context, Poll};

use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures::stream::{FusedStream, Stream};

/// Sender channel that broker uses to respond to the client
#[derive(Debug, Clone)]
pub struct ResponseStreamSender<T>(UnboundedSender<T>);

impl<T> ResponseStreamSender<T> {
    /// `Ok(())` when successfully sent, `Err(t)` when the channel is closed
    pub fn try_send(&mut self, t: T) -> std::result::Result<(), T> {
        self.0.unbounded_send(t).map_err(|e| e.into_inner())
    }
}

/// Receiver channel that the client uses to receive the response from broker
/// The channel is expected to be alive through the lifetime of `ResponseStreamReceiver`
#[derive(Debug)]
pub struct ResponseStreamReceiver<T>(UnboundedReceiver<T>);

impl<T> Stream for ResponseStreamReceiver<T> {
    type Item = T;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

impl<T> FusedStream for ResponseStreamReceiver<T> {
    fn is_terminated(&self) -> bool {
        self.0.is_terminated()
    }
}

pub fn response_stream_channel<T>() -> (ResponseStreamSender<T>, ResponseStreamReceiver<T>) {
    let (sender, receiver) = mpsc::unbounded();
    (
        ResponseStreamSender(sender),
        ResponseStreamReceiver(receiver),
    )
}
