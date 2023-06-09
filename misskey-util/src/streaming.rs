use crate::error::Error;

use futures::{
    future::BoxFuture,
    stream::{BoxStream, StreamExt, TryStreamExt},
};
#[cfg(feature = "12-47-0")]
use misskey_api::model::channel::Channel;
use misskey_api::model::{antenna::Antenna, note::Note, query::Query, user_list::UserList};
use misskey_api::{
    streaming::{self, channel},
    EntityRef,
};
use misskey_core::streaming::StreamingClient;

/// An extension trait for [`StreamingClient`][client] that provides convenient high-level APIs.
///
/// [client]: misskey_core::streaming::StreamingClient
///
/// # Streams
///
/// The methods of [`StreamingClientExt`] return ([`Future`][future] that outputs) a
/// [`Stream`][stream] that receives items from the server asynchronously.
/// You can use methods from [`TryStreamExt`][try_stream_ext] or [`StreamExt`][stream_ext]
/// to work with these streams.
///
/// [future]: futures::future::Future
/// [stream]: futures::stream::Stream
/// [try_stream_ext]: futures::stream::TryStreamExt
/// [stream_ext]: futures::stream::StreamExt
#[allow(clippy::type_complexity)]
pub trait StreamingClientExt: StreamingClient + Sync {
    /// Subscribes to the specified note and returns a stream to receive the events.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::StreamingClientExt;
    /// # use misskey_util::ClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # use misskey_api as misskey;
    /// # let http_client = misskey_test::test_client().await?;
    /// # let ws_client = misskey_test::test_websocket_client(misskey_test::env::token()).await?;
    /// # misskey_test::persist(std::time::Duration::from_secs(3), async move {
    /// use futures::stream::TryStreamExt;
    /// use misskey::streaming::note::NoteUpdateEvent;
    ///
    /// let note = http_client.create_note("Hello!").await?;
    /// let mut note_stream = ws_client.subscribe_note(&note).await?;
    /// // Wait for the next event in the stream.
    /// while let Some(event) = note_stream.try_next().await? {
    ///     match event {
    ///         // Check if the event is 'reacted'
    ///         NoteUpdateEvent::Reacted { reaction, user_id } => {
    ///             println!("reacted by {}: {}", user_id, reaction);
    ///         }
    ///         // other events are just ignored here
    ///         _ => {}
    ///    }
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// # }).await
    /// # }
    /// ```
    fn subscribe_note(
        &self,
        note: impl EntityRef<Note>,
    ) -> BoxFuture<
        Result<
            BoxStream<Result<streaming::note::NoteUpdateEvent, Error<Self::Error>>>,
            Error<Self::Error>,
        >,
    > {
        let note_id = note.entity_ref().to_string();
        Box::pin(async move {
            Ok(self
                .subnote(note_id)
                .await
                .map_err(Error::Client)?
                .map_err(Error::Client)
                .boxed())
        })
    }

    /// Returns a stream to receive the events from the main stream.
    ///
    /// Note that currently it is not possible to have multiple connections to the main stream from
    /// the same client. If you try to do so, the `Future` returned by this method will not complete.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::StreamingClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let http_client = misskey_test::test_client().await?;
    /// # let ws_client = misskey_test::test_websocket_client(misskey_test::env::token()).await?;
    /// # mod misskey {
    /// #   pub use misskey_api::streaming;
    /// #   pub use misskey_util::ClientExt;
    /// # }
    /// # misskey_test::persist(std::time::Duration::from_secs(3), async move {
    /// use futures::stream::TryStreamExt;
    /// use misskey::ClientExt;
    /// use misskey::streaming::channel::main::MainStreamEvent;
    ///
    /// let mut main_stream = ws_client.main_stream().await?;
    /// // Wait for the next event in the main stream.
    /// while let Some(event) = main_stream.try_next().await? {
    ///     match event {
    ///         // Check if the event is 'followed'
    ///         MainStreamEvent::Followed(user) => {
    ///             // Follow back `user` if you haven't already.
    ///             if !http_client.is_following(&user).await? {
    ///                 http_client.follow(&user).await?;
    ///             }
    ///         }
    ///         // other events are just ignored here
    ///         _ => {}
    ///    }
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// # }).await
    /// # }
    /// ```
    fn main_stream(
        &self,
    ) -> BoxFuture<
        Result<
            BoxStream<Result<channel::main::MainStreamEvent, Error<Self::Error>>>,
            Error<Self::Error>,
        >,
    > {
        Box::pin(async move {
            Ok(self
                .channel(channel::main::Request::default())
                .await
                .map_err(Error::Client)?
                .map_err(Error::Client)
                .boxed())
        })
    }

    /// Returns a stream to receive the notes in the home timeline.
    ///
    /// Note that currently it is not possible to have multiple connections to the home timeline from
    /// the same client. If you try to do so, the `Future` returned by this method will not complete.
    ///
    /// # Examples
    ///
    /// ```
    /// # use misskey_util::StreamingClientExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let http_client = misskey_test::test_client().await?;
    /// # let ws_client = misskey_test::test_websocket_client(misskey_test::env::token()).await?;
    /// # mod misskey {
    /// #   pub use misskey_api::model;
    /// #   pub use misskey_util::ClientExt;
    /// # }
    /// # misskey_test::persist(std::time::Duration::from_secs(3), async move {
    /// use futures::stream::TryStreamExt;
    /// use misskey::ClientExt;
    /// use misskey::model::note::Note;
    ///
    /// let mut home = ws_client.home_timeline().await?;
    /// // Wait for the next note in the home timeline.
    /// while let Some(note) = home.try_next().await? {
    ///     // if the note's text contains "Hello", react with "🙌".
    ///     match note {
    ///         Note { id, text: Some(text), .. } if text.contains("Hello") => {
    ///             http_client.react(id, "🙌").await?;
    ///         }
    ///         _ => {}
    ///     }
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// # }).await
    /// # }
    /// ```
    fn home_timeline(
        &self,
    ) -> BoxFuture<Result<BoxStream<Result<Note, Error<Self::Error>>>, Error<Self::Error>>> {
        use channel::home_timeline::{HomeTimelineEvent, Request};

        Box::pin(async move {
            Ok(self
                .channel(Request::default())
                .await
                .map_err(Error::Client)?
                .map_err(Error::Client)
                .map_ok(|HomeTimelineEvent::Note(note)| note)
                .boxed())
        })
    }

