use crate::model::note::Note;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum HashtagEvent {
    Note(Note),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub q: Vec<Vec<String>>,
}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = HashtagEvent;
    type Outgoing = ();

    const NAME: &'static str = "hashtag";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};
    use misskey_core::streaming::ChannelClient;

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;

        let mut stream = client
            .connect(Request {
                q: vec![vec!["tag".to_string()]],
            })
            .await
            .unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream() {
        let client = TestClient::new().await;
        let mut stream = client
            .connect(Request {
                q: vec![vec!["test".to_string(), "good".to_string()]],
            })
            .await
            .unwrap();

        future::join(client.create_note(Some("#test #good"), None, None), async {
            stream.next().await.unwrap().unwrap()
        })
        .await;
    }
}
