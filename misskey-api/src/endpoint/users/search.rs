use crate::model::user::User;

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[builder(setter(into))]
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub local_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub detail: Option<bool>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub offset: Option<u64>,
}

impl misskey_core::Request for Request {
    type Response = Vec<User>;
    const ENDPOINT: &'static str = "users/search";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client
            .test(Request {
                query: "test".to_string(),
                local_only: None,
                detail: None,
                limit: None,
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        client
            .test(Request {
                query: "admin".to_string(),
                local_only: Some(true),
                detail: Some(false),
                limit: None,
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .test(Request {
                query: "test".to_string(),
                local_only: None,
                detail: None,
                limit: Some(100),
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_offset() {
        let client = TestClient::new();
        client
            .test(Request {
                query: "admin".to_string(),
                local_only: None,
                detail: None,
                limit: None,
                offset: Some(5),
            })
            .await;
    }
}
