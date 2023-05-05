use crate::model::{clip::Clip, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub clip_id: Id<Clip>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "clips/unfavorite";
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

        client.test(Request { clip_id: clip.id }).await;
    }
}
