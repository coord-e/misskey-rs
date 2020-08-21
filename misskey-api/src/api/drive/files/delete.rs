use crate::model::drive::DriveFileId;

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub file_id: DriveFileId,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "drive/files/delete";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let file = client.create_text_file("test.txt", "test").await;
        client.test(Request { file_id: file.id }).await;
    }
}
