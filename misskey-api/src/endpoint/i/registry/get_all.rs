use std::collections::HashMap;

use crate::model::registry::{RegistryKey, RegistryScope, RegistryValue};

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<RegistryScope>,
}

impl misskey_core::Request for Request {
    type Response = HashMap<RegistryKey, RegistryValue>;
    const ENDPOINT: &'static str = "i/registry/get-all";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_simple() {
        let client = TestClient::new();
        client.user.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_scope() {
        use crate::model::registry::RegistryScope;

        let client = TestClient::new();
        client
            .user
            .test(Request {
                scope: Some(RegistryScope::from_segments(vec!["client", "base"]).unwrap()),
            })
            .await;
    }
}