    /// Returns a stream to receive the notes in the local timeline.
    ///
    /// Note that currently it is not possible to have multiple connections to the local timeline from
    /// the same client. If you try to do so, the `Future` returned by this method will not complete.
    fn local_timeline(
        &self,
    ) -> BoxFuture<Result<BoxStream<Result<Note, Error<Self::Error>>>, Error<Self::Error>>> {
        use channel::local_timeline::{LocalTimelineEvent, Request};

        Box::pin(async move {
            Ok(self
                .channel(Request::default())
                .await
                .map_err(Error::Client)?
                .map_err(Error::Client)
                .map_ok(|LocalTimelineEvent::Note(note)| note)
                .boxed())
        })
    }

    /// Returns a stream to receive the notes in the social timeline.
    ///
    /// Note that currently it is not possible to have multiple connections to the social timeline from
    /// the same client. If you try to do so, the `Future` returned by this method will not complete.
    fn social_timeline(
        &self,
    ) -> BoxFuture<Result<BoxStream<Result<Note, Error<Self::Error>>>, Error<Self::Error>>> {
        use channel::hybrid_timeline::{HybridTimelineEvent, Request};

        Box::pin(async move {
            Ok(self
                .channel(Request::default())
                .await
                .map_err(Error::Client)?
                .map_err(Error::Client)
                .map_ok(|HybridTimelineEvent::Note(note)| note)
                .boxed())
        })
    }

    /// Returns a stream to receive the notes in the global timeline.
    ///
    /// Note that currently it is not possible to have multiple connections to the global timeline from
    /// the same client. If you try to do so, the `Future` returned by this method will not complete.
    fn global_timeline(
        &self,
    ) -> BoxFuture<Result<BoxStream<Result<Note, Error<Self::Error>>>, Error<Self::Error>>> {
        use channel::global_timeline::{GlobalTimelineEvent, Request};

        Box::pin(async move {
            Ok(self
                .channel(Request::default())
                .await
                .map_err(Error::Client)?
                .map_err(Error::Client)
                .map_ok(|GlobalTimelineEvent::Note(note)| note)
                .boxed())
        })
    }

    /// Returns a stream to receive the notes with the given hashtags.
    fn hashtag_timeline(
        &self,
        query: impl Into<Query<String>>,
    ) -> BoxFuture<Result<BoxStream<Result<Note, Error<Self::Error>>>, Error<Self::Error>>> {
        use channel::hashtag::{HashtagEvent, Request};

        let q = query.into();
        Box::pin(async move {
            Ok(self
                .channel(Request { q })
                .await
                .map_err(Error::Client)?
                .map_err(Error::Client)
                .map_ok(|HashtagEvent::Note(note)| note)
                .boxed())
        })
    }

    /// Returns a stream to receive notes in the timeline of the specified antenna.
    fn antenna_timeline(
        &self,
        antenna: impl EntityRef<Antenna>,
    ) -> BoxFuture<Result<BoxStream<Result<Note, Error<Self::Error>>>, Error<Self::Error>>> {
        use channel::antenna::{AntennaStreamEvent, Request};

        let antenna_id = antenna.entity_ref();
        Box::pin(async move {
            Ok(self
                .channel(Request { antenna_id })
                .await
                .map_err(Error::Client)?
                .map_err(Error::Client)
                .map_ok(|AntennaStreamEvent::Note(note)| note)
                .boxed())
        })
    }

    /// Returns a stream to receive notes in the timeline of the specified channel.
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    fn channel_timeline(
        &self,
        channel: impl EntityRef<Channel>,
    ) -> BoxFuture<Result<BoxStream<Result<Note, Error<Self::Error>>>, Error<Self::Error>>> {
        use channel::channel::{ChannelEvent, Request};

        let channel_id = channel.entity_ref();
        Box::pin(async move {
            Ok(self
                .channel(Request { channel_id })
                .await
                .map_err(Error::Client)?
                .map_err(Error::Client)
                .try_filter_map(|event| async move {
                    if let ChannelEvent::Note(note) = event {
                        Ok(Some(note))
                    } else {
                        Ok(None)
                    }
                })
                .boxed())
        })
    }

    /// Returns a stream to receive notes in the timeline of the specified user list.
    fn user_list_timeline(
        &self,
        list: impl EntityRef<UserList>,
    ) -> BoxFuture<Result<BoxStream<Result<Note, Error<Self::Error>>>, Error<Self::Error>>> {
        use channel::user_list::{Request, UserListEvent};

        let list_id = list.entity_ref();
        Box::pin(async move {
            Ok(self
                .channel(Request { list_id })
                .await
                .map_err(Error::Client)?
                .map_err(Error::Client)
                .try_filter_map(|event| async move {
                    if let UserListEvent::Note(note) = event {
                        Ok(Some(note))
                    } else {
                        Ok(None)
                    }
                })
                .boxed())
        })
    }
}

impl<C: StreamingClient + Sync> StreamingClientExt for C {}
