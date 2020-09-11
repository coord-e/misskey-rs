use crate::model::messaging::{MessagingMessage, MessagingMessageId};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum MessagingIndexStreamEvent {
    Message(MessagingMessage),
    Read(Vec<MessagingMessageId>),
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct Request {}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = MessagingIndexStreamEvent;
    type Outgoing = ();

    const NAME: &'static str = "messagingIndex";
}

#[cfg(test)]
mod tests {
    use super::{MessagingIndexStreamEvent, Request};
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;
        let mut stream = client.user.channel(Request::default()).await.unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream_message() {
        let client = TestClient::new().await;
        let user = client.user.me().await;
        let mut stream = client.user.channel(Request::default()).await.unwrap();

        future::join(
            client
                .admin
                .test(crate::endpoint::messaging::messages::create::Request {
                    text: Some("hi".to_string()),
                    user_id: Some(user.id),
                    group_id: None,
                    file_id: None,
                }),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        MessagingIndexStreamEvent::Message(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }

    #[tokio::test]
    async fn stream_read() {
        let client = TestClient::new().await;
        let user = client.user.me().await;
        let message = client
            .admin
            .test(crate::endpoint::messaging::messages::create::Request {
                text: Some("hi".to_string()),
                user_id: Some(user.id.clone()),
                group_id: None,
                file_id: None,
            })
            .await;
        let mut stream = client.user.channel(Request::default()).await.unwrap();

        future::join(
            client
                .user
                .test(crate::endpoint::messaging::messages::read::Request {
                    message_id: message.id,
                }),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        MessagingIndexStreamEvent::Read(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }
}
