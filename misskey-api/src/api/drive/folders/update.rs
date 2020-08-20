use crate::model::drive::{DriveFolder, DriveFolderId};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub folder_id: DriveFolderId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<DriveFolderId>>,
}

impl ApiRequest for Request {
    type Response = DriveFolder;
    const ENDPOINT: &'static str = "drive/folders/update";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let folder = client
            .test(crate::api::drive::folders::create::Request {
                name: None,
                parent_id: None,
            })
            .await;

        client
            .test(Request {
                folder_id: folder.id,
                name: None,
                parent_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_name() {
        let mut client = TestClient::new();
        let folder = client
            .test(crate::api::drive::folders::create::Request {
                name: None,
                parent_id: None,
            })
            .await;

        client
            .test(Request {
                folder_id: folder.id,
                name: Some("test".to_string()),
                parent_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_parent() {
        let mut client = TestClient::new();
        let folder1 = client
            .test(crate::api::drive::folders::create::Request {
                name: None,
                parent_id: None,
            })
            .await;
        let folder2 = client
            .test(crate::api::drive::folders::create::Request {
                name: None,
                parent_id: None,
            })
            .await;

        client
            .test(Request {
                folder_id: folder1.id.clone(),
                name: None,
                parent_id: Some(Some(folder2.id)),
            })
            .await;
        client
            .test(Request {
                folder_id: folder1.id,
                name: None,
                parent_id: Some(None),
            })
            .await;
    }
}
