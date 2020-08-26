use crate::streaming::api::{BroadcastItem, SubscriptionRequest};

#[async_trait::async_trait]
pub trait BroadcastClient<I: BroadcastItem> {
    type Error: std::error::Error;
    type Stream: futures_core::stream::Stream<Item = Result<I, Self::Error>>;

    async fn broadcast<'a>(&mut self) -> Result<Self::Stream, Self::Error>
    where
        I: 'a;
}

#[async_trait::async_trait]
pub trait SubscriptionClient<R: SubscriptionRequest> {
    type Error: std::error::Error;
    type Stream: futures_core::stream::Stream<Item = Result<R::Item, Self::Error>>;

    async fn subscribe<'a>(&mut self, request: R) -> Result<Self::Stream, Self::Error>
    where
        R: 'a;
}
