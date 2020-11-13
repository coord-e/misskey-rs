use crate::model::{id::Id, user::User, user_group::UserGroup};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub group_id: Id<UserGroup>,
    pub user_id: Id<User>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "users/groups/invite";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let (user, _) = client.admin.create_user().await;
        let group = client
            .test(crate::endpoint::users::groups::create::Request {
                name: "test".to_string(),
            })
            .await;

        client
            .test(Request {
                group_id: group.id,
                user_id: user.id,
            })
            .await;
    }
}
