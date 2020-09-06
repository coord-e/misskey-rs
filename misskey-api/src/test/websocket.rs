use crate::test::env;

use futures::future::BoxFuture;
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

impl Client for TestClient {
    type Error = <WebSocketClient as Client>::Error;
    fn request<'a, R>(
        &'a self,
        request: R,
    ) -> BoxFuture<'a, Result<ApiResult<R::Response>, Self::Error>>
    where
        R: Request + 'a,
    {
        self.user.request(request)
    }
}

impl<E> BroadcastClient<E> for TestClient
where
    E: BroadcastEvent,
{
    type Error = <WebSocketClient as BroadcastClient<E>>::Error;
    type Stream = <WebSocketClient as BroadcastClient<E>>::Stream;

    fn broadcast(&self) -> BoxFuture<'static, Result<Self::Stream, Self::Error>> {
        self.user.broadcast()
    }
}

impl<R> ChannelClient<R> for TestClient
where
    R: ConnectChannelRequest,
{
    type Error = <WebSocketClient as ChannelClient<R>>::Error;
    type Stream = <WebSocketClient as ChannelClient<R>>::Stream;

    fn connect<'a>(&self, request: R) -> BoxFuture<'a, Result<Self::Stream, Self::Error>>
    where
        R: 'a,
    {
        self.user.connect(request)
    }
}

impl<E> SubNoteClient<E> for TestClient
where
    E: SubNoteEvent,
{
    type Error = <WebSocketClient as SubNoteClient<E>>::Error;
    type Stream = <WebSocketClient as SubNoteClient<E>>::Stream;

    fn subscribe_note<I>(&self, id: I) -> BoxFuture<'static, Result<Self::Stream, Self::Error>>
    where
        I: Into<SubNoteId>,
    {
        self.user.subscribe_note(id)
    }
}
