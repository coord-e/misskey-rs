use crate::model::{drive::DriveFolder, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub folder_id: Id<DriveFolder>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "drive/folders/delete";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let folder = client
            .test(crate::endpoint::drive::folders::create::Request {
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
