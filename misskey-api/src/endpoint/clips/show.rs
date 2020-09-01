use crate::model::clip::{Clip, ClipId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub clip_id: ClipId,
}

impl misskey_core::Request for Request {
    type Response = Clip;
    const ENDPOINT: &'static str = "clips/show";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let clip = client
            .test(crate::endpoint::clips::create::Request {
                name: "clip test".to_string(),
            })
            .await;

        client.test(Request { clip_id: clip.id }).await;
    }
}
