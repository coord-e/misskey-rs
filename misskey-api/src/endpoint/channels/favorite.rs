use crate::model::{channel::Channel, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub channel_id: Id<Channel>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "channels/favorite";
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
            .test(Request {
                channel_id: channel.id,
            })
            .await;
    }
}
