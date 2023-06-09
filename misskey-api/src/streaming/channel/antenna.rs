use crate::model::{antenna::Antenna, id::Id, note::Note};
use crate::streaming::channel::NoOutgoing;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum AntennaStreamEvent {
    Note(Note),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub antenna_id: Id<Antenna>,
}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = AntennaStreamEvent;
    type Outgoing = NoOutgoing;

    const NAME: &'static str = "antenna";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{http::TestClient as HttpTestClient, websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        use crate::model::{antenna::AntennaSource, query::Query};

        let http_client = HttpTestClient::new();
        let client = TestClient::new().await;
        let antenna = http_client
            .test(crate::endpoint::antennas::create::Request {
                name: "test".to_string(),
                src: AntennaSource::All,
                user_list_id: None,
                #[cfg(feature = "12-10-0")]
                user_group_id: None,
                keywords: Query::from_vec(vec![vec!["hello".to_string(), "awesome".to_string()]]),
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Query::default(),
                users: Vec::new(),
                case_sensitive: false,
                with_replies: false,
                with_file: false,
                notify: false,
            })
            .await;

        let mut stream = client
            .channel(Request {
                antenna_id: antenna.id,
            })
            .await
            .unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream() {
        use crate::model::{antenna::AntennaSource, query::Query};

        let http_client = HttpTestClient::new();
        let client = TestClient::new().await;
        let antenna = http_client
            .test(crate::endpoint::antennas::create::Request {
                name: "test".to_string(),
                src: AntennaSource::All,
                user_list_id: None,
                #[cfg(feature = "12-10-0")]
                user_group_id: None,
                keywords: Query::from_vec(vec![vec!["hello".to_string()]]),
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Query::default(),
                users: Vec::new(),
                case_sensitive: false,
                with_replies: false,
                with_file: false,
                notify: false,
            })
            .await;

        let mut stream = client
            .channel(Request {
                antenna_id: antenna.id,
            })
            .await
            .unwrap();

        future::join(http_client.create_note(Some("hello"), None, None), async {
            stream.next().await.unwrap().unwrap()
        })
        .await;
    }
}
