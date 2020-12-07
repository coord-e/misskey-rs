use crate::model::{id::Id, messaging::MessagingMessage, user::User, user_group::UserGroup};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum MessagingStreamEvent {
    Message(MessagingMessage),
    Deleted(Id<MessagingMessage>),
    Read(Vec<Id<MessagingMessage>>),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum Message {
    Read { id: Id<MessagingMessage> },
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Request {
    Otherparty(Id<User>),
    Group(Id<UserGroup>),
}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = MessagingStreamEvent;
    type Outgoing = Message;

    const NAME: &'static str = "messaging";
}

#[cfg(test)]
mod tests {
    use super::{Message, MessagingStreamEvent, Request};
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, SinkExt, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe_otherparty() {
        let client = TestClient::new().await;
        let admin = client.admin.me().await;
        let mut stream = client
            .user
            .channel(Request::Otherparty(admin.id))
            .await
            .unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn subscribe_unsubscribe_group() {
        let client = TestClient::new().await;
        let group = client
            .test(crate::endpoint::users::groups::create::Request {
                name: "test".to_string(),
            })
            .await;
        let mut stream = client.user.channel(Request::Group(group.id)).await.unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream_message() {
        let client = TestClient::new().await;
        let user = client.user.me().await;
        let admin = client.admin.me().await;
        let mut stream = client
            .user
            .channel(Request::Otherparty(admin.id))
            .await
            .unwrap();

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
                        MessagingStreamEvent::Message(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }

    #[tokio::test]
    async fn stream_deleted() {
        let client = TestClient::new().await;
        let user = client.user.me().await;
        let admin = client.admin.me().await;
        let message = client
            .admin
            .test(crate::endpoint::messaging::messages::create::Request {
                text: Some("hi".to_string()),
                user_id: Some(user.id),
                group_id: None,
                file_id: None,
            })
            .await;
        let mut stream = client
            .user
            .channel(Request::Otherparty(admin.id))
            .await
            .unwrap();

        future::join(
            client
                .admin
                .test(crate::endpoint::messaging::messages::delete::Request {
                    message_id: message.id,
                }),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        MessagingStreamEvent::Deleted(_) => break,
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
        let admin = client.admin.me().await;
        let user = client.user.me().await;
        let message = client
            .user
            .test(crate::endpoint::messaging::messages::create::Request {
                text: Some("hi".to_string()),
                user_id: Some(admin.id.clone()),
                group_id: None,
                file_id: None,
            })
            .await;
        let mut user_stream = client
            .user
            .channel(Request::Otherparty(admin.id))
            .await
            .unwrap();
        let mut admin_stream = client
            .admin
            .channel(Request::Otherparty(user.id))
            .await
            .unwrap();

        future::join(
            async {
                admin_stream
                    .send(Message::Read {
                        id: message.id.clone(),
                    })
                    .await
                    .unwrap();
            },
            async {
                loop {
                    match user_stream.next().await.unwrap().unwrap() {
                        MessagingStreamEvent::Read(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }
}
