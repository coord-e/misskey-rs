use crate::model::user_group::{UserGroupInvitation, UserGroupInvitationId};

use misskey_core::ApiRequest;
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

impl ApiRequest for Request {
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

    // TODO: request_paginate
}
