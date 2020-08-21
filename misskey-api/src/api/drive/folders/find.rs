use crate::model::drive::{DriveFolder, DriveFolderId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub name: String,
    pub parent_id: Option<DriveFolderId>,
}

impl misskey_core::Request for Request {
    type Response = Vec<DriveFolder>;
    const ENDPOINT: &'static str = "drive/folders/find";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client
            .test(crate::api::drive::folders::create::Request {
                name: Some("test1".to_string()),
                parent_id: None,
            })
            .await;

        client
            .test(Request {
                name: "test".to_string(),
                parent_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_parent() {
        let mut client = TestClient::new();
        let folder = client
            .test(crate::api::drive::folders::create::Request {
                name: None,
                parent_id: None,
            })
            .await;

        client
            .test(Request {
                name: "query".to_string(),
                parent_id: Some(folder.id),
            })
            .await;
    }
}
