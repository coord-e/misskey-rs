use std::sync::Arc;

use crate::broker::{
    channel::{response_channel, ControlSender},
    model::{BrokerControl, SharedBrokerState},
    Broker,
};
use crate::channel::{connect_websocket, WebSocketSender};
use crate::error::{Error, Result};
use crate::model::{
    request::{Request, TimelineType},
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
    state: SharedBrokerState,
}

impl WebSocketClient {
    pub async fn connect(url: Url) -> Result<WebSocketClient> {
        let (websocket_tx, websocket_rx) = connect_websocket(url).await?;
        let (broker_tx, state) = Broker::spawn(websocket_rx);

        Ok(WebSocketClient {
            websocket_tx,
            broker_tx,
            state,
        })
    }

    pub async fn timeline<'a>(&'a mut self, timeline: TimelineType) -> Result<Timeline<'a>> {
        Timeline::subscribe(
            timeline,
            self.broker_tx.clone(),
            Arc::clone(&self.state),
            &mut self.websocket_tx,
        )
        .await
    }

    pub async fn main_stream<'a>(&'a mut self) -> Result<MainStream<'a>> {
        MainStream::subscribe(
            self.broker_tx.clone(),
            Arc::clone(&self.state),
            &mut self.websocket_tx,
        )
        .await
    }

    pub async fn capture_note<'a>(&'a mut self, note_id: NoteId) -> Result<NoteUpdate<'a>> {
        NoteUpdate::subscribe(
            note_id,
            self.broker_tx.clone(),
            Arc::clone(&self.state),
            &mut self.websocket_tx,
        )
        .await
    }
}

#[async_trait::async_trait]
impl Client for WebSocketClient {
    type Error = Error;

    async fn request<R: ApiRequest + Send>(&mut self, request: R) -> Result<R::Response> {
        let id = ChannelId::new();

        let (tx, rx) = response_channel(Arc::clone(&self.state));
        self.broker_tx
            .send(BrokerControl::HandleApiResponse {
                id: id.clone(),
                sender: tx,
            })
            .await?;

        let req = Request::Api {
            id,
            endpoint: R::ENDPOINT.to_string(),
            data: value::to_value(request)?,
        };
        self.websocket_tx.send_json(&req).await?;

        let x = rx.recv().await?;
        Ok(value::from_value(x)?)
    }
}
