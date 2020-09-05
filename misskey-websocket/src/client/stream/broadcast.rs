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
    stream::{FusedStream, Stream, StreamExt},
};
use log::{info, warn};
use misskey_core::streaming::BroadcastEvent;
use serde_json::Value;

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Broadcast<E: BroadcastEvent> {
    id: BroadcastId,
    broker_tx: ControlSender,
    response_rx: ResponseStreamReceiver<Value>,
    is_terminated: bool,
    _marker: PhantomData<fn() -> E>,
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

    pub async fn stop(&mut self) -> Result<()> {
        if self.is_terminated() {
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

        match self.response_rx.poll_next_unpin(cx)? {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(serde_json::from_value(v)?))),
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

impl<E> Drop for Broadcast<E>
where
    E: BroadcastEvent,
{
    fn drop(&mut self) {
        if self.is_terminated() {
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
