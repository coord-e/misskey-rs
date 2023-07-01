use crate::model::{antenna::Antenna, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub antenna_id: Id<Antenna>,
}

impl misskey_core::Request for Request {
    type Response = Antenna;
    const ENDPOINT: &'static str = "antennas/show";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let antenna = client
            .test(
                crate::endpoint::antennas::create::Request::builder()
                    .name("test")
                    .keywords("hello awesome")
                    .build(),
            )
            .await;

        client
            .test(Request {
                antenna_id: antenna.id,
            })
            .await;
    }
}
