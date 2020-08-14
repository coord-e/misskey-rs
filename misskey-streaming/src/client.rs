use crate::broker::{
    channel::{control_channel, response_channel, response_stream_channel, ControlSender},
    model::BrokerControl,
    Broker,
};
use crate::channel::{connect_websocket, WebSocketSender};
use crate::error::{Error, Result};
use crate::model::{
    request::{ConnectChannel, MainType, Request, TimelineType},
    ChannelId,
};

use misskey::api::ApiRequest;
use misskey::client::Client;
use misskey::model::note::NoteId;
use serde_json::value;
use url::Url;

pub mod builder;
pub mod stream;

use stream::{MainStream, NoteUpdate, Timeline};

pub struct WebSocketClient {
    websocket_tx: WebSocketSender,
    broker_tx: ControlSender,
}

impl WebSocketClient {
    pub async fn connect(url: Url) -> Result<WebSocketClient> {
        let (websocket_tx, websocket_rx) = connect_websocket(url).await?;
        let (broker_tx, broker_rx) = control_channel();

        Broker::new(websocket_rx, broker_rx).spawn();

        Ok(WebSocketClient {
            websocket_tx,
            broker_tx,
        })
    }

    pub async fn timeline(&mut self, timeline: TimelineType) -> Result<Timeline> {
        let id = ChannelId::new();

        let (tx, rx) = response_stream_channel();
        self.broker_tx.send(BrokerControl::SubscribeTimeline {
            id: id.clone(),
            sender: tx,
        });

        let req = Request::Connect {
            id: id.clone(),
            channel: ConnectChannel::Timeline(timeline),
        };
        self.websocket_tx.send_json(&req).await?;

        Ok(Timeline {
            id,
            broker_tx: self.broker_tx.clone(),
            response_rx: rx,
        })
    }

    pub async fn main_stream(&mut self) -> Result<MainStream> {
        let id = ChannelId::new();

        let (tx, rx) = response_stream_channel();
        self.broker_tx.send(BrokerControl::SubscribeMainStream {
            id: id.clone(),
            sender: tx,
        });

        let req = Request::Connect {
            id: id.clone(),
            channel: ConnectChannel::Main(MainType::Main),
        };
        self.websocket_tx.send_json(&req).await?;

        Ok(MainStream {
            id,
            broker_tx: self.broker_tx.clone(),
            response_rx: rx,
        })
    }

    pub async fn capture_note(&mut self, note_id: NoteId) -> Result<NoteUpdate> {
        let (tx, rx) = response_stream_channel();
        self.broker_tx.send(BrokerControl::SubscribeNote {
            id: note_id.clone(),
            sender: tx,
        });

        let req = Request::SubNote {
            id: note_id.clone(),
        };
        self.websocket_tx.send_json(&req).await?;

        Ok(NoteUpdate {
            id: note_id,
            broker_tx: self.broker_tx.clone(),
            response_rx: rx,
        })
    }
}

#[async_trait::async_trait]
impl Client for WebSocketClient {
    type Error = Error;

    async fn request<R: ApiRequest + Send>(&mut self, request: R) -> Result<R::Response> {
        let id = ChannelId::new();

        let (tx, rx) = response_channel();
        self.broker_tx.send(BrokerControl::HandleApiResponse {
            id: id.clone(),
            sender: tx,
        });

        let req = Request::Api {
            id,
            endpoint: R::ENDPOINT.to_string(),
            data: value::to_value(request)?,
        };
        self.websocket_tx.send_json(&req).await?;

        let x = rx.recv().await;
        Ok(value::from_value(x)?)
    }
}
