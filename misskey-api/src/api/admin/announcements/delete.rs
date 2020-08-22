use crate::model::announcement::AnnouncementId;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: AnnouncementId,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/announcements/delete";
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
                title: "attention".to_string(),
                text: "hello".to_string(),
                image_url: None,
            })
            .await;

        client
            .admin
            .test(Request {
                id: announcement.id,
            })
            .await;
    }
}
