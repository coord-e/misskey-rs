use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{response_stream_channel, ControlSender, ResponseStreamReceiver},
    model::{BrokerControl, SharedBrokerState},
};
use crate::channel::WebSocketSender;
use crate::error::Result;
use crate::model::{
    request::{ConnectChannel, Request, TimelineType},
    ChannelId,
};

use futures::{
    executor,
    stream::{FusedStream, Stream},
};
use misskey::model::note::Note;

pub struct Timeline<'a> {
    id: ChannelId,
    broker_tx: ControlSender,
    response_rx: ResponseStreamReceiver<Note>,
    websocket_tx: &'a mut WebSocketSender,
    is_terminated: bool,
}

impl Timeline<'_> {
    pub(crate) async fn subscribe<'a>(
        timeline: TimelineType,
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
        websocket_tx: &'a mut WebSocketSender,
    ) -> Result<Timeline<'a>> {
        let id = ChannelId::new();

        let (response_tx, response_rx) = response_stream_channel(state);
        broker_tx
            .send(BrokerControl::SubscribeTimeline {
                id: id.clone(),
                sender: response_tx,
            })
            .await?;

        let req = Request::Connect {
            id: id.clone(),
            channel: ConnectChannel::Timeline(timeline),
        };
        websocket_tx.send_json(&req).await?;

        Ok(Timeline {
            id,
            broker_tx,
            response_rx,
            websocket_tx,
            is_terminated: false,
        })
    }

    pub async fn unsubscribe(&mut self) -> Result<()> {
        self.websocket_tx
            .send_json(&Request::Disconnect {
                id: self.id.clone(),
            })
            .await?;

        self.broker_tx
            .send(BrokerControl::UnsubscribeChannel(self.id.clone()))
            .await?;

        self.is_terminated = true;

        Ok(())
    }
}

impl Stream for Timeline<'_> {
    type Item = Result<Note>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Note>>> {
        if self.is_terminated {
            return Poll::Ready(None);
        }

        Pin::new(&mut self.response_rx).poll_next(cx)
    }
}

impl FusedStream for Timeline<'_> {
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl Drop for Timeline<'_> {
    fn drop(&mut self) {
        executor::block_on(async {
            // If the broker or websocket connection is dead, we don't need to unsubscribe anyway
            // because the client can't be used anymore.
            let _ = self.unsubscribe().await;
        });
    }
}
