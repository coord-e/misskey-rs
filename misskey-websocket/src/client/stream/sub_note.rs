use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{response_stream_channel, ControlSender, ResponseStreamReceiver},
    model::{BrokerControl, SharedBrokerState},
};
use crate::error::Result;
use crate::model::SubNoteId;

use futures::{
    executor,
    sink::SinkExt,
    stream::{FusedStream, Stream, StreamExt},
};
use log::{info, warn};
use misskey_core::streaming::SubNoteEvent;
use serde_json::Value;

/// Stream for the [`subnote`][`crate::WebSocketClient::subnote`] method.
#[must_use = "streams do nothing unless polled"]
pub struct SubNote<E> {
    id: SubNoteId,
    broker_tx: ControlSender,
    response_rx: ResponseStreamReceiver<Value>,
    is_terminated: bool,
    _marker: PhantomData<fn() -> E>,
}

impl<E> Debug for SubNote<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SubNote")
            .field("id", &self.id)
            .field("is_terminated", &self.is_terminated)
            .finish()
    }
}

impl<E> SubNote<E> {
    pub(crate) async fn subscribe(
        id: SubNoteId,
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
    ) -> Result<SubNote<E>> {
        let (response_tx, response_rx) = response_stream_channel(state);
        broker_tx
            .send(BrokerControl::SubNote {
                id: id.clone(),
                sender: response_tx,
            })
            .await?;

        Ok(SubNote {
            id,
            broker_tx,
            response_rx,
            is_terminated: false,
            _marker: PhantomData,
        })
    }

    /// Stop this subscription.
    ///
    /// After this call, the stream is no longer available (terminated), i.e. [`StreamExt::next`] returns [`None`].
    /// If you call [`unsubscribe`][`SubNote::unsubscribe`] on a terminated stream, it will simply
    /// be ignored (with log message if logging is enabled).
    pub async fn unsubscribe(&mut self) -> Result<()> {
        if self.is_terminated {
            info!("unsubscribing already terminated SubNote, skipping");
            return Ok(());
        }

        self.broker_tx
            .send(BrokerControl::UnsubNote {
                id: self.id.clone(),
            })
            .await?;

        self.is_terminated = true;

        Ok(())
    }
}

impl<E> Stream for SubNote<E>
where
    E: SubNoteEvent,
{
    type Item = Result<E>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<E>>> {
        if self.is_terminated {
            return Poll::Ready(None);
        }

        match futures::ready!(self.response_rx.poll_next_unpin(cx)?) {
            None => Poll::Ready(None),
            Some(v) => Poll::Ready(Some(Ok(serde_json::from_value(v)?))),
        }
    }
}

impl<E> FusedStream for SubNote<E>
where
    E: SubNoteEvent,
{
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl<E> Drop for SubNote<E> {
    fn drop(&mut self) {
        if self.is_terminated {
            return;
        }

        executor::block_on(async {
            // If the broker connection is dead, we don't need to unsubscribe anyway
            // because the client can't be used anymore.
            if let Err(e) = self.unsubscribe().await {
                warn!(
                    "SubNote::unsubscribe failed in Drop::drop (ignored): {:?}",
                    e
                );
            }
        });
    }
}
