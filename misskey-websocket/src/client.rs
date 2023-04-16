use std::fmt::{self, Debug};

#[cfg(not(feature = "12-111-0"))]
use crate::broker::{channel::response_channel, model::BrokerControl};
use crate::broker::{channel::ControlSender, model::SharedBrokerState, Broker, ReconnectConfig};
use crate::error::{Error, Result};
#[cfg(not(feature = "12-111-0"))]
use crate::model::ApiRequestId;
use crate::model::SubNoteId;

use async_tungstenite::tungstenite::http::HeaderMap;
#[cfg(not(feature = "12-111-0"))]
use futures_util::sink::SinkExt;
use futures_util::{
    future::{BoxFuture, FutureExt, TryFutureExt},
    sink::Sink,
    stream::{BoxStream, Stream, StreamExt},
};
use misskey_core::streaming::{BoxStreamSink, StreamingClient};
#[cfg(not(feature = "12-111-0"))]
use misskey_core::{model::ApiResult, Client};
#[cfg(not(feature = "12-111-0"))]
use serde_json::value;
use url::Url;

pub mod builder;
pub mod stream;

use builder::WebSocketClientBuilder;
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
    pub async fn connect(url: Url) -> Result<WebSocketClient> {
        WebSocketClient::connect_with_config(url, ReconnectConfig::default()).await
    }

    /// Connects to Misskey using WebSocket with given additional headers, and returns [`WebSocketClient`].
    pub async fn connect_with_headers(
        url: Url,
        additional_headers: HeaderMap,
    ) -> Result<WebSocketClient> {
        WebSocketClient::connect_with_headers_and_config(
            url,
            additional_headers,
            ReconnectConfig::default(),
        )
        .await
    }

    /// Connects to Misskey using WebSocket with a given reconnect configuration, and returns [`WebSocketClient`].
    pub async fn connect_with_config(
        url: Url,
        reconnect_config: ReconnectConfig,
    ) -> Result<WebSocketClient> {
        WebSocketClient::connect_with_headers_and_config(
            url,
            HeaderMap::default(),
            reconnect_config,
        )
        .await
    }

    /// Connects to Misskey using WebSocket with given additional headers and
    /// reconnect configuration, and returns [`WebSocketClient`].
    pub async fn connect_with_headers_and_config(
        url: Url,
        additional_headers: HeaderMap,
        reconnect_config: ReconnectConfig,
    ) -> Result<WebSocketClient> {
        let (broker_tx, state) = Broker::spawn(url, additional_headers, reconnect_config).await?;
        Ok(WebSocketClient { broker_tx, state })
    }

    /// Creates a new builder instance with `url`.
    /// All configurations are set to default.
    ///
    /// This function is identical to [`WebSocketClientBuilder::new`].
    pub fn builder<T>(url: T) -> WebSocketClientBuilder
    where
        T: TryInto<Url>,
        T::Error: Into<Error>,
    {
        WebSocketClientBuilder::new(url)
    }

    /// Captures the note specified by `id`.
    ///
    /// The returned [`SubNote`] implements [`Stream`][stream]
    /// so that note events can be retrieved asynchronously via it.
    ///
    /// [stream]: futures_util::stream::Stream
    pub fn subnote<E, Id>(&self, note_id: Id) -> BoxFuture<'static, Result<SubNote<E>>>
    where
        E: misskey_core::streaming::SubNoteEvent,
        Id: Into<String>,
    {
        SubNote::subscribe(
            SubNoteId(note_id.into()),
            self.broker_tx.clone(),
            SharedBrokerState::clone(&self.state),
        )
        .boxed()
    }

    /// Connects to the channel using `request`.
    ///
    /// The returned [`Channel`] implements [`Stream`][stream] and [`Sink`][sink]
    /// so that you can exchange messages with channels on it.
    ///
    /// [stream]: futures_util::stream::Stream
    /// [sink]: futures_util::sink::Sink
    pub fn channel<R>(
        &self,
        request: R,
    ) -> BoxFuture<'static, Result<Channel<R::Incoming, R::Outgoing>>>
    where
        R: misskey_core::streaming::ConnectChannelRequest,
    {
        Channel::connect(
            request,
            self.broker_tx.clone(),
            SharedBrokerState::clone(&self.state),
        )
    }

    /// Receive messages from the broadcast stream.
    ///
    /// The returned [`Broadcast`] implements [`Stream`][stream]
    /// so that broadcast events can be retrieved asynchronously via it.
    ///
    /// [stream]: futures_util::stream::Stream
    pub fn broadcast<E>(&self) -> BoxFuture<'static, Result<Broadcast<E>>>
    where
        E: misskey_core::streaming::BroadcastEvent,
    {
        Broadcast::start(
            self.broker_tx.clone(),
            SharedBrokerState::clone(&self.state),
        )
        .boxed()
    }
}

