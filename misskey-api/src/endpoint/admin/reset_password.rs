use crate::model::user::UserId;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: UserId,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub password: String,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "admin/reset-password";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let (user, _) = client.admin.create_user().await;

        client.admin.test(Request { user_id: user.id }).await;
    }
}
