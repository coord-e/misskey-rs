use crate::model::{id::Id, messaging::MessagingMessage};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub message_id: Id<MessagingMessage>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "messaging/messages/delete";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let admin = client.admin.me().await;
        let message = client
            .user
            .test(crate::endpoint::messaging::messages::create::Request {
                text: Some("hi".to_string()),
                user_id: Some(admin.id),
                group_id: None,
                file_id: None,
            })
            .await;
        client
            .user
            .test(Request {
                message_id: message.id,
            })
            .await;
    }
}
