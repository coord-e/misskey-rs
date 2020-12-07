use crate::model::{
    drive::{DriveFile, DriveFolder},
    id::Id,
};

use serde::Serialize;
use typed_builder::TypedBuilder;
use url::Url;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub folder_id: Option<Id<DriveFolder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub force: Option<bool>,
}

impl misskey_core::Request for Request {
    type Response = DriveFile;
    const ENDPOINT: &'static str = "drive/files/upload-from-url";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client
            .test(Request {
                url: url::Url::parse("http://example.com/index.html").unwrap(),
                folder_id: None,
                is_sensitive: None,
                force: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_image() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;

        client
            .test(Request {
                url: image_url,
                folder_id: None,
                is_sensitive: None,
                force: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        let folder = client
            .test(crate::endpoint::drive::folders::create::Request {
                name: None,
                parent_id: None,
            })
            .await;

        client
            .test(Request {
                url: url::Url::parse("http://example.com/index.html").unwrap(),
                folder_id: Some(folder.id),
                is_sensitive: Some(true),
                force: Some(true),
            })
            .await;
    }
}
