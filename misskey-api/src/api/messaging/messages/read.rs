use crate::model::messaging::MessagingMessageId;

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub message_id: MessagingMessageId,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "messaging/messages/read";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let admin = client.admin.me().await;
        let message = client
            .user
            .test(crate::api::messaging::messages::create::Request {
                text: Some("hi".to_string()),
                user_id: Some(admin.id),
                group_id: None,
                file_id: None,
            })
            .await;
        client
            .admin
            .test(Request {
                message_id: message.id,
            })
            .await;
    }
}
