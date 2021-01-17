use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Debug, Clone)]
pub struct Request {}

#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    #[serde(with = "ts_milliseconds")]
    pub pong: DateTime<Utc>,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "ping";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_simple() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }
}
