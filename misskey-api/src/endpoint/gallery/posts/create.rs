use crate::model::{drive::DriveFile, gallery::GalleryPost, id::Id};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// [ 1 .. ] characters
    #[builder(default, setter(into))]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub description: Option<String>,
    /// [ 1 .. 32 ] ids
    #[builder(default, setter(into))]
    pub file_ids: Vec<Id<DriveFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub is_sensitive: Option<bool>,
}

impl misskey_core::Request for Request {
    type Response = GalleryPost;
    const ENDPOINT: &'static str = "gallery/posts/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let url = client.avatar_url().await;
        let file = client.upload_from_url(url).await;

        client
            .test(Request {
                title: "gallery post".to_string(),
                description: None,
                file_ids: vec![file.id],
                is_sensitive: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        let url = client.avatar_url().await;
        let file = client.upload_from_url(url).await;

        client
            .test(Request {
                title: "gallery post".to_string(),
                description: Some("description".to_string()),
                file_ids: vec![file.id],
                is_sensitive: Some(true),
            })
            .await;
    }
}
