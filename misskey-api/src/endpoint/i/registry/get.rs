use crate::model::registry::{RegistryKey, RegistryScope, RegistryValue};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub key: RegistryKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<RegistryScope>,
}

impl misskey_core::Request for Request {
    type Response = RegistryValue;
    const ENDPOINT: &'static str = "i/registry/get";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_simple() {
        use serde_json::json;

        let client = TestClient::new();
        client
            .user
            .test(crate::endpoint::i::registry::set::Request {
                key: "test".into(),
                value: json!({ "test": { "test_inner": [1,2,3.14] }, "null": null }),
                scope: None,
            })
            .await;
        client
            .user
            .test(Request {
                key: "test".into(),
                scope: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_scope() {
        use crate::model::registry::RegistryScope;
        use serde_json::Value;

        let client = TestClient::new();

        let test_scope = RegistryScope::from_segments(vec!["test", "testtest"]).unwrap();
        client
            .user
            .test(crate::endpoint::i::registry::set::Request {
                key: "test".into(),
                value: Value::Null,
                scope: Some(test_scope.clone()),
            })
            .await;
        client
            .user
            .test(Request {
                key: "test".into(),
                scope: Some(test_scope),
            })
            .await;
    }
}
