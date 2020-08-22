use crate::model::user::User;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub username: Option<String>,
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<bool>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

impl misskey_core::Request for Request {
    type Response = Vec<User>;
    const ENDPOINT: &'static str = "users/search-by-username-and-host";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client
            .test(Request {
                username: Some("test".to_string()),
                host: None,
                detail: None,
                limit: None,
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_host() {
        let mut client = TestClient::new();
        client
            .test(Request {
                username: Some("test".to_string()),
                host: Some("dummy".to_string()), // TODO: use proper host string
                detail: None,
                limit: None,
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let mut client = TestClient::new();
        client
            .test(Request {
                username: Some("admin".to_string()),
                host: None,
                detail: Some(false),
                limit: None,
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let mut client = TestClient::new();
        client
            .test(Request {
                username: Some("admin".to_string()),
                host: None,
                detail: None,
                limit: Some(100),
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_offset() {
        let mut client = TestClient::new();
        client
            .test(Request {
                username: Some("admin".to_string()),
                host: None,
                detail: None,
                limit: None,
                offset: Some(5),
            })
            .await;
    }
}
