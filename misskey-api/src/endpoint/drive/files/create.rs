use crate::model::{
    drive::{DriveFile, DriveFolder},
    id::Id,
};

use serde::ser::Serializer;
use serde::Serialize;
use typed_builder::TypedBuilder;

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

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub folder_id: Option<Id<DriveFolder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "bool_string_option"
    )]
    #[builder(default, setter(strip_option))]
    pub is_sensitive: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "bool_string_option"
    )]
    #[builder(default, setter(strip_option))]
    pub force: Option<bool>,
}

impl misskey_core::Request for Request {
    type Response = DriveFile;
    const ENDPOINT: &'static str = "drive/files/create";
}

impl misskey_core::UploadFileRequest for Request {}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client
            .test_with_file(Request::default(), mime::TEXT_PLAIN, "test.txt", "hello")
            .await;
    }

    #[tokio::test]
    async fn request_image() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let image_data = reqwest::get(image_url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        client
            .test_with_file(
                Request {
                    folder_id: None,
                    name: None,
                    is_sensitive: None,
                    force: None,
                },
                mime::IMAGE_PNG,
                "icon.png",
                image_data,
            )
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
