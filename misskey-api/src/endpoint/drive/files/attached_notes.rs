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
            .test(crate::endpoint::notes::create::Request {
                visibility: None,
                visible_user_ids: None,
                text: None,
                cw: None,
                via_mobile: None,
                local_only: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: Some(vec![file.id.clone()]),
                reply_id: None,
                renote_id: None,
                poll: None,
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await;

        client.test(Request { file_id: file.id }).await;
    }
}
