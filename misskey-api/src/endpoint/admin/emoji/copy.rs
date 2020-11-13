use crate::model::{emoji::Emoji, id::Id};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub emoji_id: Id<Emoji>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: Id<Emoji>,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "admin/emoji/copy";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let emoji_id = client.admin.add_emoji_from_url(image_url).await;

        client.admin.test(Request { emoji_id }).await;
    }
}
