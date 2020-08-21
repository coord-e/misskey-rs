use crate::model::{drive::DriveFileId, note::Note};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub file_id: DriveFileId,
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
        let mut client = TestClient::new();
        let file = client.create_text_file("test.txt", "test").await;
        client
            .test(crate::api::notes::create::Request {
                visibility: None,
                visible_user_ids: Vec::new(),
                text: None,
                cw: None,
                via_mobile: false,
                local_only: false,
                no_extract_mentions: false,
                no_extract_hashtags: false,
                no_extract_emojis: false,
                file_ids: vec![file.id.clone()],
                reply_id: None,
                renote_id: None,
                poll: None,
            })
            .await;

        client.test(Request { file_id: file.id }).await;
    }
}
