use crate::error::{Error, Result};

use futures::sink::SinkExt;
use futures::stream::{SplitSink, SplitStream, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use tokio::net::TcpStream;
use tokio_native_tls::TlsStream;
use tokio_tungstenite::stream::Stream as AutoStream;
use tokio_tungstenite::tungstenite::{error::Error as WsError, Message as WsMessage};
use tokio_tungstenite::WebSocketStream;
use url::Url;

/// Receiver channel that communicates with Misskey
pub struct WebSocketReceriver(
    SplitStream<WebSocketStream<AutoStream<TcpStream, TlsStream<TcpStream>>>>,
);

impl WebSocketReceriver {
    pub async fn recv(&mut self) -> Result<WsMessage> {
        match self.0.next().await {
            Some(x) => Ok(x?),
            None => Err(Error::WebSocket(WsError::ConnectionClosed.into())),
        }
    }

    pub async fn recv_json<T: DeserializeOwned>(&mut self) -> Result<T> {
        loop {
            match self.recv().await? {
                WsMessage::Text(t) => return Ok(serde_json::from_str(&t)?),
                // tungstenite automatically handles ping/pong
                WsMessage::Ping(_) => continue,
                WsMessage::Pong(_) => continue,
                // defer `ConnectionClosed` error to next `recv`
                WsMessage::Close(_) => continue,
                m => return Err(Error::UnexpectedMessage(m)),
            }
        }
    }
}

/// Sender channel that communicates with Misskey
pub struct WebSocketSender(
    SplitSink<WebSocketStream<AutoStream<TcpStream, TlsStream<TcpStream>>>, WsMessage>,
);

impl WebSocketSender {
    pub async fn send(&mut self, msg: WsMessage) -> Result<()> {
        Ok(self.0.send(msg).await?)
    }

    pub async fn send_json<T: Serialize>(&mut self, x: &T) -> Result<()> {
        self.send(WsMessage::Text(serde_json::to_string(x)?)).await
    }
}

pub async fn connect_websocket(url: Url) -> Result<(WebSocketSender, WebSocketReceriver)> {
    let (ws, _) = tokio_tungstenite::connect_async(url).await?;
    let (sink, stream) = ws.split();
    Ok((WebSocketSender(sink), WebSocketReceriver(stream)))
}
