use std::sync::Arc;

use crate::broker::{
    channel::{response_channel, ControlSender},
    model::{BrokerControl, SharedBrokerState},
    Broker,
};
use crate::channel::{connect_websocket, SharedWebSocketSender};
use crate::error::{Error, Result};
use crate::model::{ApiRequestId, OutgoingMessage};

use futures::{future::BoxFuture, lock::Mutex, sink::SinkExt};
use misskey_core::model::ApiResult;
use misskey_core::{
    streaming::{BroadcastClient, ChannelClient, SubNoteClient},
    Client,
};
use serde_json::value;
use url::Url;

pub mod builder;
pub mod stream;

use stream::{Broadcast, Channel, SubNote};

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
}

impl Client for WebSocketClient {
    type Error = Error;

    fn request<'a, R>(&'a mut self, request: R) -> BoxFuture<'a, Result<ApiResult<R::Response>>>
    where
        R: misskey_core::Request + 'a,
    {
        let id = ApiRequestId::uuid();

        // limit the use of `R` to the outside of `async`
        // in order not to require `Send` on `R`
        let serialized_request = serde_json::to_value(request);

        Box::pin(async move {
            let (tx, rx) = response_channel(Arc::clone(&self.state));
            self.broker_tx
                .send(BrokerControl::HandleApiResponse {
                    id: id.clone(),
                    sender: tx,
                })
                .await?;

            self.websocket_tx
                .lock()
                .await
                .send(OutgoingMessage::Api {
                    id,
                    endpoint: R::ENDPOINT,
                    data: serialized_request?,
                })
                .await?;

            Ok(match rx.recv().await? {
                ApiResult::Ok(x) => ApiResult::Ok(value::from_value(x)?),
                ApiResult::Err { error } => ApiResult::Err { error },
            })
        })
    }
}

impl<E> SubNoteClient<E> for WebSocketClient
where
    E: misskey_core::streaming::SubNoteEvent,
{
    type Error = Error;
    type Stream = SubNote<E>;

    fn subscribe_note<I>(&mut self, id: I) -> BoxFuture<'static, Result<SubNote<E>>>
    where
        I: Into<misskey_core::streaming::SubNoteId>,
    {
        Box::pin(SubNote::subscribe(
            id.into(),
            self.broker_tx.clone(),
            Arc::clone(&self.state),
            Arc::clone(&self.websocket_tx),
        ))
    }
}

impl<R> ChannelClient<R> for WebSocketClient
where
    R: misskey_core::streaming::ConnectChannelRequest,
{
    type Error = Error;
    type Stream = Channel<R::Incoming, R::Outgoing>;

    fn connect<'a>(&mut self, request: R) -> BoxFuture<'a, Result<Self::Stream>>
    where
        R: 'a,
    {
        Box::pin(Channel::connect(
            request,
            self.broker_tx.clone(),
            Arc::clone(&self.state),
            Arc::clone(&self.websocket_tx),
        ))
    }
}

impl<E> BroadcastClient<E> for WebSocketClient
where
    E: misskey_core::streaming::BroadcastEvent,
{
    type Error = Error;
    type Stream = Broadcast<E>;

    fn broadcast(&mut self) -> BoxFuture<'static, Result<Self::Stream>> {
        Box::pin(Broadcast::start(
            self.broker_tx.clone(),
            Arc::clone(&self.state),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use super::{builder::WebSocketClientBuilder, WebSocketClient};

    use futures::stream::StreamExt;
    use misskey_core::{streaming::SubNoteClient, Client};
    use url::Url;

    static INIT_LOGGER: Once = Once::new();

    async fn test_client() -> WebSocketClient {
        INIT_LOGGER.call_once(env_logger::init);

        let url = std::env::var("TEST_WEBSOCKET_URL").unwrap();
        let token = std::env::var("TEST_USER_TOKEN").unwrap();
        WebSocketClientBuilder::new(Url::parse(&url).unwrap())
            .token(token)
            .connect()
            .await
            .unwrap()
    }

    #[cfg_attr(feature = "tokio-runtime", tokio::test)]
    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    async fn request() {
        let mut client = test_client().await;

        client
            .request(
                misskey_api::endpoint::notes::create::Request::builder()
                    .text("hi")
                    .build(),
            )
            .await
            .unwrap()
            .unwrap();
    }

    #[cfg_attr(feature = "tokio-runtime", tokio::test)]
    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    async fn subscribe_note() {
        let mut client = test_client().await;
        let note = client
            .request(
                misskey_api::endpoint::notes::create::Request::builder()
                    .text("hi")
                    .build(),
            )
            .await
            .unwrap()
            .unwrap()
            .created_note;

        let mut stream: crate::stream::SubNote<misskey_api::streaming::note::NoteUpdateEvent> =
            client.subscribe_note(note.id.clone()).await.unwrap();

        futures::future::join(
            async {
                client
                    .request(misskey_api::endpoint::notes::delete::Request { note_id: note.id })
                    .await
                    .unwrap()
                    .unwrap()
            },
            async { stream.next().await.unwrap().unwrap() },
        )
        .await;
    }

    // TODO: test of `Broadcast`
}