// API call through streaming is disabled for Misskey v12.111.0 and later.
// https://github.com/misskey-dev/misskey/commit/3770bb6
#[cfg(not(feature = "12-111-0"))]
impl Client for WebSocketClient {
    type Error = Error;

    fn request<R: misskey_core::Request>(
        &self,
        request: R,
    ) -> BoxFuture<Result<ApiResult<R::Response>>> {
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

fn boxed_stream_sink<'a, I, O, E, S>(s: S) -> BoxStreamSink<'a, I, O, E>
where
    S: Stream<Item = std::result::Result<I, E>> + Sink<O, Error = E> + Send + 'a,
{
    Box::pin(s)
}

impl StreamingClient for WebSocketClient {
    type Error = Error;

    fn subnote<E>(&self, note_id: String) -> BoxFuture<Result<BoxStream<Result<E>>>>
    where
        E: misskey_core::streaming::SubNoteEvent,
    {
        Box::pin(async move {
            Ok(SubNote::subscribe(
                SubNoteId(note_id),
                self.broker_tx.clone(),
                SharedBrokerState::clone(&self.state),
            )
            .await?
            .boxed())
        })
    }

    fn channel<R>(
        &self,
        request: R,
    ) -> BoxFuture<Result<misskey_core::streaming::ChannelStream<R, Error>>>
    where
        R: misskey_core::streaming::ConnectChannelRequest,
    {
        Channel::connect(
            request,
            self.broker_tx.clone(),
            SharedBrokerState::clone(&self.state),
        )
        .map_ok(boxed_stream_sink)
        .boxed()
    }

    fn broadcast<E>(&self) -> BoxFuture<Result<BoxStream<Result<E>>>>
    where
        E: misskey_core::streaming::BroadcastEvent,
    {
        Box::pin(async move {
            Ok(Broadcast::start(
                self.broker_tx.clone(),
                SharedBrokerState::clone(&self.state),
            )
            .await?
            .boxed())
        })
    }
}

#[cfg(not(feature = "12-111-0"))]
#[cfg(test)]
mod tests {
    use super::{builder::WebSocketClientBuilder, WebSocketClient};

    use futures_util::stream::StreamExt;
    use misskey_core::Client;
    use misskey_test::{self, env};

    #[cfg(feature = "tokio02-runtime")]
    use tokio02 as tokio;

    async fn test_client() -> WebSocketClient {
        misskey_test::init_logger();

        WebSocketClientBuilder::new(env::websocket_url())
            .token(env::token())
            .header("User-Agent", "misskey-rs")
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
    #[cfg_attr(feature = "tokio02-runtime", tokio02::test)]
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
    #[cfg_attr(feature = "tokio02-runtime", tokio02::test)]
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

        let mut stream = client
            .subnote::<misskey_api::streaming::note::NoteUpdateEvent, _>(note.id.to_string())
            .await
            .unwrap();

        futures_util::future::join(
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
