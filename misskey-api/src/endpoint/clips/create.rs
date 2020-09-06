use crate::model::clip::Clip;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// [ 1 .. 100 ] characters
    pub name: String,
}

impl misskey_core::Request for Request {
    type Response = Clip;
    const ENDPOINT: &'static str = "clips/create";
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
                name: "clip".to_string(),
            })
            .await;
    }
}
