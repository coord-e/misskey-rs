use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{response_stream_channel, ControlSender, ResponseStreamReceiver},
    model::{BroadcastId, BrokerControl, SharedBrokerState},
};
use crate::error::Result;

use futures::{
    executor,
    sink::SinkExt,
    stream::{FusedStream, Stream, StreamExt},
};
use log::{info, warn};
use misskey_core::streaming::BroadcastEvent;
use serde_json::Value;

/// Stream for the [`broadcast`][`crate::WebSocketClient::broadcast`] method.
#[must_use = "streams do nothing unless polled"]
pub struct Broadcast<E> {
    id: BroadcastId,
    broker_tx: ControlSender,
    response_rx: ResponseStreamReceiver<Value>,
    is_terminated: bool,
    _marker: PhantomData<fn() -> E>,
}

impl<E> Debug for Broadcast<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Broadcast")
            .field("id", &self.id)
            .field("is_terminated", &self.is_terminated)
            .finish()
    }
}

impl<E> Broadcast<E>
where
    E: BroadcastEvent,
{
    pub(crate) async fn start(
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
    ) -> Result<Broadcast<E>> {
        let id = BroadcastId::new();

        let (response_tx, response_rx) = response_stream_channel(state);
        broker_tx
            .send(BrokerControl::StartBroadcast {
                id,
                type_: E::TYPE,
                sender: response_tx,
            })
            .await?;

        Ok(Broadcast {
            id,
            broker_tx,
            response_rx,
            is_terminated: false,
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

impl<E> Stream for Broadcast<E>
where
    E: BroadcastEvent,
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

impl<E> FusedStream for Broadcast<E>
where
    E: BroadcastEvent,
{
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl<E> Drop for Broadcast<E> {
    fn drop(&mut self) {
        if self.is_terminated {
            return;
        }

        executor::block_on(async {
            // If the broker connection is dead, we don't need to stop this anyway
            // because the client can't be used anymore.
            if let Err(e) = self.stop().await {
                warn!("Broadcast::stop failed in Drop::drop (ignored): {:?}", e);
            }
        });
    }
}
