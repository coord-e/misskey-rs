use std::fmt::{self, Debug};
use std::future::Future;
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

use futures::{
    executor,
    sink::{Sink, SinkExt},
    stream::{FusedStream, Stream, StreamExt},
};
use log::{info, warn};
use misskey_core::streaming::ConnectChannelRequest;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

/// Stream for the [`channel`][`crate::WebSocketClient::channel`] method.
#[must_use = "streams do nothing unless polled"]
pub struct Channel<I, O> {
    id: ChannelId,
    broker_tx: ControlSender,
    response_rx: ResponseStreamReceiver<Value>,
    is_terminated: bool,
    _marker: PhantomData<fn() -> (I, O)>,
}

impl<I, O> Debug for Channel<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Channel")
            .field("id", &self.id)
            .field("is_terminated", &self.is_terminated)
            .finish()
    }
}

impl<I, O> Channel<I, O>
where
    I: DeserializeOwned + 'static,
    O: Serialize,
{
    pub(crate) fn connect<R>(
        req: R,
        mut broker_tx: ControlSender,
        state: SharedBrokerState,
    ) -> impl Future<Output = Result<Channel<I, O>>>
    where
        R: ConnectChannelRequest<Incoming = I, Outgoing = O>,
    {
        let id = ChannelId::uuid();

        let (response_tx, response_rx) = response_stream_channel(SharedBrokerState::clone(&state));
        let (pong_tx, pong_rx) = channel_pong_channel(state);

        // limit the use of `R` to the outside of `async`
        // in order not to require `Send` on `R`
        let serialized_req = serde_json::to_value(req);

        async move {
            broker_tx
                .send(BrokerControl::Connect {
                    id,
                    name: R::NAME,
                    params: serialized_req?,
                    sender: response_tx,
                    pong: pong_tx,
                })
                .await?;

            // wait for `connected` pong message from server
            pong_rx.recv().await?;

            Ok(Channel {
                id,
                broker_tx,
                response_rx,
                is_terminated: false,
                _marker: PhantomData,
            })
        }
    }
}

impl<I, O> Channel<I, O> {
    /// Disconnect from the channel.
    ///
    /// After this call, the stream is no longer available (terminated), i.e. [`StreamExt::next`] returns [`None`].
    /// If you call [`disconnect`][`Channel::disconnect`] on a terminated stream, it will simply
    /// be ignored (with log message if logging is enabled).
    pub async fn disconnect(&mut self) -> Result<()> {
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

impl<I, O> Stream for Channel<I, O>
where
    I: DeserializeOwned,
{
    type Item = Result<I>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<I>>> {
        if self.is_terminated {
            return Poll::Ready(None);
        }

        match futures::ready!(self.response_rx.poll_next_unpin(cx)?) {
            None => Poll::Ready(None),
            Some(v) => Poll::Ready(Some(Ok(serde_json::from_value(v)?))),
        }
    }
}

impl<I, O> Sink<O> for Channel<I, O>
where
    O: Serialize,
{
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        self.broker_tx.poll_ready_unpin(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: O) -> Result<()> {
        let item = BrokerControl::Channel {
            id: self.id,
            message: serde_json::to_value(item)?,
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

impl<I, O> FusedStream for Channel<I, O>
where
    I: DeserializeOwned,
{
    fn is_terminated(&self) -> bool {
        self.is_terminated
    }
}

impl<I, O> Drop for Channel<I, O> {
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
