use crate::model::drive::{DriveFolder, DriveFolderId};

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub parent_id: Option<DriveFolderId>,
}

impl misskey_core::Request for Request {
    type Response = DriveFolder;
    const ENDPOINT: &'static str = "drive/folders/create";
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
    async fn request_with_name() {
        let client = TestClient::new();
        client
            .test(Request {
                name: Some("folder".to_string()),
                parent_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_parent() {
        let client = TestClient::new();
        let folder = client
            .test(Request {
                name: None,
                parent_id: None,
            })
            .await;
        client
            .test(Request {
                name: None,
                parent_id: Some(folder.id),
            })
            .await;
    }
}
