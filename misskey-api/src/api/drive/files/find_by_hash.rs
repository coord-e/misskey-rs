use crate::model::drive::DriveFile;

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub md5: String,
}

impl ApiRequest for Request {
    type Response = Vec<DriveFile>;
    const ENDPOINT: &'static str = "drive/files/find-by-hash";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let file = client.create_text_file("test.txt", "test").await;
        client.test(Request { md5: file.md5 }).await;
    }
}
