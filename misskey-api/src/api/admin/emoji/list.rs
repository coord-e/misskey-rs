use crate::model::emoji::{Emoji, EmojiId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<EmojiId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<EmojiId>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Emoji>;
    const ENDPOINT: &'static str = "admin/emoji/list";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client
            .admin
            .test(Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let mut client = TestClient::new();
        client
            .admin
            .test(Request {
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let mut client = TestClient::new();
        let image_url = client.avatar_url().await;
        let emoji_id = client.admin.add_emoji_from_url(image_url).await;

        client
            .admin
            .test(Request {
                limit: None,
                since_id: Some(emoji_id.clone()),
                until_id: Some(emoji_id.clone()),
            })
            .await;
    }
}
