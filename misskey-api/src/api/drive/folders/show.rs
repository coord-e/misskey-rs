use crate::model::drive::{DriveFolder, DriveFolderId};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub folder_id: DriveFolderId,
}

impl ApiRequest for Request {
    type Response = DriveFolder;
    const ENDPOINT: &'static str = "drive/folders/show";
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
            })
            .await;
    }
}
