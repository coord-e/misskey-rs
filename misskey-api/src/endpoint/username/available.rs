use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub username: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub available: bool,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "username/available";
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
                username: "test".to_string(),
            })
            .await;
    }
}
