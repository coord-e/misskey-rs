use std::{fmt, sync::Arc};

use crate::error::{Error, Result};

use async_std::sync::Mutex;
use async_tungstenite::async_std::{connect_async, ConnectStream};
use async_tungstenite::tungstenite::{error::Error as WsError, Message as WsMessage};
use async_tungstenite::WebSocketStream;
use futures::sink::SinkExt;
use futures::stream::{SplitSink, SplitStream, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

/// Receiver channel that communicates with Misskey
pub struct WebSocketReceriver(SplitStream<WebSocketStream<ConnectStream>>);

impl fmt::Debug for WebSocketReceriver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WebSocketReceriver").finish()
    }
}

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
pub struct WebSocketSender(SplitSink<WebSocketStream<ConnectStream>, WsMessage>);

impl fmt::Debug for WebSocketSender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WebSocketSender").finish()
    }
}

impl WebSocketSender {
    pub async fn send(&mut self, msg: WsMessage) -> Result<()> {
        Ok(self.0.send(msg).await?)
    }

    pub async fn send_json<T: Serialize>(&mut self, x: &T) -> Result<()> {
        self.send(WsMessage::Text(serde_json::to_string(x)?)).await
    }
}

pub type SharedWebSocketSender = Arc<Mutex<WebSocketSender>>;

pub async fn connect_websocket(url: Url) -> Result<(WebSocketSender, WebSocketReceriver)> {
    let (ws, _) = connect_async(url).await?;
    let (sink, stream) = ws.split();
    Ok((WebSocketSender(sink), WebSocketReceriver(stream)))
}
