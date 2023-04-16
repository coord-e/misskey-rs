use crate::model::{abuse_user_report::AbuseUserReport, id::Id, user::User};
use crate::streaming::channel::NoOutgoing;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum AdminStreamEvent {
    #[serde(rename_all = "camelCase")]
    NewAbuseUserReport {
        id: Id<AbuseUserReport>,
        #[cfg(any(docsrs, not(feature = "12-49-0")))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "12-49-0"))))]
        user_id: Id<User>,
        #[cfg(feature = "12-49-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "12-49-0")))]
        target_user_id: Id<User>,
        reporter_id: Id<User>,
        comment: String,
    },
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct Request {}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = AdminStreamEvent;
    type Outgoing = NoOutgoing;

    const NAME: &'static str = "admin";
}

#[cfg(test)]
mod tests {
    use super::{AdminStreamEvent, Request};
    use crate::test::{http::TestClient as HttpTestClient, websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;
        let mut stream = client.admin.channel(Request::default()).await.unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream() {
        let http_client = HttpTestClient::new();
        let client = TestClient::new().await;
        let (user, _) = http_client.admin.create_user().await;
        let mut stream = client.admin.channel(Request::default()).await.unwrap();

        future::join(
            http_client.test(crate::endpoint::users::report_abuse::Request {
                user_id: user.id,
                comment: "looks bad".to_string(),
            }),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        AdminStreamEvent::NewAbuseUserReport { .. } => break,
                    }
                }
            },
        )
        .await;
    }
}
