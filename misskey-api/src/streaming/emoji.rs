use crate::model::emoji::Emoji;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmojiAddedEvent {
    pub emoji: Emoji,
}

impl misskey_core::streaming::BroadcastEvent for EmojiAddedEvent {
    const TYPE: &'static str = "emojiAdded";
}

#[cfg(test)]
mod tests {
    use super::EmojiAddedEvent;
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn broadcast() {
        let client = TestClient::new().await;
        let url = client.avatar_url().await;

        let mut stream = client.broadcast::<EmojiAddedEvent>().await.unwrap();

        future::join(client.admin.add_emoji_from_url(url), async {
            stream.next().await.unwrap().unwrap()
        })
        .await;
    }
}
