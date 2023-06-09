use crate::model::user::User;

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[builder(default, setter(strip_option, into))]
    pub username: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub detail: Option<bool>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[cfg(not(feature = "12-93-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-93-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub offset: Option<u64>,
}

impl misskey_core::Request for Request {
    type Response = Vec<User>;
    const ENDPOINT: &'static str = "users/search-by-username-and-host";
}

#[cfg(not(feature = "12-93-0"))]
impl_offset_pagination!(Request, User);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client
            .test(Request {
                username: Some("test".to_string()),
                host: None,
                detail: None,
                limit: None,
                #[cfg(not(feature = "12-93-0"))]
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_host() {
        let client = TestClient::new();
        client
            .test(Request {
                username: Some("test".to_string()),
                host: Some("dummy".to_string()), // TODO: use proper host string
                detail: None,
                limit: None,
                #[cfg(not(feature = "12-93-0"))]
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        client
            .test(Request {
                username: Some("admin".to_string()),
                host: None,
                detail: Some(false),
                limit: None,
                #[cfg(not(feature = "12-93-0"))]
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .test(Request {
                username: Some("admin".to_string()),
                host: None,
                detail: None,
                limit: Some(100),
                #[cfg(not(feature = "12-93-0"))]
                offset: None,
            })
            .await;
    }

    #[cfg(not(feature = "12-93-0"))]
    #[tokio::test]
    async fn request_with_offset() {
        let client = TestClient::new();
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
