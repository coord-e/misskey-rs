use crate::model::drive::{DriveFile, DriveFileId};

use serde::Serialize;
use url::Url;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<DriveFileId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
}

impl misskey_core::Request for Request {
    type Response = DriveFile;
    const ENDPOINT: &'static str = "drive/files/show";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request_with_id() {
        let mut client = TestClient::new();
        let file = client.create_text_file("test.txt", "test").await;
        client
            .test(Request {
                file_id: Some(file.id),
                url: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_url() {
        let mut client = TestClient::new();
        let file = client.create_text_file("test.txt", "test").await;
        client
            .test(Request {
                file_id: None,
                url: Some(file.url.unwrap()),
            })
            .await;
    }
}
