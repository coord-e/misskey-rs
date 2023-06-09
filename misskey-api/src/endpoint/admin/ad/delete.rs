use crate::model::{ad::Ad, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: Id<Ad>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/ad/delete";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let url = client.avatar_url().await;

        client
            .admin
            .test(
                crate::endpoint::admin::ad::create::Request::builder()
                    .url(url.clone())
                    .image_url(url.clone())
                    .expires_at(chrono::Utc::now() + chrono::Duration::hours(1))
                    .build(),
            )
            .await;

        let ads = client
            .admin
            .test(crate::endpoint::admin::ad::list::Request::default())
            .await;

        client.admin.test(Request { id: ads[0].id }).await;
    }
}
