use crate::model::{
    drive::{DriveFile, DriveFolder},
    id::Id,
};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub name: String,
    pub folder_id: Option<Id<DriveFolder>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<DriveFile>;
    const ENDPOINT: &'static str = "drive/files/find";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.create_text_file("test.txt", "test").await;

        client
            .test(Request {
                name: "test.txt".to_string(),
                folder_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_folder() {
        let client = TestClient::new();
        let folder = client
            .test(crate::endpoint::drive::folders::create::Request {
                name: None,
                parent_id: None,
            })
            .await;

        client
            .test(Request {
                name: "test.txt".to_string(),
                folder_id: Some(folder.id),
            })
            .await;
    }
}
