use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::model::SharedBrokerState;
use crate::error::Result;

use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures::future::FutureExt;
use futures::stream::{FusedStream, Stream, StreamExt};

/// Sender channel that broker uses to respond to the client
#[derive(Debug, Clone)]
pub(crate) struct ResponseStreamSender<T>(UnboundedSender<T>);

impl<T> ResponseStreamSender<T> {
    /// `Ok(())` when successfully sent, `Err(t)` when the channel is closed
    pub fn try_send(&mut self, t: T) -> std::result::Result<(), T> {
        self.0.unbounded_send(t).map_err(|e| e.into_inner())
    }
}

/// Receiver channel that the client uses to receive the response from broker
#[derive(Debug)]
pub(crate) struct ResponseStreamReceiver<T> {
    inner: UnboundedReceiver<T>,
    state: SharedBrokerState,
    is_terminated: bool,
}

impl<T> Stream for ResponseStreamReceiver<T> {
    type Item = Result<T>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<T>>> {
        if self.is_terminated {
            return Poll::Ready(None);
        }

        let state = {
            let read_fut = self.state.read();
            futures::pin_mut!(read_fut);
            match read_fut.poll_unpin(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(lock) => lock.dead().cloned(),
            }
        };

        if let Some(err) = state {
            self.is_terminated = true;
            Poll::Ready(Some(Err(err)))
        } else {
            self.inner.poll_next_unpin(cx).map(|x| x.map(Ok))
        }
    }
}

impl<T> FusedStream for ResponseStreamReceiver<T> {
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

pub(crate) fn response_stream_channel<T>(
    state: SharedBrokerState,
) -> (ResponseStreamSender<T>, ResponseStreamReceiver<T>) {
    let (sender, receiver) = mpsc::unbounded();
    (
        ResponseStreamSender(sender),
        ResponseStreamReceiver {
            inner: receiver,
            is_terminated: false,
            state,
        },
    )
}
