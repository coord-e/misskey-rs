use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{response_stream_channel, ControlSender, ResponseStreamReceiver},
    model::{BrokerControl, SharedBrokerState},
};
use crate::channel::SharedWebSocketSender;
use crate::error::Result;
use crate::model::{message::note_updated::NoteUpdateEvent, request::Request};

use futures::{
    executor,
    stream::{FusedStream, Stream},
};
use misskey::model::note::NoteId;

pub struct NoteUpdate {
    id: NoteId,
    broker_tx: ControlSender,
    response_rx: ResponseStreamReceiver<NoteUpdateEvent>,
    websocket_tx: SharedWebSocketSender,
    is_terminated: bool,
}

impl NoteUpdate {
    pub(crate) async fn subscribe(
        note_id: NoteId,
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
        websocket_tx: SharedWebSocketSender,
    ) -> Result<NoteUpdate> {
        let (response_tx, response_rx) = response_stream_channel(state);

        broker_tx
            .send(BrokerControl::SubscribeNote {
                id: note_id.clone(),
                sender: response_tx,
            })
            .await?;

        let req = Request::SubNote {
            id: note_id.clone(),
        };
        websocket_tx.lock().await.send_json(&req).await?;

        Ok(NoteUpdate {
            id: note_id,
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
            .send_json(&Request::UnsubNote {
                id: self.id.clone(),
            })
            .await?;

        self.broker_tx
            .send(BrokerControl::UnsubscribeNote(self.id.clone()))
            .await?;

        self.is_terminated = true;

        Ok(())
    }
}

impl Stream for NoteUpdate {
    type Item = Result<NoteUpdateEvent>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<NoteUpdateEvent>>> {
        if self.is_terminated {
            return Poll::Ready(None);
        }

        Pin::new(&mut self.response_rx).poll_next(cx)
    }
}

impl FusedStream for NoteUpdate {
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl Drop for NoteUpdate {
    fn drop(&mut self) {
        executor::block_on(async {
            // If the broker or websocket connection is dead, we don't need to unsubscribe anyway
            // because the client can't be used anymore.
            let _ = self.unsubscribe().await;
        });
    }
}
