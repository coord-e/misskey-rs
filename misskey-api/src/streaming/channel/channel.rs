use crate::model::{channel::ChannelId, note::Note};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum ChannelEvent {
    Note(Note),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub channel_id: ChannelId,
}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = ChannelEvent;
    type Outgoing = ();

    const NAME: &'static str = "channel";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};
    use misskey_core::streaming::ChannelClient;

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let mut client = TestClient::new().await;
        let channel = client
            .test(crate::endpoint::channels::create::Request {
                name: "test".to_string(),
                description: None,
                banner_id: None,
            })
            .await;

        let mut stream = client
            .connect(Request {
                channel_id: channel.id,
            })
            .await
            .unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream() {
        let mut client = TestClient::new().await;
        let channel = client
            .test(crate::endpoint::channels::create::Request {
                name: "test".to_string(),
                description: None,
                banner_id: None,
            })
            .await;

        let mut stream = client
            .connect(Request {
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
