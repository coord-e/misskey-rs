use crate::test::env;

use misskey_core::model::ApiResult;
use misskey_core::streaming::{
    BroadcastClient, BroadcastEvent, ChannelClient, ConnectChannelRequest, SubNoteClient,
    SubNoteEvent, SubNoteId,
};
use misskey_core::{Client, Request};
use misskey_websocket::{WebSocketClient, WebSocketClientBuilder};

pub struct TestClient {
    pub admin: WebSocketClient,
    pub user: WebSocketClient,
}

impl TestClient {
    pub async fn new() -> Self {
        env::init_logger();

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
impl<E> BroadcastClient<E> for TestClient
where
    E: BroadcastEvent,
{
    type Error = <WebSocketClient as BroadcastClient<E>>::Error;
    type Stream = <WebSocketClient as BroadcastClient<E>>::Stream;

    async fn broadcast<'a>(&mut self) -> Result<Self::Stream, Self::Error>
    where
        E: 'a,
    {
        self.user.broadcast().await
    }
}

#[async_trait::async_trait]
impl<R> ChannelClient<R> for TestClient
where
    R: ConnectChannelRequest + Send,
    R::Outgoing: Unpin,
{
    type Error = <WebSocketClient as ChannelClient<R>>::Error;
    type Stream = <WebSocketClient as ChannelClient<R>>::Stream;

    async fn connect<'a>(&mut self, request: R) -> Result<Self::Stream, Self::Error>
    where
        R: 'a,
    {
        self.user.connect(request).await
    }
}

#[async_trait::async_trait]
impl<E> SubNoteClient<E> for TestClient
where
    E: SubNoteEvent,
{
    type Error = <WebSocketClient as SubNoteClient<E>>::Error;
    type Stream = <WebSocketClient as SubNoteClient<E>>::Stream;

    async fn subscribe_note<'a, I>(&mut self, id: I) -> Result<Self::Stream, Self::Error>
    where
        I: Into<SubNoteId> + Send,
        E: 'a,
    {
        self.user.subscribe_note(id).await
    }
}
