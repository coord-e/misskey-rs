use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Debug, Clone)]
pub struct Request {}

#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    pub count: u64,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "get-online-users-count";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }
}
