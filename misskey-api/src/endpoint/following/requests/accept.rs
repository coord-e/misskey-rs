use crate::model::{id::Id, user::User};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: Id<User>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "following/requests/accept";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let (new_user, new_client) = client.admin.create_user().await;

        new_client
            .test(
                crate::endpoint::i::update::Request::builder()
                    .is_locked(true)
                    .build(),
            )
            .await;
        client
            .user
            .test(crate::endpoint::following::create::Request {
                user_id: new_user.id,
            })
            .await;

        let user = client.user.me().await;
        new_client.test(Request { user_id: user.id }).await;
    }
}
