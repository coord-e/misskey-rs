use crate::model::{id::Id, note::Note, role::Role};
use crate::streaming::channel::NoOutgoing;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum RoleTimelineEvent {
    Note(Note),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub role_id: Id<Role>,
}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = RoleTimelineEvent;
    type Outgoing = NoOutgoing;

    const NAME: &'static str = "roleTimeline";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{http::TestClient as HttpTestClient, websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let http_client = HttpTestClient::new();
        let client = TestClient::new().await;
        let role = http_client
            .admin
            .test(crate::endpoint::admin::roles::create::Request::default())
            .await;

        let mut stream = client.channel(Request { role_id: role.id }).await.unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream() {
        let http_client = HttpTestClient::new();
        let client = TestClient::new().await;
        let (new_user, new_client) = http_client.admin.create_user().await;
        let role = http_client
            .admin
            .test(crate::endpoint::admin::roles::create::Request::default())
            .await;
        http_client
            .admin
            .test(
                crate::endpoint::admin::roles::assign::Request::builder()
                    .role_id(role.id)
                    .user_id(new_user.id)
                    .build(),
            )
            .await;

        let mut stream = client.channel(Request { role_id: role.id }).await.unwrap();

        future::join(
            new_client.create_note(Some("The world is fancy!"), None, None),
            async { stream.next().await.unwrap().unwrap() },
        )
        .await;
    }
}
