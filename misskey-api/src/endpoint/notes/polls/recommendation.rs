use crate::model::note::Note;

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes/polls/recommendation";
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

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .test(Request {
                limit: Some(100),
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_offset() {
        let client = TestClient::new();
        client
            .test(Request {
                limit: None,
                offset: Some(5),
            })
            .await;
    }
}
