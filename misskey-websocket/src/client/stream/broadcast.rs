use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{response_stream_channel, ControlSender, ResponseStreamReceiver},
    model::{BroadcastId, BrokerControl, SharedBrokerState},
};
use crate::error::Result;

use futures::{
    executor,
    stream::{self, FusedStream, Stream, StreamExt},
};
use log::{info, warn};
use misskey_core::streaming::BroadcastMessage;
use serde_json::Value;

type DeserializedResponseStream<T> =
    stream::Map<ResponseStreamReceiver<Value>, fn(Result<Value>) -> Result<T>>;

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Broadcast<M: BroadcastMessage> {
    id: BroadcastId,
    broker_tx: ControlSender,
    response_rx: DeserializedResponseStream<M>,
    is_terminated: bool,
}

impl<M> Broadcast<M>
where
    M: BroadcastMessage,
{
    pub(crate) async fn start(
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
    ) -> Result<Broadcast<M>> {
        let id = BroadcastId::new();

        let (response_tx, response_rx_raw) = response_stream_channel(state);
        broker_tx
            .send(BrokerControl::StartBroadcast {
                id,
                type_: M::TYPE,
                sender: response_tx,
            })
            .await?;

        Ok(Broadcast {
            id,
            broker_tx,
            response_rx: response_rx_raw.map(|r| match r {
                Ok(v) => serde_json::from_value(v).map_err(Into::into),
                Err(e) => Err(e),
            }),
            is_terminated: false,
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

impl<M> Stream for Broadcast<M>
where
    M: BroadcastMessage,
{
    type Item = Result<M>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<M>>> {
        if self.is_terminated {
            return Poll::Ready(None);
        }
        Pin::new(&mut self.response_rx).poll_next(cx)
    }
}

impl<M> FusedStream for Broadcast<M>
where
    M: BroadcastMessage,
{
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl<M> Drop for Broadcast<M>
where
    M: BroadcastMessage,
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
