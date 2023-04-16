use crate::model::{id::Id, notification::Notification};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub notification_id: Id<Notification>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "notifications/read";
}

#[cfg(feature = "12-111-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-111-0")))]
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestWithNotificationIds {
    pub notification_ids: Vec<Id<Notification>>,
}

#[cfg(feature = "12-111-0")]
impl misskey_core::Request for RequestWithNotificationIds {
    type Response = ();
    const ENDPOINT: &'static str = "notifications/read";
}

#[cfg(test)]
mod tests {
    use super::Request;
    #[cfg(feature = "12-111-0")]
    use super::RequestWithNotificationIds;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client
            .admin
            .test(
                crate::endpoint::notifications::create::Request::builder()
                    .body("hi")
                    .build(),
            )
            .await;

        let mut notification = None;
        while notification.is_none() {
            notification = client
                .admin
                .test(crate::endpoint::i::notifications::Request::default())
                .await
                .pop();
        }

        client
            .admin
            .test(Request {
                notification_id: notification.unwrap().id,
            })
            .await;
    }

    #[cfg(feature = "12-111-0")]
    #[tokio::test]
    async fn request_with_notification_ids() {
        let client = TestClient::new();
        client
            .admin
            .test(crate::endpoint::notifications::create::Request {
                body: "hi".to_string(),
                header: None,
                icon: None,
            })
            .await;

        let notifications = client
            .admin
            .test(crate::endpoint::i::notifications::Request::default())
            .await;

        client
            .admin
            .test(RequestWithNotificationIds {
                notification_ids: notifications
                    .iter()
                    .map(|notification| notification.id)
                    .collect(),
            })
            .await;
    }
}
