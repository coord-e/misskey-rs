use crate::model::user::User;

use misskey_core::ApiRequest;
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

impl ApiRequest for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "admin/accounts/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let uuid = uuid::Uuid::new_v4().to_simple().to_string();
        client
            .admin
            .test(Request {
                username: uuid[..20].to_owned(),
                password: "password".to_string(),
            })
            .await;
    }
}