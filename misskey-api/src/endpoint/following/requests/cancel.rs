use crate::model::{id::Id, user::User};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: Id<User>,
}

impl misskey_core::Request for Request {
    type Response = User;
    const ENDPOINT: &'static str = "following/requests/cancel";
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
                    .auto_accept_followed(false)
                    .build(),
            )
            .await;
        client
            .user
            .test(crate::endpoint::following::create::Request {
                user_id: new_user.id.clone(),
            })
            .await;

        client
            .user
            .test(Request {
                user_id: new_user.id,
            })
            .await;
    }
}
