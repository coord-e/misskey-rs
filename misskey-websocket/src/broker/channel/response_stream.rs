use std::fmt::{self, Debug};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::model::{ReadBrokerState, SharedBrokerState};
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
pub(crate) struct ResponseStreamReceiver<T> {
    inner: UnboundedReceiver<T>,
    state: SharedBrokerState,
    is_terminated: bool,
    /// when `state_read_fut` is `Some(_)`, this stream is in terminating phase
    state_read_fut: Option<ReadBrokerState>,
}

impl<T> Debug for ResponseStreamReceiver<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ResponseStreamReceiver")
            .field("is_terminated", &self.is_terminated)
            .field("is_terminating", &self.state_read_fut.is_some())
            .finish()
    }
}

impl<T> Stream for ResponseStreamReceiver<T> {
    type Item = Result<T>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<T>>> {
        if self.is_terminated {
            return Poll::Ready(None);
        }

        if self.state_read_fut.is_none() {
            if let Some(x) = futures::ready!(self.inner.poll_next_unpin(cx)) {
                return Poll::Ready(Some(Ok(x)));
            }
        }

        // broker is unavailable, so...
        let fut = {
            let ResponseStreamReceiver {
                state_read_fut,
                state,
                ..
            } = &mut *self;
            state_read_fut.get_or_insert_with(|| state.read())
        };

        let state = futures::ready!(fut.poll_unpin(cx));
        let err = state
            .dead()
            .expect("broker must be dead after poll_next returned None (ResponseStreamReceiver)");

        self.is_terminated = true;
        Poll::Ready(Some(Err(err)))
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
            state_read_fut: None,
            state,
        },
    )
}
