use crate::model::{channel::Channel, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub channel_id: Id<Channel>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "channels/unfollow";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let channel = client
            .test(crate::endpoint::channels::create::Request {
                name: "test channel".to_string(),
                description: None,
                banner_id: None,
            })
            .await;
        client
            .test(crate::endpoint::channels::follow::Request {
                channel_id: channel.id.clone(),
            })
            .await;

        client
            .test(Request {
                channel_id: channel.id,
            })
            .await;
    }
}
