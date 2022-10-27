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

#[cfg(feature = "async-std-runtime")]
use async_std::task;
use futures::{
    sink::SinkExt,
    stream::{FusedStream, Stream, StreamExt},
};
use log::{info, warn};
use misskey_core::streaming::SubNoteEvent;
use serde_json::Value;
#[cfg(feature = "tokio-runtime")]
use tokio::task;

pub struct SubNoteInner {
    id: SubNoteId,
    broker_tx: ControlSender,
    response_rx: ResponseStreamReceiver<Value>,
    is_terminated: bool,
}

impl Debug for SubNoteInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SubNoteInner")
            .field("id", &self.id)
            .field("is_terminated", &self.is_terminated)
            .finish()
    }
}

impl Stream for SubNoteInner {
    type Item = Result<Value>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Value>>> {
        if self.is_terminated {
            return Poll::Ready(None);
        }

        self.response_rx.poll_next_unpin(cx)
    }
}

impl FusedStream for SubNoteInner {
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl Drop for SubNoteInner {
    fn drop(&mut self) {
        if self.is_terminated {
            return;
        }

        let mut broker_tx = self.broker_tx.clone();
        let id = self.id.clone();
        task::spawn(async move {
            // If the broker connection is dead, we don't need to unsubscribe anyway
            // because the client can't be used anymore.
            if let Err(e) = broker_tx.send(BrokerControl::UnsubNote { id }).await {
                warn!(
                    "SubNote::unsubscribe failed in Drop::drop (ignored): {:?}",
                    e
                );
            }
        });
    }
}

impl SubNoteInner {
    async fn subscribe(
        id: SubNoteId,
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
    ) -> Result<SubNoteInner> {
        let (response_tx, response_rx) = response_stream_channel(state);
        broker_tx
            .send(BrokerControl::SubNote {
                id: id.clone(),
                sender: response_tx,
            })
            .await?;

        Ok(SubNoteInner {
            id,
            broker_tx,
            response_rx,
            is_terminated: false,
        })
    }

    async fn unsubscribe(&mut self) -> Result<()> {
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

/// Stream for the [`subnote`][`crate::WebSocketClient::subnote`] method.
#[must_use = "streams do nothing unless polled"]
#[derive(Debug)]
pub struct SubNote<E> {
    inner: SubNoteInner,
    _marker: PhantomData<fn() -> E>,
}

impl<E> SubNote<E> {
    pub(crate) async fn subscribe(
        id: SubNoteId,
        broker_tx: ControlSender,
        state: SharedBrokerState,
    ) -> Result<SubNote<E>> {
        SubNoteInner::subscribe(id, broker_tx, state)
            .await
            .map(|inner| SubNote {
                inner,
                _marker: PhantomData,
            })
    }

    /// Stop this subscription.
    ///
    /// After this call, the stream is no longer available (terminated), i.e. [`StreamExt::next`] returns [`None`].
    /// If you call [`unsubscribe`][`SubNote::unsubscribe`] on a terminated stream, it will simply
    /// be ignored (with log message if logging is enabled).
    pub async fn unsubscribe(&mut self) -> Result<()> {
        self.inner.unsubscribe().await
    }
}

impl<E> Stream for SubNote<E>
where
    E: SubNoteEvent,
{
    type Item = Result<E>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<E>>> {
        let result = futures::ready!(self.inner.poll_next_unpin(cx))
            .map(|res| res.and_then(|v| serde_json::from_value(v).map_err(Into::into)));
        Poll::Ready(result)
    }
}

impl<E> FusedStream for SubNote<E>
where
    E: SubNoteEvent,
{
    fn is_terminated(&self) -> bool {
        self.inner.is_terminated()
    }
}
