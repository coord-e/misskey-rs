use crate::model::{id::Id, user::User};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: Id<User>,
    pub override_mb: Option<u64>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/drive-capacity-override";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let user_id = client.user.me().await.id;

        client
            .admin
            .test(Request {
                user_id,
                override_mb: Some(1000),
            })
            .await;
    }
}
