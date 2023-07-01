use crate::model::meta::Meta;

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<bool>,
}

impl misskey_core::Request for Request {
    type Response = Meta;
    const ENDPOINT: &'static str = "meta";
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

    #[cfg(not(feature = "12-109-0"))]
    #[tokio::test]
    async fn request_by_admin() {
        let client = TestClient::new();
        client.admin.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_details() {
        let client = TestClient::new();
        client.test(Request { detail: Some(true) }).await;
    }

    #[tokio::test]
    async fn request_without_details() {
        let client = TestClient::new();
        client
            .test(Request {
                detail: Some(false),
            })
            .await;
    }
}
