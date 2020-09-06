use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "notifications/mark-all-as-read";
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
