use crate::model::{channel::Channel, id::Id, note::Note};
use crate::streaming::channel::NoOutgoing;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum ChannelEvent {
    Note(Note),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub channel_id: Id<Channel>,
}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = ChannelEvent;
    type Outgoing = NoOutgoing;

    const NAME: &'static str = "channel";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;
        let channel = client
            .test(crate::endpoint::channels::create::Request {
                name: "test".to_string(),
                description: None,
                banner_id: None,
            })
            .await;

        let mut stream = client
            .channel(Request {
                channel_id: channel.id,
            })
            .await
            .unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream() {
        let client = TestClient::new().await;
        let channel = client
            .test(crate::endpoint::channels::create::Request {
                name: "test".to_string(),
                description: None,
                banner_id: None,
            })
            .await;

        let mut stream = client
            .channel(Request {
                channel_id: channel.id.clone(),
            })
            .await
            .unwrap();

        future::join(
            client.test(crate::endpoint::notes::create::Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("some text".to_string()),
                cw: None,
                via_mobile: None,
                local_only: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                channel_id: Some(channel.id),
            }),
            async { stream.next().await.unwrap().unwrap() },
        )
        .await;
    }
}
