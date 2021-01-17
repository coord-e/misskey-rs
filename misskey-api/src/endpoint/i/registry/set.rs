use crate::model::registry::{RegistryKey, RegistryScope, RegistryValue};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub key: RegistryKey,
    pub value: RegistryValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<RegistryScope>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "i/registry/set";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_simple() {
        use serde_json::Value;

        let client = TestClient::new();
        client
            .user
            .test(Request {
                key: "test_set".into(),
                value: Value::String("hello".to_string()),
                scope: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_object() {
        use serde_json::json;

        let client = TestClient::new();
        client
            .user
            .test(Request {
                key: "test_set_object".into(),
                value: json!({ "test": ["item1", "item2"] }),
                scope: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_array() {
        use serde_json::json;

        let client = TestClient::new();
        client
            .user
            .test(Request {
                key: "test_set_array".into(),
                value: json!([1, 2]),
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
            .test(Request {
                key: "test_set".into(),
                value: Value::Number(42_u64.into()),
                scope: Some(test_scope),
            })
            .await;
    }
}
