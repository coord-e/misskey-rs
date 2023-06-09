use crate::model::{
    drive::{DriveFile, DriveFolder},
    id::Id,
};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub file_id: Id<DriveFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub folder_id: Option<Option<Id<DriveFolder>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_sensitive: Option<bool>,
    #[cfg(feature = "12-82-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-82-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub comment: Option<String>,
}

impl misskey_core::Request for Request {
    type Response = DriveFile;
    const ENDPOINT: &'static str = "drive/files/update";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let file = client.create_text_file("test.txt", "test").await;
        client
            .test(Request {
                file_id: file.id,
                folder_id: None,
                name: None,
                is_sensitive: None,
                #[cfg(feature = "12-82-0")]
                comment: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        let file = client.create_text_file("test.txt", "test").await;
        let folder = client
            .test(crate::endpoint::drive::folders::create::Request {
                name: None,
                parent_id: None,
            })
            .await;

        client
            .test(Request {
                file_id: file.id,
                folder_id: Some(Some(folder.id)),
                name: Some("test2.txt".to_string()),
                is_sensitive: Some(true),
                #[cfg(feature = "12-82-0")]
                comment: Some("comment".to_string()),
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_null_options() {
        let client = TestClient::new();
        let file = client.create_text_file("test.txt", "test").await;
        client
            .test(Request {
                file_id: file.id,
                folder_id: Some(None),
                name: None,
                is_sensitive: None,
                #[cfg(feature = "12-82-0")]
                comment: None,
            })
            .await;
    }
}
