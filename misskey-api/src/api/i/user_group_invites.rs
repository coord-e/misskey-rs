use crate::model::user_group::{UserGroupInvitation, UserGroupInvitationId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<UserGroupInvitationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<UserGroupInvitationId>,
}

impl misskey_core::Request for Request {
    type Response = Vec<UserGroupInvitation>;
    const ENDPOINT: &'static str = "i/user-group-invites";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client
            .test(Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let mut client = TestClient::new();
        client
            .test(Request {
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let mut client = TestClient::new();
        let (new_user, mut new_user_client) = client.admin.create_user().await;
        let group = client
            .test(crate::api::users::groups::create::Request {
                name: "test".to_string(),
            })
            .await;
        client
            .test(crate::api::users::groups::invite::Request {
                group_id: group.id,
                user_id: new_user.id,
            })
            .await;
        let invitation = new_user_client
            .test(Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await
            .pop()
            .unwrap();

        new_user_client
            .test(Request {
                limit: None,
                since_id: Some(invitation.id.clone()),
                until_id: Some(invitation.id.clone()),
            })
            .await;
    }
}
