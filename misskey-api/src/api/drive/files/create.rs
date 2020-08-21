use crate::model::drive::{DriveFile, DriveFolderId};

use serde::ser::Serializer;
use serde::Serialize;

fn bool_string_option<S: Serializer>(
    input: &Option<bool>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match input {
        Some(true) => serializer.serialize_some("true"),
        Some(false) => serializer.serialize_some("false"),
        None => serializer.serialize_none(),
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<DriveFolderId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "bool_string_option"
    )]
    pub is_sensitive: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "bool_string_option"
    )]
    pub force: Option<bool>,
}

impl misskey_core::Request for Request {
    type Response = DriveFile;
    const ENDPOINT: &'static str = "drive/files/create";
}

impl misskey_core::RequestWithFile for Request {}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client
            .test_with_file(
                Request {
                    folder_id: None,
                    name: None,
                    is_sensitive: None,
                    force: None,
                },
                mime::TEXT_PLAIN,
                "test.txt",
                "hello",
            )
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let mut client = TestClient::new();
        let folder = client
            .test(crate::api::drive::folders::create::Request {
                name: None,
                parent_id: None,
            })
            .await;
        client
            .test_with_file(
                Request {
                    folder_id: Some(folder.id),
                    name: Some("hello.txt".to_string()),
                    is_sensitive: Some(true),
                    force: Some(true),
                },
                mime::TEXT_PLAIN,
                "test.txt",
                "hello",
            )
            .await;
    }
}
