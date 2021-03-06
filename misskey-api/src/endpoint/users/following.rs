use crate::model::{following::Following, id::Id, user::User};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowingWithFollowee {
    #[serde(flatten)]
    pub following: Following,
    pub followee: User,
}

impl crate::PaginationItem for FollowingWithFollowee {
    type Id = Id<User>;
    fn item_id(&self) -> Id<User> {
        self.following.followee_id
    }
}

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct RequestWithUserId {
    pub user_id: Id<User>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<User>>,
}

impl misskey_core::Request for RequestWithUserId {
    type Response = Vec<FollowingWithFollowee>;
    const ENDPOINT: &'static str = "users/following";
}

impl crate::PaginationRequest for RequestWithUserId {
    type Item = FollowingWithFollowee;

    fn set_since_id(&mut self, id: Id<User>) {
        self.since_id.replace(id);
    }

    fn set_until_id(&mut self, id: Id<User>) {
        self.until_id.replace(id);
    }

    fn set_limit(&mut self, limit: u8) {
        self.limit.replace(limit);
    }
}

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct RequestWithUsername {
    #[builder(setter(into))]
    pub username: String,
    #[builder(default, setter(strip_option, into))]
    pub host: Option<String>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<User>>,
}

impl misskey_core::Request for RequestWithUsername {
    type Response = Vec<FollowingWithFollowee>;
    const ENDPOINT: &'static str = "users/following";
}

impl crate::PaginationRequest for RequestWithUsername {
    type Item = FollowingWithFollowee;

    fn set_since_id(&mut self, id: Id<User>) {
        self.since_id.replace(id);
    }

    fn set_until_id(&mut self, id: Id<User>) {
        self.until_id.replace(id);
    }

    fn set_limit(&mut self, limit: u8) {
        self.limit.replace(limit);
    }
}

#[cfg(test)]
mod tests {
    use super::{RequestWithUserId, RequestWithUsername};
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_with_id() {
        let client = TestClient::new();
        let user = client.me().await;

        client
            .test(RequestWithUserId {
                user_id: user.id,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_username() {
        let client = TestClient::new();
        let user = client.me().await;

        client
            .test(RequestWithUsername {
                username: user.username,
                host: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    // TODO: request_with_username_and_host

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        let user = client.me().await;
        client
            .test(RequestWithUserId {
                user_id: user.id,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let user = client.user.me().await;
        let (new_user, _) = client.admin.create_user().await;
        client
            .user
            .test(crate::endpoint::following::create::Request {
                user_id: new_user.id.clone(),
            })
            .await;

        client
            .user
            .test(RequestWithUserId {
                user_id: user.id,
                limit: None,
                since_id: Some(new_user.id.clone()),
                until_id: Some(new_user.id.clone()),
            })
            .await;
    }
}
