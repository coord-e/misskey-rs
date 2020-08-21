use crate::model::drive::{DriveFile, DriveFolderId};

use serde::Serialize;
use url::Url;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<DriveFolderId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl misskey_core::Request for Request {
    type Response = DriveFile;
    const ENDPOINT: &'static str = "drive/files/upload-from-url";
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
                url: url::Url::parse("http://example.com/index.html").unwrap(),
                folder_id: None,
                is_sensitive: None,
                force: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let mut client = TestClient::new();
        let folder = client
            .test(crate::api::drive::folders::create::Request {
                name: None,
                parent_id: None,
            })
            .await;

        client
            .test(Request {
                url: url::Url::parse("http://example.com/index.html").unwrap(),
                folder_id: Some(folder.id),
                is_sensitive: Some(true),
                force: Some(true),
            })
            .await;
    }
}
