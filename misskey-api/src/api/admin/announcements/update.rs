use crate::model::announcement::AnnouncementId;

use serde::Serialize;
use url::Url;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: AnnouncementId,
    pub title: String,
    pub text: String,
    pub image_url: Option<Url>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/announcements/update";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let announcement = client
            .admin
            .test(crate::api::admin::announcements::create::Request {
                title: "hello".to_string(),
                text: "ok".to_string(),
                image_url: None,
            })
            .await;

        client
            .admin
            .test(Request {
                id: announcement.id,
                title: "attention".to_string(),
                text: "hello".to_string(),
                image_url: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_image() {
        let mut client = TestClient::new();
        let announcement = client
            .admin
            .test(crate::api::admin::announcements::create::Request {
                title: "hello".to_string(),
                text: "ok".to_string(),
                image_url: None,
            })
            .await;
        let image_url = client.avatar_url().await;

        client
            .admin
            .test(Request {
                id: announcement.id,
                title: "hey".to_string(),
                text: "ok".to_string(),
                image_url: Some(image_url),
            })
            .await;
    }
}
