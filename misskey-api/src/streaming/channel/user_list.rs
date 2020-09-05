use crate::model::{note::Note, user::User, user_list::UserListId};

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
    pub list_id: UserListId,
}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = UserListEvent;
    type Outgoing = ();

    const NAME: &'static str = "userList";
}

#[cfg(test)]
mod tests {
    use super::{Request, UserListEvent};
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};
    use misskey_core::streaming::ChannelClient;

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let mut client = TestClient::new().await;
        let list = client
            .test(crate::endpoint::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;

        let mut stream = client.connect(Request { list_id: list.id }).await.unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream_note() {
        let mut client = TestClient::new().await;
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
            .connect(Request { list_id: list.id })
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

    #[tokio::test]
    async fn stream_added() {
        let mut client = TestClient::new().await;
        let admin = client.admin.me().await;
        let list = client
            .user
            .test(crate::endpoint::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;

        let mut stream = client
            .user
            .connect(Request {
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

    #[tokio::test]
    async fn stream_removed() {
        let mut client = TestClient::new().await;
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
            .connect(Request {
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
