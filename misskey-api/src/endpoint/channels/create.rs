use crate::model::{channel::Channel, drive::DriveFileId};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// [ 1 .. 128 ] characters
    #[builder(setter(into))]
    pub name: String,
    /// [ 1 .. 2048 ] characters
    #[builder(default, setter(strip_option, into))]
    pub description: Option<String>,
    #[builder(default, setter(strip_option))]
    pub banner_id: Option<DriveFileId>,
}

impl misskey_core::Request for Request {
    type Response = Channel;
    const ENDPOINT: &'static str = "channels/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request {
            // random 128 chars
            name: "yCdKKHkRAmqhE49j3TBCVnnsQiZ2KkCK83z6NNKoGaiqQNOALsAOIz6XVJAcaV9lNbQYuuhSe7nAM8qdvUtokhWYWDO999z07N4ajtikDmzANpgwRh7rxMgb8PLsgcbm".to_string(),
            description: None,
            banner_id: None
        }).await;
    }

    #[tokio::test]
    async fn request_with_description() {
        let client = TestClient::new();
        client
            .test(Request {
                name: "test channel".to_string(),
                description: Some("description".to_string()),
                banner_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_banner() {
        let client = TestClient::new();
        let url = client.avatar_url().await;
        let file = client
            .test(crate::endpoint::drive::files::upload_from_url::Request {
                url,
                folder_id: None,
                is_sensitive: None,
                force: None,
            })
            .await;

        client
            .test(Request {
                name: "test channel".to_string(),
                description: None,
                banner_id: Some(file.id),
            })
            .await;
    }
}
