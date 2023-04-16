use crate::model::note::Note;
use crate::streaming::channel::NoOutgoing;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum HomeTimelineEvent {
    Note(Note),
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct Request {}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = HomeTimelineEvent;
    type Outgoing = NoOutgoing;

    const NAME: &'static str = "homeTimeline";
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
}
