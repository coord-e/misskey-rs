use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{response_stream_channel, ControlSender, ResponseStreamReceiver},
    model::{BrokerControl, SharedBrokerState},
};
use crate::channel::SharedWebSocketSender;
use crate::error::Result;

use futures::{
    executor,
    stream::{self, FusedStream, Stream, StreamExt},
};
use log::{info, warn};
use misskey_core::streaming::{SubscribeRequest, SubscriptionId, UnsubscribeRequest};
use serde_json::Value;

type DeserializedResponseStream<T> =
    stream::Map<ResponseStreamReceiver<Value>, fn(Result<Value>) -> Result<T>>;

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Subscription<R: SubscribeRequest> {
    id: SubscriptionId,
    unsubscribe: R::Unsubscribe,
    broker_tx: ControlSender,
    response_rx: DeserializedResponseStream<R::Content>,
    websocket_tx: SharedWebSocketSender,
    is_terminated: bool,
}

impl<R> Subscription<R>
where
    R: SubscribeRequest,
{
    pub(crate) async fn subscribe(
        req: R,
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
        websocket_tx: SharedWebSocketSender,
    ) -> Result<Subscription<R>> {
        let id = req.id().clone().into();
        let unsubscribe = <R::Unsubscribe as UnsubscribeRequest>::from_id(req.id().clone());

        let (response_tx, response_rx_raw) = response_stream_channel(state);
        broker_tx
            .send(BrokerControl::Subscribe {
                id: id.clone(),
                type_: R::TYPE,
                sender: response_tx,
            })
            .await?;

        websocket_tx.lock().await.send_request(req).await?;

        Ok(Subscription {
            id,
            unsubscribe,
            broker_tx,
            response_rx: response_rx_raw.map(|r| match r {
                Ok(v) => serde_json::from_value(v).map_err(Into::into),
                Err(e) => Err(e),
            }),
            websocket_tx,
            is_terminated: false,
        })
    }

    pub async fn unsubscribe(&mut self) -> Result<()> {
        if self.is_terminated {
            info!("unsubscribing already terminated Subscription, skipping");
            return Ok(());
        }

        self.broker_tx
            .send(BrokerControl::Unsubscribe {
                id: self.id.clone(),
            })
            .await?;

        self.websocket_tx
            .lock()
            .await
            .send_request(&self.unsubscribe)
            .await?;

        self.is_terminated = true;

        Ok(())
    }
}

impl<R> Stream for Subscription<R>
where
    R: SubscribeRequest,
    R::Unsubscribe: Unpin,
{
    type Item = Result<R::Content>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<R::Content>>> {
        if self.is_terminated {
            return Poll::Ready(None);
        }

        Pin::new(&mut self.response_rx).poll_next(cx)
    }
}

impl<R> FusedStream for Subscription<R>
where
    R: SubscribeRequest,
    R::Unsubscribe: Unpin,
{
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl<R> Drop for Subscription<R>
where
    R: SubscribeRequest,
{
    fn drop(&mut self) {
        if self.is_terminated {
            return;
        }

        executor::block_on(async {
            // If the broker or websocket connection is dead, we don't need to unsubscribe anyway
            // because the client can't be used anymore.
            if let Err(e) = self.unsubscribe().await {
                warn!(
                    "Subscription::unsubscribe failed in Drop::drop (ignored): {:?}",
                    e
                );
            }
        });
    }
}
