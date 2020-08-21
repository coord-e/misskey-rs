use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub md5: String,
}

impl ApiRequest for Request {
    type Response = bool;
    const ENDPOINT: &'static str = "drive/files/check-existence";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request_exists() {
        let mut client = TestClient::new();
        let file = client.create_text_file("test.txt", "test").await;
        client.test(Request { md5: file.md5 }).await;
    }

    #[tokio::test]
    async fn request_not_exists() {
        let mut client = TestClient::new();
        // dummy
        client
            .test(Request {
                md5: "45641d522d66392c28117d46c099d08b".to_string(),
            })
            .await;
    }
}
