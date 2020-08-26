use serde::{Deserialize, Serialize};

pub mod files;
pub mod folders;
pub mod stream;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub capacity: u64,
    pub usage: u64,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "drive";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client.test(Request::default()).await;
    }
}
