use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    count: u64,
    size: u64,
}

impl misskey_core::Request for Request {
    type Response = HashMap<String, Stat>;
    const ENDPOINT: &'static str = "admin/get-table-stats";
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
}
