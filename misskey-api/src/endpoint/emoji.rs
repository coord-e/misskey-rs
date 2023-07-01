use crate::model::emoji::Emoji;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub name: String,
}

impl misskey_core::Request for Request {
    type Response = Emoji;
    const ENDPOINT: &'static str = "emoji";
}

#[cfg(test)]
mod tests {
    use ulid_crate::Ulid;

    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let emoji_id = client.admin.add_emoji_from_url(image_url).await;
        let name = Ulid::new().to_string();
        client
            .admin
            .test(
                crate::endpoint::admin::emoji::update::Request::builder()
                    .id(emoji_id)
                    .name(name.clone())
                    .build(),
            )
            .await;

        client.test(Request { name }).await;
    }
}
