use crate::model::channel::Channel;

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl misskey_core::Request for Request {
    type Response = Vec<Channel>;
    const ENDPOINT: &'static str = "channels/my-favorites";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test")
                    .build(),
            )
            .await;
        client
            .test(crate::endpoint::channels::favorite::Request {
                channel_id: channel.id,
            })
            .await;

        client.test(Request::default()).await;
    }
}
