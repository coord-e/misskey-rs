use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::error::{Error, Result};
use crate::model::{incoming::IncomingMessage, outgoing::OutgoingMessage};

#[cfg(all(feature = "async-std-runtime", not(feature = "tokio-runtime")))]
use async_tungstenite::async_std::{connect_async, ConnectStream};
#[cfg(all(not(feature = "async-std-runtime"), feature = "tokio-runtime"))]
use async_tungstenite::tokio::{connect_async, ConnectStream};
use async_tungstenite::tungstenite::{error::Error as WsError, Message as WsMessage};
use async_tungstenite::WebSocketStream;
use futures::{
    sink::{Sink, SinkExt},
    stream::{SplitSink, SplitStream, Stream, StreamExt},
};
#[cfg(feature = "inspect-contents")]
use log::debug;
use url::Url;

/// Receiver channel that communicates with Misskey
pub struct WebSocketReceiver(SplitStream<WebSocketStream<ConnectStream>>);

impl fmt::Debug for WebSocketReceiver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WebSocketReceiver").finish()
    }
}

impl Stream for WebSocketReceiver {
    type Item = Result<IncomingMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let opt = match self.0.poll_next_unpin(cx)? {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(opt) => opt,
        };

        let text = match opt {
            Some(WsMessage::Text(t)) => t,
            Some(WsMessage::Ping(_)) | Some(WsMessage::Pong(_)) => return Poll::Pending,
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
pub struct WebSocketSender(SplitSink<WebSocketStream<ConnectStream>, WsMessage>);

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

pub async fn connect_websocket(url: Url) -> Result<(WebSocketSender, WebSocketReceiver)> {
    let (ws, _) = connect_async(url).await?;
    let (sink, stream) = ws.split();
    Ok((WebSocketSender(sink), WebSocketReceiver(stream)))
}
