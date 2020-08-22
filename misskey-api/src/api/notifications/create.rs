use serde::Serialize;
use url::Url;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub body: String,
    pub header: Option<String>,
    pub icon: Option<Url>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "notifications/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client
            .test(Request {
                body: "hello".to_string(),
                header: None,
                icon: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let mut client = TestClient::new();
        let url = client.avatar_url().await;

        client
            .test(Request {
                body: "hi".to_string(),
                header: Some("header".to_string()),
                icon: Some(url),
            })
            .await;
    }
}