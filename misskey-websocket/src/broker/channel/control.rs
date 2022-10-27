use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::model::{BrokerControl, SharedBrokerState};
use crate::error::{Error, Result};

use futures_channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures_util::sink::{Sink, SinkExt};
use futures_util::stream::{FusedStream, Stream, StreamExt};

/// Sender channel that the client uses to communicate with broker
#[derive(Debug, Clone)]
pub(crate) struct ControlSender {
    inner: UnboundedSender<BrokerControl>,
    state: SharedBrokerState,
}

impl ControlSender {
    /// obtain `Error` from shared state after broker is dead (incompletely witnessed by `SendError`)
    fn to_error(&self, _witness: &mpsc::SendError) -> Error {
        let state = self
            .state
            .try_read()
            .expect("broker state must be unlocked after broker is dead");
        state
            .dead()
            .expect("broker control channel unexpectedly closed")
    }
}

impl Sink<BrokerControl> for ControlSender {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        self.inner
            .poll_ready_unpin(cx)
            .map_err(|e| self.to_error(&e))
    }

    fn start_send(mut self: Pin<&mut Self>, item: BrokerControl) -> Result<()> {
        self.inner
            .start_send_unpin(item)
            .map_err(|e| self.to_error(&e))
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        self.inner
            .poll_flush_unpin(cx)
            .map_err(|e| self.to_error(&e))
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        self.inner
            .poll_close_unpin(cx)
            .map_err(|e| self.to_error(&e))
    }
}

/// Receiver channel that broker uses to communicate with the client
#[derive(Debug)]
pub(crate) struct ControlReceiver(UnboundedReceiver<BrokerControl>);

impl ControlReceiver {
    // returns `None` when either no message is available or all senders are closed
    pub fn try_recv(&mut self) -> Option<BrokerControl> {
        if self.0.is_terminated() {
            return None;
        }

        match self.0.try_next() {
            Ok(Some(x)) => Some(x),
            // all control senders are dropped
            Ok(None) => None,
            Err(_) => None,
        }
    }
}

impl Stream for ControlReceiver {
    type Item = BrokerControl;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<BrokerControl>> {
        // always return `None` when terminated
        if self.0.is_terminated() {
            Poll::Ready(None)
        } else {
            self.0.poll_next_unpin(cx)
        }
    }
}

impl FusedStream for ControlReceiver {
    fn is_terminated(&self) -> bool {
        self.0.is_terminated()
    }
}

pub(crate) fn control_channel(state: SharedBrokerState) -> (ControlSender, ControlReceiver) {
    let (sender, receiver) = mpsc::unbounded();
    (
        ControlSender {
            inner: sender,
            state,
        },
        ControlReceiver(receiver),
    )
}
