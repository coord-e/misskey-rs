use crate::model::{abuse_user_report::AbuseUserReportId, user::UserId};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum AdminStreamEvent {
    #[serde(rename_all = "camelCase")]
    NewAbuseUserReport {
        id: AbuseUserReportId,
        user_id: UserId,
        reporter_id: UserId,
        comment: String,
    },
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct Request {}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = AdminStreamEvent;
    type Outgoing = ();

    const NAME: &'static str = "admin";
}

#[cfg(test)]
mod tests {
    use super::{AdminStreamEvent, Request};
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};
    use misskey_core::streaming::ChannelClient;

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let mut client = TestClient::new().await;
        let mut stream = client.admin.connect(Request::default()).await.unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream() {
        let mut client = TestClient::new().await;
        let (user, _) = client.admin.create_user().await;
        let mut stream = client.admin.connect(Request::default()).await.unwrap();

        future::join(
            client.test(crate::endpoint::users::report_abuse::Request {
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
