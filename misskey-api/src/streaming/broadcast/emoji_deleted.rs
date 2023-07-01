use crate::model::emoji::EmojiSimple;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmojiDeletedEvent {
    pub emojis: Vec<EmojiSimple>,
}

impl misskey_core::streaming::BroadcastEvent for EmojiDeletedEvent {
    const TYPE: &'static str = "emojiDeleted";
}

#[cfg(test)]
mod tests {
    use super::EmojiDeletedEvent;
    use crate::test::{http::TestClient as HttpTestClient, websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn broadcast() {
        let http_client = HttpTestClient::new();
        let client = TestClient::new().await;
        let image_url = http_client.avatar_url().await;
        let emoji_id = http_client.admin.add_emoji_from_url(image_url).await;

        let mut stream = client.broadcast::<EmojiDeletedEvent>().await.unwrap();

        future::join(
            http_client
                .admin
                .test(crate::endpoint::admin::emoji::delete::Request { id: emoji_id }),
            async { stream.next().await.unwrap().unwrap() },
        )
        .await;
    }
}
