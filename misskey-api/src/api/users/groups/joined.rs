use crate::model::user_group::UserGroup;

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl misskey_core::Request for Request {
    type Response = Vec<UserGroup>;
    const ENDPOINT: &'static str = "users/groups/joined";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let (new_user, mut new_user_client) = client.admin.create_user().await;
        let group = client
            .test(crate::api::users::groups::create::Request {
                name: "test".to_string(),
            })
            .await;
        client
            .test(crate::api::users::groups::invite::Request {
                group_id: group.id.clone(),
                user_id: new_user.id.clone(),
            })
            .await;
        let invitation = new_user_client
            .test(crate::api::i::user_group_invites::Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await
            .pop()
            .unwrap();
        new_user_client
            .test(crate::api::users::groups::invitations::accept::Request {
                invitation_id: invitation.id,
            })
            .await;

        new_user_client.test(Request::default()).await;
    }
}
