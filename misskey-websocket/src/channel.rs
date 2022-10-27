use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::error::{Error, Result};
use crate::model::{incoming::IncomingMessage, outgoing::OutgoingMessage};

#[cfg(feature = "async-std-runtime")]
use async_tungstenite::async_std::{connect_async, ConnectStream};
#[cfg(feature = "tokio-runtime")]
use async_tungstenite::tokio::{connect_async, ConnectStream};
use async_tungstenite::tungstenite::{
    error::{Error as WsError, Result as WsResult},
    Message as WsMessage,
};
use async_tungstenite::WebSocketStream;
use futures_util::{
    sink::{Sink, SinkExt},
    stream::{SplitSink, SplitStream, Stream, StreamExt, TryStreamExt},
};
#[cfg(feature = "inspect-contents")]
use log::debug;
use url::Url;

/// Receiver channel that communicates with Misskey
pub struct WebSocketReceiver(SplitStream<PingPongWebSocketStream<WebSocketStream<ConnectStream>>>);

impl fmt::Debug for WebSocketReceiver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WebSocketReceiver").finish()
    }
}

impl Stream for WebSocketReceiver {
    type Item = Result<IncomingMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let text = match futures_util::ready!(self.0.poll_next_unpin(cx)?) {
            Some(WsMessage::Text(t)) => t,
            Some(WsMessage::Ping(_)) | Some(WsMessage::Pong(_)) => return self.poll_next(cx),
            None | Some(WsMessage::Close(_)) => return Poll::Ready(None),
            Some(m) => return Poll::Ready(Some(Err(Error::UnexpectedMessage(m)))),
        };

        #[cfg(feature = "inspect-contents")]
        debug!("received message: {}", text);

        Poll::Ready(Some(Ok(serde_json::from_str(&text)?)))
    }
}

pub struct Recv<'a> {
    stream: &'a mut WebSocketReceiver,
}

impl Future for Recv<'_> {
    type Output = Result<IncomingMessage>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.stream
            .poll_next_unpin(cx)
            .map(|opt| opt.unwrap_or_else(|| Err(WsError::ConnectionClosed.into())))
    }
}

impl WebSocketReceiver {
    /// Receive one message from the stream using `StreamExt::next`,
    /// while folding `None` to an error that represents closed connection.
    pub fn recv(&mut self) -> Recv<'_> {
        Recv { stream: self }
    }
}

/// Sender channel that communicates with Misskey
pub struct WebSocketSender(
    SplitSink<PingPongWebSocketStream<WebSocketStream<ConnectStream>>, WsMessage>,
);

#[derive(Debug, Clone)]
pub struct TrySendError {
    pub message: OutgoingMessage,
    pub error: Error,
}

impl WebSocketSender {
    /// convenient method that retains the message in the error
    pub async fn try_send(
        &mut self,
        item: OutgoingMessage,
    ) -> std::result::Result<(), TrySendError> {
        self.send(&item).await.map_err(|error| TrySendError {
            message: item,
            error,
        })
    }
}

impl fmt::Debug for WebSocketSender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WebSocketSender").finish()
    }
}

impl Sink<&'_ OutgoingMessage> for WebSocketSender {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        self.0.poll_ready_unpin(cx).map_err(Into::into)
    }

    fn start_send(mut self: Pin<&mut Self>, item: &OutgoingMessage) -> Result<()> {
        let msg = WsMessage::Text(serde_json::to_string(item)?);

        #[cfg(feature = "inspect-contents")]
        debug!("send message: {:?}", msg);

        self.0.start_send_unpin(msg).map_err(Into::into)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        self.0.poll_flush_unpin(cx).map_err(Into::into)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        self.0.poll_close_unpin(cx).map_err(Into::into)
    }
}

pub enum SendPongState {
    WaitSink(Vec<u8>),
    WaitFlush,
}

pub struct PingPongWebSocketStream<S> {
    stream: S,
    state: Option<SendPongState>,
}

impl<S> PingPongWebSocketStream<S> {
    pub fn new(stream: S) -> Self {
        PingPongWebSocketStream {
            stream,
            state: None,
        }
    }
}

impl<S: Unpin> Stream for PingPongWebSocketStream<S>
where
    S: Sink<WsMessage, Error = WsError> + Stream<Item = WsResult<WsMessage>>,
{
    type Item = WsResult<WsMessage>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match self.state.take() {
            None => {
                let data = match futures_util::ready!(self.stream.try_poll_next_unpin(cx)) {
                    Some(Ok(WsMessage::Ping(data))) => data,
                    opt => return Poll::Ready(opt),
                };

                self.state.replace(SendPongState::WaitSink(data));
                self.poll_next(cx)
            }
            Some(SendPongState::WaitSink(data)) => {
                match self.stream.poll_ready_unpin(cx) {
                    Poll::Pending => {
                        self.state.replace(SendPongState::WaitSink(data));
                        return Poll::Pending;
                    }
                    Poll::Ready(Err(e)) => return Poll::Ready(Some(Err(e))),
                    Poll::Ready(Ok(())) => {}
                }

                self.stream.start_send_unpin(WsMessage::Pong(data))?;
                self.state.replace(SendPongState::WaitFlush);
                self.poll_next(cx)
            }
            Some(SendPongState::WaitFlush) => match self.stream.poll_flush_unpin(cx) {
                Poll::Pending => {
                    self.state.replace(SendPongState::WaitFlush);
                    Poll::Pending
                }
                Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
                Poll::Ready(Ok(())) => self.poll_next(cx),
            },
        }
    }
}

impl<S: Unpin> Sink<WsMessage> for PingPongWebSocketStream<S>
where
    S: Sink<WsMessage, Error = WsError>,
{
    type Error = WsError;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<WsResult<()>> {
        self.stream.poll_ready_unpin(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: WsMessage) -> WsResult<()> {
        self.stream.start_send_unpin(item)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<WsResult<()>> {
        self.stream.poll_flush_unpin(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<WsResult<()>> {
        self.stream.poll_close_unpin(cx)
    }
}

pub async fn connect_websocket(url: Url) -> Result<(WebSocketSender, WebSocketReceiver)> {
    let (ws, _) = connect_async(url).await?;
    let (sink, stream) = PingPongWebSocketStream::new(ws).split();
    Ok((WebSocketSender(sink), WebSocketReceiver(stream)))
}
