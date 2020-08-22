use std::sync::Arc;

use crate::broker::{
    channel::{response_channel, ControlSender},
    model::{BrokerControl, SharedBrokerState},
    Broker,
};
use crate::channel::{connect_websocket, SharedWebSocketSender};
use crate::error::{Error, Result};
use crate::model::{
    request::{Request, Timeline},
    ChannelId,
};

use async_std::sync::Mutex;
use misskey_api::model::note::NoteId;
use misskey_core::model::ApiResult;
use misskey_core::Client;
use serde_json::value;
use url::Url;

pub mod builder;
pub mod stream;

use stream::{MainStream, NoteUpdateStream, TimelineStream};

#[derive(Debug, Clone)]
pub struct WebSocketClient {
    websocket_tx: SharedWebSocketSender,
    broker_tx: ControlSender,
    state: SharedBrokerState,
}

impl WebSocketClient {
    pub async fn connect(url: Url) -> Result<WebSocketClient> {
        let (websocket_tx, websocket_rx) = connect_websocket(url).await?;
        let websocket_tx = Arc::new(Mutex::new(websocket_tx));

        let (broker_tx, state) = Broker::spawn(websocket_rx);

        Ok(WebSocketClient {
            websocket_tx,
            broker_tx,
            state,
        })
    }

    pub async fn timeline(&mut self, timeline: Timeline) -> Result<TimelineStream> {
        TimelineStream::subscribe(
            timeline,
            self.broker_tx.clone(),
            Arc::clone(&self.state),
            Arc::clone(&self.websocket_tx),
        )
        .await
    }

    pub async fn main_stream(&mut self) -> Result<MainStream> {
        MainStream::subscribe(
            self.broker_tx.clone(),
            Arc::clone(&self.state),
            Arc::clone(&self.websocket_tx),
        )
        .await
    }

    pub async fn capture_note(&mut self, note_id: NoteId) -> Result<NoteUpdateStream> {
        NoteUpdateStream::subscribe(
            note_id,
            self.broker_tx.clone(),
            Arc::clone(&self.state),
            Arc::clone(&self.websocket_tx),
        )
        .await
    }
}

#[async_trait::async_trait]
impl Client for WebSocketClient {
    type Error = Error;

    async fn request<R: misskey_core::Request + Send>(
        &mut self,
        request: R,
    ) -> Result<ApiResult<R::Response>> {
        let id = ChannelId::new();

        let (tx, rx) = response_channel(Arc::clone(&self.state));
        self.broker_tx
            .send(BrokerControl::HandleApiResponse { id, sender: tx })
            .await?;

        let req = Request::Api {
            id,
            endpoint: R::ENDPOINT.to_string(),
            data: value::to_value(request)?,
        };
        self.websocket_tx.lock().await.send_json(&req).await?;

        Ok(match rx.recv().await? {
            ApiResult::Ok(x) => ApiResult::Ok(value::from_value(x)?),
            ApiResult::Err { error } => ApiResult::Err { error },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::WebSocketClient;
    use misskey_core::Client;

    async fn test_client() -> WebSocketClient {
        use crate::WebSocketClientBuilder;

        let url = std::env::var("TEST_WEBSOCKET_URL").unwrap();
        let url = url::Url::parse(&url).unwrap();
        let user_token = std::env::var("TEST_USER_TOKEN").unwrap();

        WebSocketClientBuilder::new(url)
            .token(user_token)
            .connect()
            .await
            .unwrap()
    }

    #[async_std::test]
    async fn timeline() {
        use crate::model::Timeline;

        let mut client = test_client().await;

        {
            let mut stream = client.timeline(Timeline::Home).await.unwrap();
            stream.unsubscribe().await.unwrap();
        }
        {
            let mut stream = client.timeline(Timeline::Local).await.unwrap();
            stream.unsubscribe().await.unwrap();
        }
        {
            let mut stream = client.timeline(Timeline::Global).await.unwrap();
            stream.unsubscribe().await.unwrap();
        }
        {
            let mut stream = client.timeline(Timeline::Social).await.unwrap();
            stream.unsubscribe().await.unwrap();
        }
    }

    #[async_std::test]
    async fn main_stream() {
        let mut client = test_client().await;
        let mut stream = client.main_stream().await.unwrap();
        stream.unsubscribe().await.unwrap();
    }

    #[async_std::test]
    async fn capture_note() {
        let mut client = test_client().await;
        let note = client
            .request(misskey_api::api::notes::create::Request {
                visibility: None,
                visible_user_ids: Vec::new(),
                text: Some("test".to_string()),
                cw: None,
                via_mobile: false,
                local_only: false,
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                file_ids: Vec::new(),
                reply_id: None,
                renote_id: None,
                poll: None,
            })
            .await
            .unwrap()
            .unwrap()
            .created_note;

        let mut stream = client.capture_note(note.id).await.unwrap();
        stream.unsubscribe().await.unwrap();
    }

    #[async_std::test]
    async fn request() {
        let mut client = test_client().await;
        client
            .request(misskey_api::api::i::Request {})
            .await
            .unwrap()
            .unwrap();
    }
}
