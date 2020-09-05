use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{response_stream_channel, ControlSender, ResponseStreamReceiver},
    model::{BrokerControl, SharedBrokerState},
};
use crate::channel::SharedWebSocketSender;
use crate::error::Result;
use crate::model::outgoing::OutgoingMessage;

use futures::{
    executor,
    sink::SinkExt,
    stream::{FusedStream, Stream, StreamExt},
};
use log::{info, warn};
use misskey_core::streaming::{SubNoteEvent, SubNoteId};
use serde_json::Value;

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct SubNote<E> {
    id: SubNoteId,
    broker_tx: ControlSender,
    response_rx: ResponseStreamReceiver<Value>,
    websocket_tx: SharedWebSocketSender,
    is_terminated: bool,
    _marker: PhantomData<fn() -> E>,
}

impl<E> SubNote<E> {
    pub(crate) async fn subscribe(
        id: SubNoteId,
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
        websocket_tx: SharedWebSocketSender,
    ) -> Result<SubNote<E>> {
        let (response_tx, response_rx) = response_stream_channel(state);
        broker_tx
            .send(BrokerControl::SubNote {
                id: id.clone(),
                sender: response_tx,
            })
            .await?;

        websocket_tx
            .lock()
            .await
            .send(OutgoingMessage::SubNote { id: id.clone() })
            .await?;

        Ok(SubNote {
            id,
            broker_tx,
            response_rx,
            websocket_tx,
            is_terminated: false,
            _marker: PhantomData,
        })
    }

    pub async fn unsubscribe(&mut self) -> Result<()> {
        if self.is_terminated {
            info!("unsubscribing already terminated SubNote, skipping");
            return Ok(());
        }

        self.broker_tx
            .send(BrokerControl::UnsubNote {
                id: self.id.clone(),
            })
            .await?;

        self.websocket_tx
            .lock()
            .await
            .send(OutgoingMessage::UnsubNote {
                id: self.id.clone(),
            })
            .await?;

        self.is_terminated = true;

        Ok(())
    }
}

impl<E> Stream for SubNote<E>
where
    E: SubNoteEvent,
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

impl<E> FusedStream for SubNote<E>
where
    E: SubNoteEvent,
{
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl<E> Drop for SubNote<E> {
    fn drop(&mut self) {
        if self.is_terminated {
            return;
        }

        executor::block_on(async {
            // If the broker or websocket connection is dead, we don't need to unsubscribe anyway
            // because the client can't be used anymore.
            if let Err(e) = self.unsubscribe().await {
                warn!(
                    "SubNote::unsubscribe failed in Drop::drop (ignored): {:?}",
                    e
                );
            }
        });
    }
}
