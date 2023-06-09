use crate::model::{id::Id, note::Note, user::User, user_list::UserList};
use crate::streaming::channel::NoOutgoing;

use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum UserListEvent {
    // from userListStream
    UserAdded(User),
    UserRemoved(User),
    // from the channel
    Note(Note),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub list_id: Id<UserList>,
}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = UserListEvent;
    type Outgoing = NoOutgoing;

    const NAME: &'static str = "userList";
}

#[cfg(test)]
mod tests {
    use super::{Request, UserListEvent};
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;
        let list = client
            .test(crate::endpoint::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;

        let mut stream = client.channel(Request { list_id: list.id }).await.unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream_note() {
        let client = TestClient::new().await;
        let admin = client.admin.me().await;
        let list = client
            .user
            .test(crate::endpoint::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;
        client
            .user
            .test(crate::endpoint::users::lists::push::Request {
                list_id: list.id.clone(),
                user_id: admin.id,
            })
            .await;

        let mut stream = client
            .user
            .channel(Request { list_id: list.id })
            .await
            .unwrap();

        future::join(
            client
                .admin
                .create_note(Some("The world is fancy!"), None, None),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        UserListEvent::Note(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }

    #[cfg(not(feature = "12-108-0"))]
    #[tokio::test]
    async fn stream_added() {
        let client = TestClient::new().await;
        let admin = client.admin.me().await;
        let list = client
            .user
            .test(crate::endpoint::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;

        let mut stream = client
            .user
            .channel(Request {
                list_id: list.id.clone(),
            })
            .await
            .unwrap();

        future::join(
            client
                .user
                .test(crate::endpoint::users::lists::push::Request {
                    list_id: list.id,
                    user_id: admin.id,
                }),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        UserListEvent::UserAdded(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }

    #[cfg(not(feature = "12-108-0"))]
    #[tokio::test]
    async fn stream_removed() {
        let client = TestClient::new().await;
        let admin = client.admin.me().await;
        let list = client
            .user
            .test(crate::endpoint::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;
        client
            .user
            .test(crate::endpoint::users::lists::push::Request {
                list_id: list.id.clone(),
                user_id: admin.id.clone(),
            })
            .await;

        let mut stream = client
            .user
            .channel(Request {
                list_id: list.id.clone(),
            })
            .await
            .unwrap();

        future::join(
            client
                .user
                .test(crate::endpoint::users::lists::pull::Request {
                    list_id: list.id,
                    user_id: admin.id,
                }),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        UserListEvent::UserRemoved(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }
}
