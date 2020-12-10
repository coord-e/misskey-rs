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
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub comment: Option<String>,
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub marker: Option<String>,
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
                #[cfg(feature = "12-48-0")]
                comment: None,
                #[cfg(feature = "12-48-0")]
                marker: None,
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
                #[cfg(feature = "12-48-0")]
                comment: None,
                #[cfg(feature = "12-48-0")]
                marker: None,
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
                #[cfg(feature = "12-48-0")]
                comment: Some("comment".to_string()),
                #[cfg(feature = "12-48-0")]
                marker: Some("marker".to_string()),
            })
            .await;
    }
}
