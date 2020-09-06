use crate::model::clip::{Clip, ClipId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub clip_id: ClipId,
    /// [ 1 .. 100 ] characters
    pub name: String,
}

impl misskey_core::Request for Request {
    type Response = Clip;
    const ENDPOINT: &'static str = "clips/update";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let clip = client
            .test(crate::endpoint::clips::create::Request {
                name: "clip".to_string(),
            })
            .await;

        client
            .test(Request {
                clip_id: clip.id,
                name: "updated".to_string(),
            })
            .await;
    }
}
