use crate::model::{user::UserId, user_group::UserGroupId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub group_id: UserGroupId,
    pub user_id: UserId,
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
        let mut client = TestClient::new();
        let (user, _) = client.admin.create_user().await;
        let group = client
            .test(crate::api::users::groups::create::Request {
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
