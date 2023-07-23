use crate::model::{id::Id, role::Role, user::User};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub role_id: Id<Role>,
    pub user_id: Id<User>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/roles/unassign";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let (user, _) = client.admin.create_user().await;
        let role = client
            .admin
            .test(crate::endpoint::admin::roles::create::Request::default())
            .await;
        client
            .admin
            .test(
                crate::endpoint::admin::roles::assign::Request::builder()
                    .role_id(role.id)
                    .user_id(user.id)
                    .build(),
            )
            .await;

        client
            .admin
            .test(Request {
                role_id: role.id,
                user_id: user.id,
            })
            .await;
    }
}
