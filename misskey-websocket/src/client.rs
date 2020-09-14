use std::fmt::{self, Debug};
use std::future::Future;

use crate::broker::{
    channel::{response_channel, ControlSender},
    model::{BrokerControl, SharedBrokerState},
    Broker, ReconnectConfig,
};
use crate::error::{Error, Result};
use crate::model::ApiRequestId;

use futures::{future::BoxFuture, sink::SinkExt};
use misskey_core::model::ApiResult;
use misskey_core::Client;
use serde_json::value;
use url::Url;

pub mod builder;
pub mod stream;

use stream::{Broadcast, Channel, SubNote};

/// Asynchronous WebSocket-based client for Misskey.
///
/// [`WebSocketClient`] can be constructed using [`WebSocketClient::connect`] or
/// [`WebSocketClientBuilder`][`builder::WebSocketClientBuilder`].
/// The latter is more flexible and intuitive.
///
/// You do not have to wrap this in [`Arc`][`std::sync::Arc`] and [`Mutex`][`std::sync::Mutex`]
/// to share it because [`WebSocketClient`] is already [`Clone`] and every methods of [`WebSocketClient`] takes `&self`, i.e. they does not require mutability.
#[derive(Clone)]
pub struct WebSocketClient {
    broker_tx: ControlSender,
    state: SharedBrokerState,
}

impl Debug for WebSocketClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct("WebSocketClient");

        match self.state.try_read() {
            Some(state) => debug.field("state", &state),
            None => debug.field("state", &"exiting"),
        };

        debug.finish()
    }
}

impl WebSocketClient {
    /// Connects to Misskey using WebSocket, and returns [`WebSocketClient`].
    ///
    /// If `reconnect` contains [`ReconnectConfig`],
    /// then the returned client will automatically reconnect on disconnection.
    pub async fn connect(url: Url, reconnect: Option<ReconnectConfig>) -> Result<WebSocketClient> {
        let (broker_tx, state) = Broker::spawn(url, reconnect).await?;
        Ok(WebSocketClient { broker_tx, state })
    }

    /// Captures the note specified by `id`.
    ///
    /// The returned [`SubNote`] implements [`Stream`][`futures::Stream`] so that note events can be retrieved asynchronously via it.
    pub fn subscribe_note<E, Id>(
        &self,
        id: Id,
    ) -> impl Future<Output = Result<SubNote<E>>> + 'static
    where
        E: misskey_core::streaming::SubNoteEvent,
        Id: Into<misskey_core::streaming::SubNoteId>,
    {
        SubNote::subscribe(
            id.into(),
            self.broker_tx.clone(),
            SharedBrokerState::clone(&self.state),
        )
    }

    /// Connects to the channel using `request`.
    ///
    /// The returned [`Channel`] implements [`Stream`][`futures::Stream`] and [`Sink`][`futures::Sink`] so that you can exchange messages with channels on it.
    pub fn channel<'a, R>(
        &self,
        request: R,
    ) -> impl Future<Output = Result<Channel<R::Incoming, R::Outgoing>>> + 'a
    where
        R: misskey_core::streaming::ConnectChannelRequest + 'a,
    {
        Channel::connect(
            request,
            self.broker_tx.clone(),
            SharedBrokerState::clone(&self.state),
        )
    }

    /// Receive messages from the broadcast stream.
    pub fn broadcast<E>(&self) -> impl Future<Output = Result<Broadcast<E>>> + 'static
    where
        E: misskey_core::streaming::BroadcastEvent,
    {
        Broadcast::start(
            self.broker_tx.clone(),
            SharedBrokerState::clone(&self.state),
        )
    }
}

impl Client for WebSocketClient {
    type Error = Error;

    fn request<'a, R>(&'a self, request: R) -> BoxFuture<'a, Result<ApiResult<R::Response>>>
    where
        R: misskey_core::Request + 'a,
    {
        let id = ApiRequestId::uuid();

        // limit the use of `R` to the outside of `async`
        // in order not to require `Send` on `R`
        let serialized_request = serde_json::to_value(request);

        Box::pin(async move {
            let (tx, rx) = response_channel(SharedBrokerState::clone(&self.state));
            self.broker_tx
                .clone()
                .send(BrokerControl::Api {
                    id,
                    endpoint: R::ENDPOINT,
                    data: serialized_request?,
                    sender: tx,
                })
                .await?;

            Ok(match rx.recv().await? {
                ApiResult::Ok(x) => ApiResult::Ok(value::from_value(x)?),
                ApiResult::Err { error } => ApiResult::Err { error },
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use super::{builder::WebSocketClientBuilder, WebSocketClient};

    use futures::stream::StreamExt;
    use misskey_core::Client;
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

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<WebSocketClient>();
    }

    #[test]
    fn test_sync() {
        fn assert_send<T: Sync>() {}
        assert_send::<WebSocketClient>();
    }

    #[cfg_attr(feature = "tokio-runtime", tokio::test)]
    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    async fn request() {
        let client = test_client().await;

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
        let client = test_client().await;
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
