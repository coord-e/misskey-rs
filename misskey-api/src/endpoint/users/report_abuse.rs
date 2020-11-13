use crate::model::{id::Id, user::User};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: Id<User>,
    /// 1 .. 3000 characters
    pub comment: String,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "users/report-abuse";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let (new_user, _) = client.admin.create_user().await;

        client
            .test(Request {
                user_id: new_user.id,
                comment: "damesou".to_string(),
            })
            .await;
    }
}
