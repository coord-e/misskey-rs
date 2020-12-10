use crate::model::clip::Clip;

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// [ 1 .. 100 ] characters
    #[builder(default, setter(into))]
    pub name: String,
    #[cfg(feature = "12-57-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub is_public: Option<bool>,
    #[cfg(feature = "12-57-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub description: Option<String>,
}

impl misskey_core::Request for Request {
    type Response = Clip;
    const ENDPOINT: &'static str = "clips/create";
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
                name: "clip".to_string(),
                #[cfg(feature = "12-57-0")]
                is_public: None,
                #[cfg(feature = "12-57-0")]
                description: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        client
            .test(Request {
                name: "clip".to_string(),
                #[cfg(feature = "12-57-0")]
                is_public: Some(true),
                #[cfg(feature = "12-57-0")]
                description: Some("test".to_string()),
            })
            .await;
    }
}
