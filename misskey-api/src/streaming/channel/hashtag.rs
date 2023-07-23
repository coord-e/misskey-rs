use crate::model::{note::Note, query::Query};
use crate::streaming::channel::NoOutgoing;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum HashtagEvent {
    Note(Note),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub q: Query<String>,
}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = HashtagEvent;
    type Outgoing = NoOutgoing;

    const NAME: &'static str = "hashtag";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::model::query::Query;
    use crate::test::{http::TestClient as HttpTestClient, websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;

        let mut stream = client
            .channel(Request {
                q: Query(vec![vec!["tag".to_string()]]),
            })
            .await
            .unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream() {
        let http_client = HttpTestClient::new();
        let client = TestClient::new().await;
        let mut stream = client
            .channel(Request {
                q: Query(vec![vec!["test".to_string(), "good".to_string()]]),
            })
            .await
            .unwrap();

        future::join(
            http_client.create_note(Some("#test #good"), None, None),
            async { stream.next().await.unwrap().unwrap() },
        )
        .await;
    }
}
