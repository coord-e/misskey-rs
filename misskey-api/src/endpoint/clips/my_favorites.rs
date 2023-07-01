use crate::model::clip::Clip;

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl misskey_core::Request for Request {
    type Response = Vec<Clip>;
    const ENDPOINT: &'static str = "clips/my-favorites";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let clip = client
            .test(
                crate::endpoint::clips::create::Request::builder()
                    .name("test")
                    .build(),
            )
            .await;
        client
            .test(crate::endpoint::clips::favorite::Request { clip_id: clip.id })
            .await;

        client.test(Request::default()).await;
    }
}
