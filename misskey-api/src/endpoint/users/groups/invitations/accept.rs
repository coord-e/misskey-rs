use crate::model::{id::Id, user_group::UserGroupInvitation};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[cfg_attr(not(feature = "12-8-0"), serde(rename = "inviteId"))]
    pub invitation_id: Id<UserGroupInvitation>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "users/groups/invitations/accept";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let (new_user, new_user_client) = client.admin.create_user().await;
        let group = client
            .test(crate::endpoint::users::groups::create::Request {
                name: "test".to_string(),
            })
            .await;
        client
            .test(crate::endpoint::users::groups::invite::Request {
                group_id: group.id.clone(),
                user_id: new_user.id.clone(),
            })
            .await;
        let invitation = new_user_client
            .test(crate::endpoint::i::user_group_invites::Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await
            .pop()
            .unwrap();

        new_user_client
            .test(Request {
                invitation_id: invitation.id,
            })
            .await;
    }
}
