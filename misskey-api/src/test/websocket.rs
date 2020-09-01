use crate::test::env;

use misskey_core::model::ApiResult;
use misskey_core::streaming::{
    BroadcastClient, BroadcastMessage, SubscribeRequest, SubscriptionClient,
};
use misskey_core::{Client, Request};
use misskey_websocket::{WebSocketClient, WebSocketClientBuilder};

pub struct TestClient {
    pub admin: WebSocketClient,
    pub user: WebSocketClient,
}

impl TestClient {
    pub async fn new() -> Self {
        let admin = WebSocketClientBuilder::new(env::TEST_WEBSOCKET_URL.clone())
            .token(env::TEST_ADMIN_TOKEN.clone())
            .connect()
            .await
            .unwrap();
        let user = WebSocketClientBuilder::new(env::TEST_WEBSOCKET_URL.clone())
            .token(env::TEST_USER_TOKEN.clone())
            .connect()
            .await
            .unwrap();

        TestClient { admin, user }
    }
}

#[async_trait::async_trait]
impl Client for TestClient {
    type Error = <WebSocketClient as Client>::Error;
    async fn request<R: Request + Send>(
        &mut self,
        request: R,
    ) -> Result<ApiResult<R::Response>, Self::Error> {
        self.user.request(request).await
    }
}

#[async_trait::async_trait]
impl<M> BroadcastClient<M> for TestClient
where
    M: BroadcastMessage,
{
    type Error = <WebSocketClient as BroadcastClient<M>>::Error;
    type Stream = <WebSocketClient as BroadcastClient<M>>::Stream;

    async fn broadcast<'a>(&mut self) -> Result<Self::Stream, Self::Error>
    where
        M: 'a,
    {
        self.user.broadcast().await
    }
}

#[async_trait::async_trait]
impl<R> SubscriptionClient<R> for TestClient
where
    R: SubscribeRequest + Send,
    R::Unsubscribe: Send + Unpin,
{
    type Error = <WebSocketClient as SubscriptionClient<R>>::Error;
    type Stream = <WebSocketClient as SubscriptionClient<R>>::Stream;

    async fn subscribe<'a>(&mut self, request: R) -> Result<Self::Stream, Self::Error>
    where
        R: 'a,
    {
        self.user.subscribe(request).await
    }
}
