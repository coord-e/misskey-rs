use crate::model::{flash::Flash, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub flash_id: Id<Flash>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "flash/like";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let flash = client
            .admin
            .test(crate::endpoint::flash::create::Request::default())
            .await;

        client.user.test(Request { flash_id: flash.id }).await;
    }
}
