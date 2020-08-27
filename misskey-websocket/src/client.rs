use std::sync::Arc;

use crate::broker::{
    channel::{response_channel, ControlSender},
    model::{BrokerControl, SharedBrokerState},
    Broker,
};
use crate::channel::{connect_websocket, SharedWebSocketSender};
use crate::error::{Error, Result};
use crate::model::{
    request::{ApiRequest, Request},
    RequestId,
};

use futures::lock::Mutex;
use misskey_core::model::ApiResult;
use misskey_core::{
    streaming::{BroadcastClient, SubscriptionClient},
    Client,
};
use serde_json::value;
use url::Url;

pub mod builder;
pub mod stream;

use stream::{Broadcast, Subscription};

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

#[async_trait::async_trait]
impl Client for WebSocketClient {
    type Error = Error;

    async fn request<R: misskey_core::Request + Send>(
        &mut self,
        request: R,
    ) -> Result<ApiResult<R::Response>> {
        let id = RequestId::uuid();

        let (tx, rx) = response_channel(Arc::clone(&self.state));
        self.broker_tx
            .send(BrokerControl::HandleApiResponse {
                id: id.clone(),
                sender: tx,
            })
            .await?;

        let req = ApiRequest {
            id,
            endpoint: R::ENDPOINT.to_string(),
            data: value::to_value(request)?,
        };
        self.websocket_tx
            .lock()
            .await
            .send_json(&Request {
                type_: "api",
                body: serde_json::to_value(req)?,
            })
            .await?;

        Ok(match rx.recv().await? {
            ApiResult::Ok(x) => ApiResult::Ok(value::from_value(x)?),
            ApiResult::Err { error } => ApiResult::Err { error },
        })
    }
}

#[async_trait::async_trait]
impl<I> BroadcastClient<I> for WebSocketClient
where
    I: misskey_core::streaming::BroadcastItem,
{
    type Error = Error;
    type Stream = Broadcast<I>;

    async fn broadcast<'a>(&mut self) -> Result<Self::Stream>
    where
        I: 'a,
    {
        Broadcast::start(self.broker_tx.clone(), Arc::clone(&self.state)).await
    }
}

#[async_trait::async_trait]
impl<R> SubscriptionClient<R> for WebSocketClient
where
    R: misskey_core::streaming::SubscriptionRequest + Send,
{
    type Error = Error;
    type Stream = Subscription<R>;

    async fn subscribe<'a>(&mut self, request: R) -> Result<Self::Stream>
    where
        R: 'a,
    {
        Subscription::subscribe(
            request,
            self.broker_tx.clone(),
            Arc::clone(&self.state),
            Arc::clone(&self.websocket_tx),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::{builder::WebSocketClientBuilder, WebSocketClient};

    use futures::stream::StreamExt;
    use misskey_core::{streaming::SubscriptionClient, Client};
    use url::Url;

    async fn test_client() -> WebSocketClient {
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
    async fn subscribe_timeline() {
        let mut client = test_client().await;

        let mut stream = client
            .subscribe(misskey_api::streaming::timeline::Request {
                channel: misskey_api::model::timeline::Timeline::Local,
            })
            .await
            .unwrap();

        futures::future::join(
            async {
                client
                    .request(
                        misskey_api::endpoint::notes::create::Request::builder()
                            .text("stream me")
                            .build(),
                    )
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
