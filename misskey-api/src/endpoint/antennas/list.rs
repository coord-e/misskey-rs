use crate::model::antenna::Antenna;

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl misskey_core::Request for Request {
    type Response = Vec<Antenna>;
    const ENDPOINT: &'static str = "antennas/list";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client
            .test(
                crate::endpoint::antennas::create::Request::builder()
                    .name("test")
                    .build(),
            )
            .await;

        client.test(Request::default()).await;
    }
}
