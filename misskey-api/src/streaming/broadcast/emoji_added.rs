#[cfg(any(not(feature = "13-2-3"), feature = "13-7-0"))]
use crate::model::emoji::Emoji;
#[cfg(all(feature = "13-2-3", not(feature = "13-7-0")))]
use crate::model::emoji::EmojiSimple;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmojiAddedEvent {
    #[cfg(any(not(feature = "13-2-3"), feature = "13-7-0"))]
    #[cfg_attr(docsrs, doc(cfg(any(not(feature = "13-2-3"), feature = "13-7-0"))))]
    pub emoji: Emoji,
    #[cfg(all(feature = "13-2-3", not(feature = "13-7-0")))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "13-2-3", not(feature = "13-7-0")))))]
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
