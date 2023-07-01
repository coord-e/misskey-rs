use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub name: String,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "i/claim-achievement";
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
                name: "brainDiver".to_string(),
            })
            .await;
    }
}
