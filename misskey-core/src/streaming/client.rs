use crate::streaming::api::{BroadcastMessage, OneshotRequest, SubscribeRequest};

#[async_trait::async_trait]
pub trait OneshotClient {
    type Error: std::error::Error;

    async fn send<R>(&mut self, request: R) -> Result<(), Self::Error>
    where
        R: OneshotRequest + Send;
}

#[async_trait::async_trait]
pub trait SubscriptionClient<R: SubscribeRequest> {
    type Error: std::error::Error;
    type Stream: futures_core::stream::Stream<Item = Result<R::Content, Self::Error>>;

    async fn subscribe<'a>(&mut self, request: R) -> Result<Self::Stream, Self::Error>
    where
        R: 'a;
}

#[async_trait::async_trait]
pub trait BroadcastClient<M: BroadcastMessage> {
    type Error: std::error::Error;
    type Stream: futures_core::stream::Stream<Item = Result<M, Self::Error>>;

    async fn broadcast<'a>(&mut self) -> Result<Self::Stream, Self::Error>
    where
        M: 'a;
}
