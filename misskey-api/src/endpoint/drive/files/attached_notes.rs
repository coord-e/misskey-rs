use crate::model::{drive::DriveFile, id::Id, note::Note};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub file_id: Id<DriveFile>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "drive/files/attached-notes";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let file = client.create_text_file("test.txt", "test").await;
        client
            .test(
                crate::endpoint::notes::create::Request::builder()
                    .file_ids(vec![file.id.clone()])
                    .build(),
            )
            .await;

        client.test(Request { file_id: file.id }).await;
    }
}
