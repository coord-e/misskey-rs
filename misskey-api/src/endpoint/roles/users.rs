use crate::model::{
    id::Id,
    role::{Role, RoleAssignment},
};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub role_id: Id<Role>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<RoleAssignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<RoleAssignment>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<RoleAssignment>;
    const ENDPOINT: &'static str = "roles/users";
}

impl_pagination!(Request, RoleAssignment);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_simple() {
        let client = TestClient::new();
        let role = client
            .admin
            .test(
                crate::endpoint::admin::roles::create::Request::builder()
                    .is_public(true)
                    .build(),
            )
            .await;

        client
            .test(Request {
                role_id: role.id,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        let role = client
            .admin
            .test(
                crate::endpoint::admin::roles::create::Request::builder()
                    .is_public(true)
                    .build(),
            )
            .await;

        client
            .test(Request {
                role_id: role.id,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let role = client
            .admin
            .test(
                crate::endpoint::admin::roles::create::Request::builder()
                    .is_public(true)
                    .build(),
            )
            .await;
        let (user, _) = client.admin.create_user().await;
        client
            .admin
            .test(
                crate::endpoint::admin::roles::assign::Request::builder()
                    .role_id(role.id)
                    .user_id(user.id)
                    .build(),
            )
            .await;
        let assignments = client
            .test(Request {
                role_id: role.id,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;

        client
            .test(Request {
                role_id: role.id,
                limit: None,
                since_id: Some(assignments[0].id),
                until_id: Some(assignments[0].id),
            })
            .await;
    }
}
