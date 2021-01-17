use crate::model::registry::{RegistryKey, RegistryScope};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub key: RegistryKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<RegistryScope>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "i/registry/remove";
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
            .test(crate::endpoint::i::registry::set::Request {
                key: "test_remove".into(),
                value: Value::Bool(true),
                scope: None,
            })
            .await;
        client
            .user
            .test(Request {
                key: "test_remove".into(),
                scope: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_scope() {
        use crate::model::registry::RegistryScope;
        use serde_json::Value;

        let client = TestClient::new();

        let test_scope = RegistryScope::from_segments(vec!["test_remove", "testtest"]).unwrap();
        client
            .user
            .test(crate::endpoint::i::registry::set::Request {
                key: "test_remove".into(),
                value: Value::Bool(true),
                scope: Some(test_scope.clone()),
            })
            .await;
        client
            .user
            .test(Request {
                key: "test_remove".into(),
                scope: Some(test_scope),
            })
            .await;
    }
}
