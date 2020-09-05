use std::collections::VecDeque;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{
        channel_pong_channel, response_stream_channel, ControlSender, ResponseStreamReceiver,
    },
    model::{BrokerControl, SharedBrokerState},
};
use crate::channel::SharedWebSocketSender;
use crate::error::{Error, Result};
use crate::model::{ChannelId, OutgoingMessage};

use futures::{
    executor,
    future::FutureExt,
    sink::{Sink, SinkExt},
    stream::{FusedStream, Stream, StreamExt},
};
use log::{info, warn};
use misskey_core::streaming::ConnectChannelRequest;
use serde_json::Value;

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Channel<R> {
    id: ChannelId,
    broker_tx: ControlSender,
    response_rx: ResponseStreamReceiver<Value>,
    websocket_tx: SharedWebSocketSender,
    is_terminated: bool,
    sink_buffer: VecDeque<OutgoingMessage>,
    _marker: PhantomData<fn() -> R>,
}

impl<R> Channel<R>
where
    R: ConnectChannelRequest,
{
    pub(crate) async fn connect(
        req: R,
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
        websocket_tx: SharedWebSocketSender,
    ) -> Result<Channel<R>> {
        let id = ChannelId::uuid();

        let (response_tx, response_rx) = response_stream_channel(Arc::clone(&state));
        let (pong_tx, pong_rx) = channel_pong_channel(state);

        broker_tx
            .send(BrokerControl::Connect {
                id,
                name: R::NAME,
                sender: response_tx,
                pong: pong_tx,
            })
            .await?;

        websocket_tx
            .lock()
            .await
            .send(OutgoingMessage::Connect {
                channel: R::NAME,
                id,
                params: serde_json::to_value(req)?,
                pong: true,
            })
            .await?;

        // wait for `connected` pong message from server
        pong_rx.recv().await?;

        Ok(Channel {
            id,
            broker_tx,
            response_rx,
            websocket_tx,
            is_terminated: false,
            sink_buffer: VecDeque::new(),
            _marker: PhantomData,
        })
    }
}

impl<R> Channel<R> {
    pub async fn disconnect(&mut self) -> Result<()> {
        if self.is_terminated {
            info!("disconnecting from already terminated Channel, skipping");
            return Ok(());
        }

        self.broker_tx
            .send(BrokerControl::Disconnect { id: self.id })
            .await?;

        self.websocket_tx
            .lock()
            .await
            .send(OutgoingMessage::Disconnect { id: self.id })
            .await?;

        self.is_terminated = true;

        Ok(())
    }
}

impl<R> Stream for Channel<R>
where
    R: ConnectChannelRequest,
{
    type Item = Result<R::Incoming>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<R::Incoming>>> {
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

impl<R> Sink<R::Outgoing> for Channel<R>
where
    R: ConnectChannelRequest,
    R::Outgoing: Unpin,
{
    type Error = Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        match self.websocket_tx.lock().poll_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(mut lock) => lock.poll_ready_unpin(cx),
        }
    }

    fn start_send(self: Pin<&mut Self>, item: R::Outgoing) -> Result<()> {
        let message = OutgoingMessage::Channel {
            id: self.id,
            message: serde_json::to_value(item)?,
        };

        if let Some(mut lock) = self.websocket_tx.try_lock() {
            return lock.start_send_unpin(message);
        }

        self.get_mut().sink_buffer.push_back(message);
        Ok(())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        let Self {
            websocket_tx,
            sink_buffer,
            ..
        } = &mut *self;

        let mut lock = match websocket_tx.lock().poll_unpin(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(lock) => lock,
        };

        while let Some(message) = sink_buffer.pop_front() {
            lock.start_send_unpin(message)?;
        }

        lock.poll_flush_unpin(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        if !self.sink_buffer.is_empty() {
            match self.poll_flush_unpin(cx) {
                Poll::Ready(Ok(())) => (),
                p => return p,
            }
        }

        assert!(self.sink_buffer.is_empty());

        match self.websocket_tx.lock().poll_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(mut lock) => lock.poll_close_unpin(cx),
        }
    }
}

impl<R> FusedStream for Channel<R>
where
    R: ConnectChannelRequest,
{
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl<R> Drop for Channel<R> {
    fn drop(&mut self) {
        if self.is_terminated {
            return;
        }

        executor::block_on(async {
            // If the broker or websocket connection is dead, we don't need to unsubscribe anyway
            // because the client can't be used anymore.
            if let Err(e) = self.disconnect().await {
                warn!(
                    "Channel::disconnect failed in Drop::drop (ignored): {:?}",
                    e
                );
            }
        });
    }
}
