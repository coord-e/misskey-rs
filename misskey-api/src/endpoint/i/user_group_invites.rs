use crate::model::{id::Id, user_group::UserGroupInvitation};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<UserGroupInvitation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<UserGroupInvitation>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<UserGroupInvitation>;
    const ENDPOINT: &'static str = "i/user-group-invites";
}

impl_pagination!(Request, UserGroupInvitation);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
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
        let client = TestClient::new();
        let (new_user, new_user_client) = client.admin.create_user().await;
        let group = client
            .test(crate::endpoint::users::groups::create::Request {
                name: "test".to_string(),
            })
            .await;
        client
            .test(crate::endpoint::users::groups::invite::Request {
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
