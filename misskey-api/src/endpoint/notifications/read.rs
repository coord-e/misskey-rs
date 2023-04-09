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

#[cfg(test)]
mod tests {
    use super::Request;
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
}
