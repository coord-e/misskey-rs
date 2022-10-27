use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{response_stream_channel, ControlSender, ResponseStreamReceiver},
    model::{BroadcastId, BrokerControl, SharedBrokerState},
};
use crate::error::Result;

#[cfg(feature = "async-std-runtime")]
use async_std::task;
use futures::{
    sink::SinkExt,
    stream::{FusedStream, Stream, StreamExt},
};
use log::{info, warn};
use misskey_core::streaming::BroadcastEvent;
use serde_json::Value;
#[cfg(feature = "tokio-runtime")]
use tokio::task;

struct BroadcastInner {
    id: BroadcastId,
    broker_tx: ControlSender,
    response_rx: ResponseStreamReceiver<Value>,
    is_terminated: bool,
}

impl Debug for BroadcastInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BroadcastInner")
            .field("id", &self.id)
            .field("is_terminated", &self.is_terminated)
            .finish()
    }
}

impl Stream for BroadcastInner {
    type Item = Result<Value>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Value>>> {
        if self.is_terminated {
            return Poll::Ready(None);
        }

        self.response_rx.poll_next_unpin(cx)
    }
}

impl FusedStream for BroadcastInner {
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl Drop for BroadcastInner {
    fn drop(&mut self) {
        if self.is_terminated {
            return;
        }

        let mut broker_tx = self.broker_tx.clone();
        let id = self.id;
        task::spawn(async move {
            // If the broker connection is dead, we don't need to stop this anyway
            // because the client can't be used anymore.
            if let Err(e) = broker_tx.send(BrokerControl::StopBroadcast { id }).await {
                warn!("Broadcast::stop failed in Drop::drop (ignored): {:?}", e);
            }
        });
    }
}

impl BroadcastInner {
    async fn start(
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
        type_: &'static str,
    ) -> Result<BroadcastInner> {
        let id = BroadcastId::new();

        let (response_tx, response_rx) = response_stream_channel(state);
        broker_tx
            .send(BrokerControl::StartBroadcast {
                id,
                type_,
                sender: response_tx,
            })
            .await?;

        Ok(BroadcastInner {
            id,
            broker_tx,
            response_rx,
            is_terminated: false,
        })
    }

    async fn stop(&mut self) -> Result<()> {
        if self.is_terminated {
            info!("stopping already terminated Broadcast, skipping");
            return Ok(());
        }

        self.broker_tx
            .send(BrokerControl::StopBroadcast { id: self.id })
            .await?;

        self.is_terminated = true;

        Ok(())
    }
}

/// Stream for the [`broadcast`][`crate::WebSocketClient::broadcast`] method.
#[must_use = "streams do nothing unless polled"]
#[derive(Debug)]
pub struct Broadcast<E> {
    inner: BroadcastInner,
    _marker: PhantomData<fn() -> E>,
}

impl<E> Broadcast<E>
where
    E: BroadcastEvent,
{
    pub(crate) async fn start(
        broker_tx: ControlSender,
        state: SharedBrokerState,
    ) -> Result<Broadcast<E>> {
        BroadcastInner::start(broker_tx, state, E::TYPE)
            .await
            .map(|inner| Broadcast {
                inner,
                _marker: PhantomData,
            })
    }
}

impl<E> Broadcast<E> {
    /// Stop this subscription.
    ///
    /// After this call, the stream is no longer available (terminated), i.e. [`StreamExt::next`] returns [`None`].
    /// If you call [`stop`][`Broadcast::stop`] on a terminated stream, it will simply
    /// be ignored (with log message if logging is enabled).
    pub async fn stop(&mut self) -> Result<()> {
        self.inner.stop().await
    }
}

impl<E> Stream for Broadcast<E>
where
    E: BroadcastEvent,
{
    type Item = Result<E>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<E>>> {
        let result = futures::ready!(self.inner.poll_next_unpin(cx))
            .map(|res| res.and_then(|v| serde_json::from_value(v).map_err(Into::into)));
        Poll::Ready(result)
    }
}

impl<E> FusedStream for Broadcast<E>
where
    E: BroadcastEvent,
{
    fn is_terminated(&self) -> bool {
        self.inner.is_terminated()
    }
}
