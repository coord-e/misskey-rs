use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::{id::Id, user::User};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    user_id: Id<User>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ip {
    pub ip: String,
    pub created_at: DateTime<Utc>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Ip>;
    const ENDPOINT: &'static str = "admin/get-user-ips";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let user_id = client.user.me().await.id;
        client.admin.test(Request { user_id }).await;
    }
}
