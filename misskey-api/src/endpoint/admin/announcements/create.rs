use crate::model::announcement::Announcement;

use serde::Serialize;
use typed_builder::TypedBuilder;
use url::Url;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[builder(setter(into))]
    pub title: String,
    #[builder(setter(into))]
    pub text: String,
    #[builder(default, setter(strip_option))]
    pub image_url: Option<Url>,
}

impl misskey_core::Request for Request {
    type Response = Announcement;
    const ENDPOINT: &'static str = "admin/announcements/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client
            .admin
            .test(Request {
                title: "attention".to_string(),
                text: "hello".to_string(),
                image_url: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_image() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;

        client
            .admin
            .test(Request {
                title: "hey".to_string(),
                text: "ok".to_string(),
                image_url: Some(image_url),
            })
            .await;
    }
}
