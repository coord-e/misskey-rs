#[cfg(not(feature = "13-2-3"))]
use crate::model::emoji::Emoji;
#[cfg(feature = "13-2-3")]
use crate::model::emoji::EmojiSimple;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmojiAddedEvent {
    #[cfg(not(feature = "13-2-3"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-2-3"))))]
    pub emoji: Emoji,
    #[cfg(feature = "13-2-3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-2-3")))]
    pub emoji: EmojiSimple,
}

impl misskey_core::streaming::BroadcastEvent for EmojiAddedEvent {
    const TYPE: &'static str = "emojiAdded";
}

#[cfg(test)]
mod tests {
    use super::EmojiAddedEvent;
    use crate::test::{http::TestClient as HttpTestClient, websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn broadcast() {
        let http_client = HttpTestClient::new();
        let client = TestClient::new().await;
        let url = http_client.avatar_url().await;

        let mut stream = client.broadcast::<EmojiAddedEvent>().await.unwrap();

        future::join(http_client.admin.add_emoji_from_url(url), async {
            stream.next().await.unwrap().unwrap()
        })
        .await;
    }
}
