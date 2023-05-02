use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub code: String,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "invite";
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
