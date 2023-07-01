use crate::model::note::Note;
use crate::streaming::channel::NoOutgoing;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum LocalTimelineEvent {
    Note(Note),
}
#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub with_replies: Option<bool>,
}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = LocalTimelineEvent;
    type Outgoing = NoOutgoing;

    const NAME: &'static str = "localTimeline";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{http::TestClient as HttpTestClient, websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;

        let mut stream = client.channel(Request::default()).await.unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream() {
        let http_client = HttpTestClient::new();
        let client = TestClient::new().await;

        let mut stream = client.channel(Request::default()).await.unwrap();

        future::join(
            http_client.create_note(Some("The world is fancy!"), None, None),
            async { stream.next().await.unwrap().unwrap() },
        )
        .await;
    }

    #[cfg(feature = "13-13-0")]
    #[tokio::test]
    async fn stream_with_replies() {
        let http_client = HttpTestClient::new();
        let client = TestClient::new().await;
        let note = http_client
            .admin
            .create_note(Some("The world is fancy!"), None, None)
            .await;
        let (_, new_client) = http_client.admin.create_user().await;

        let mut stream = client
            .channel(Request {
                with_replies: Some(true),
            })
            .await
            .unwrap();

        future::join(
            new_client.create_note(Some("The world is fancy!"), None, Some(note.id)),
            async { stream.next().await.unwrap().unwrap() },
        )
        .await;
    }
}
