use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{response_stream_channel, ControlSender, ResponseStreamReceiver},
    model::{BrokerControl, SharedBrokerState},
};
use crate::channel::SharedWebSocketSender;
use crate::error::Result;
use crate::model::{
    request::{ConnectChannel, Request, Timeline},
    ChannelId,
};

use futures::{
    executor,
    stream::{FusedStream, Stream},
};
use misskey_api::model::note::Note;

#[must_use = "streams do nothing unless polled"]
pub struct TimelineStream {
    id: ChannelId,
    broker_tx: ControlSender,
    response_rx: ResponseStreamReceiver<Note>,
    websocket_tx: SharedWebSocketSender,
    is_terminated: bool,
}

impl TimelineStream {
    pub(crate) async fn subscribe(
        timeline: Timeline,
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
        websocket_tx: SharedWebSocketSender,
    ) -> Result<TimelineStream> {
        let id = ChannelId::new();

        let (response_tx, response_rx) = response_stream_channel(state);
        broker_tx
            .send(BrokerControl::SubscribeTimeline {
                id,
                sender: response_tx,
            })
            .await?;

        let req = Request::Connect {
            id,
            channel: ConnectChannel::Timeline(timeline),
        };
        websocket_tx.lock().await.send_json(&req).await?;

        Ok(TimelineStream {
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

impl Stream for TimelineStream {
    type Item = Result<Note>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Note>>> {
        if self.is_terminated {
            return Poll::Ready(None);
        }

        Pin::new(&mut self.response_rx).poll_next(cx)
    }
}

impl FusedStream for TimelineStream {
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl Drop for TimelineStream {
    fn drop(&mut self) {
        executor::block_on(async {
            // If the broker or websocket connection is dead, we don't need to unsubscribe anyway
            // because the client can't be used anymore.
            let _ = self.unsubscribe().await;
        });
    }
}
