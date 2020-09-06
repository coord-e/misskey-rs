use crate::model::messaging::MessagingMessage;

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<bool>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
}

impl misskey_core::Request for Request {
    type Response = Vec<MessagingMessage>;
    const ENDPOINT: &'static str = "messaging/history";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_option() {
        let client = TestClient::new();
        client
            .test(Request {
                group: Some(true),
                limit: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .test(Request {
                group: None,
                limit: Some(100),
            })
            .await;
    }
}
