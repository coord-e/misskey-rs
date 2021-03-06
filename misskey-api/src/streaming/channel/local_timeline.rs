use crate::model::note::Note;
use crate::streaming::channel::NoOutgoing;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum LocalTimelineEvent {
    Note(Note),
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct Request {}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = LocalTimelineEvent;
    type Outgoing = NoOutgoing;

    const NAME: &'static str = "localTimeline";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;

        let mut stream = client.channel(Request::default()).await.unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream() {
        let client = TestClient::new().await;

        let mut stream = client.channel(Request::default()).await.unwrap();

        future::join(
            client.create_note(Some("The world is fancy!"), None, None),
            async { stream.next().await.unwrap().unwrap() },
        )
        .await;
    }
}
