use crate::model::log::{Log, LogLevel};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub level: Option<LogLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub domain: Option<String>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Log>;
    const ENDPOINT: &'static str = "admin/logs";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.admin.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .admin
            .test(Request {
                limit: Some(100),
                level: None,
                domain: None,
            })
            .await;
    }

    // TODO: test with all `LogLevel`
    #[tokio::test]
    async fn request_with_options() {
        use crate::model::log::LogLevel;

        let client = TestClient::new();

        client
            .admin
            .test(Request {
                limit: None,
                level: Some(LogLevel::Debug),
                #[cfg(not(feature = "12-89-0"))]
                domain: Some("chart remote -resolve-user".to_string()),
                #[cfg(feature = "12-89-0")]
                domain: Some("chart".to_string()),
            })
            .await;
    }
}
