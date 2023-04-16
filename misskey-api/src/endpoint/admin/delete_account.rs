use crate::model::{id::Id, user::User};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: Id<User>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/delete-account";
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
