use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub notes_count: u64,
    pub original_notes_count: u64,
    pub users_count: u64,
    pub original_users_count: u64,
    #[cfg(feature = "12-62-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-62-0")))]
    pub reactions_count: u64,
    pub instances: u64,
    #[cfg(not(feature = "12-104-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-104-0"))))]
    pub drive_usage_local: u64,
    #[cfg(not(feature = "12-104-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-104-0"))))]
    pub drive_usage_remote: u64,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "stats";
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
