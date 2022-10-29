use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{
        channel_pong_channel, response_stream_channel, ControlSender, ResponseStreamReceiver,
    },
    model::{BrokerControl, SharedBrokerState},
};
use crate::error::{Error, Result};
use crate::model::ChannelId;

#[cfg(feature = "async-std-runtime")]
use async_std::task;
use futures_util::{
    future::BoxFuture,
    sink::{Sink, SinkExt},
    stream::{FusedStream, Stream, StreamExt},
};
use log::{info, warn};
use misskey_core::streaming::ConnectChannelRequest;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
#[cfg(feature = "tokio-runtime")]
use tokio::task;

pub struct ChannelInner {
    id: ChannelId,
    broker_tx: ControlSender,
    response_rx: ResponseStreamReceiver<Value>,
    is_terminated: bool,
}

impl Debug for ChannelInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ChannelInner")
            .field("id", &self.id)
            .field("is_terminated", &self.is_terminated)
            .finish()
    }
}

impl Stream for ChannelInner {
    type Item = Result<Value>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Value>>> {
        if self.is_terminated {
            return Poll::Ready(None);
        }

        self.response_rx.poll_next_unpin(cx)
    }
}

impl Sink<Value> for ChannelInner {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        self.broker_tx.poll_ready_unpin(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: Value) -> Result<()> {
        let item = BrokerControl::Channel {
            id: self.id,
            message: item,
        };
        self.broker_tx.start_send_unpin(item)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        self.broker_tx.poll_flush_unpin(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        self.broker_tx.poll_close_unpin(cx)
    }
}

impl FusedStream for ChannelInner {
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl Drop for ChannelInner {
    fn drop(&mut self) {
        if self.is_terminated {
            return;
        }

        let mut broker_tx = self.broker_tx.clone();
        let id = self.id;
        task::spawn(async move {
            // If the broker or websocket connection is dead, we don't need to unsubscribe anyway
            // because the client can't be used anymore.
            if let Err(e) = broker_tx.send(BrokerControl::Disconnect { id }).await {
                warn!(
                    "Channel::disconnect failed in Drop::drop (ignored): {:?}",
                    e
                );
            }
        });
    }
}

impl ChannelInner {
    async fn connect(
        name: &'static str,
        serialized_req: Value,
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
    ) -> Result<ChannelInner> {
        let id = ChannelId::uuid();

        let (response_tx, response_rx) = response_stream_channel(SharedBrokerState::clone(&state));
        let (pong_tx, pong_rx) = channel_pong_channel(state);

        broker_tx
            .send(BrokerControl::Connect {
                id,
                name,
                params: serialized_req,
                sender: response_tx,
                pong: pong_tx,
            })
            .await?;

        // wait for `connected` pong message from server
        pong_rx.recv().await?;

        Ok(ChannelInner {
            id,
            broker_tx,
            response_rx,
            is_terminated: false,
        })
    }

    async fn disconnect(&mut self) -> Result<()> {
        if self.is_terminated {
            info!("disconnecting from already terminated Channel, skipping");
            return Ok(());
        }

        self.broker_tx
            .send(BrokerControl::Disconnect { id: self.id })
            .await?;

        self.is_terminated = true;

        Ok(())
    }
}

/// Stream for the [`channel`][`crate::WebSocketClient::channel`] method.
#[must_use = "streams do nothing unless polled"]
#[derive(Debug)]
pub struct Channel<I, O> {
    inner: ChannelInner,
    _marker: PhantomData<fn() -> (I, O)>,
}

impl<I, O> Channel<I, O>
where
    I: DeserializeOwned + 'static,
    O: Serialize + 'static,
{
    // We can't use return-position `impl Trait` syntax here because it assumes all type parameters (i.e. `R`)
    // are "in scope" of (hidden) returned type, and they indirectly brings (unmentioned) lifetime of `R`.
    // Thus we can't express our returned (anonymous) `Future` without `BoxFuture` for now
    // because it is actually independent from `req: R` argument.
    pub(crate) fn connect<R>(
        req: R,
        broker_tx: ControlSender,
        state: SharedBrokerState,
    ) -> BoxFuture<'static, Result<Channel<I, O>>>
    where
        R: ConnectChannelRequest<Incoming = I, Outgoing = O>,
    {
        let req = serde_json::to_value(req);
        Box::pin(async move {
            ChannelInner::connect(R::NAME, req?, broker_tx, state)
                .await
                .map(|inner| Channel {
                    inner,
                    _marker: PhantomData,
                })
        })
    }
}

impl<I, O> Channel<I, O> {
    /// Disconnect from the channel.
    ///
    /// After this call, the stream is no longer available (terminated), i.e. [`StreamExt::next`] returns [`None`].
    /// If you call [`disconnect`][`Channel::disconnect`] on a terminated stream, it will simply
    /// be ignored (with log message if logging is enabled).
    pub async fn disconnect(&mut self) -> Result<()> {
        self.inner.disconnect().await
    }
}

impl<I, O> Stream for Channel<I, O>
where
    I: DeserializeOwned,
{
    type Item = Result<I>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<I>>> {
        let result = futures_util::ready!(self.inner.poll_next_unpin(cx))
            .map(|res| res.and_then(|v| serde_json::from_value(v).map_err(Into::into)));
        Poll::Ready(result)
    }
}

impl<I, O> Sink<O> for Channel<I, O>
where
    O: Serialize,
{
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        self.inner.poll_ready_unpin(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: O) -> Result<()> {
        self.inner.start_send_unpin(serde_json::to_value(item)?)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        self.inner.poll_flush_unpin(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        self.inner.poll_close_unpin(cx)
    }
}

impl<I, O> FusedStream for Channel<I, O>
where
    I: DeserializeOwned,
{
    fn is_terminated(&self) -> bool {
        self.inner.is_terminated()
    }
}
