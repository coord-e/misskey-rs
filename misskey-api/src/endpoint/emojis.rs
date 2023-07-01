use serde::{Deserialize, Serialize};

use crate::model::emoji::EmojiSimple;

#[derive(Serialize, Default, Debug, Clone)]
pub struct Request {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub emojis: Vec<EmojiSimple>,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "emojis";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }
}
