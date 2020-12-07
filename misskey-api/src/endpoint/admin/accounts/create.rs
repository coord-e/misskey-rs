use crate::model::user::User;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(flatten)]
    pub user: User,
    pub token: String,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "admin/accounts/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let ulid = ulid_crate::Ulid::new().to_string();
        client
            .admin
            .test(Request {
                username: ulid[..20].to_owned(),
                password: "password".to_string(),
            })
            .await;
    }
}
