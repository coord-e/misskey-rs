use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{response_stream_channel, ControlSender, ResponseStreamReceiver},
    model::{BrokerControl, SharedBrokerState},
};
use crate::channel::SharedWebSocketSender;
use crate::error::Result;
use crate::model::{
    message::channel::MainStreamEvent,
    request::{ConnectChannel, Request},
    ChannelId,
};

use futures::{
    executor,
    stream::{FusedStream, Stream},
};

#[must_use = "streams do nothing unless polled"]
pub struct MainStream {
    id: ChannelId,
    broker_tx: ControlSender,
    response_rx: ResponseStreamReceiver<MainStreamEvent>,
    websocket_tx: SharedWebSocketSender,
    is_terminated: bool,
}

impl MainStream {
    pub(crate) async fn subscribe(
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
        websocket_tx: SharedWebSocketSender,
    ) -> Result<MainStream> {
        let id = ChannelId::new();

        let (response_tx, response_rx) = response_stream_channel(state);
        broker_tx
            .send(BrokerControl::SubscribeMainStream {
                id,
                sender: response_tx,
            })
            .await?;

        let req = Request::Connect {
            id,
            channel: ConnectChannel::Main,
        };
        websocket_tx.lock().await.send_json(&req).await?;

        Ok(MainStream {
            id,
            broker_tx,
            response_rx,
            websocket_tx,
            is_terminated: false,
        })
    }

    pub async fn unsubscribe(&mut self) -> Result<()> {
        self.websocket_tx
            .lock()
            .await
            .send_json(&Request::Disconnect { id: self.id })
            .await?;

        self.broker_tx
            .send(BrokerControl::UnsubscribeChannel(self.id))
            .await?;

        self.is_terminated = true;

        Ok(())
    }
}

impl Stream for MainStream {
    type Item = Result<MainStreamEvent>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<MainStreamEvent>>> {
        if self.is_terminated {
            return Poll::Ready(None);
        }

        Pin::new(&mut self.response_rx).poll_next(cx)
    }
}

impl FusedStream for MainStream {
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl Drop for MainStream {
    fn drop(&mut self) {
        executor::block_on(async {
            // If the broker or websocket connection is dead, we don't need to unsubscribe anyway
            // because the client can't be used anymore.
            let _ = self.unsubscribe().await;
        });
    }
}
