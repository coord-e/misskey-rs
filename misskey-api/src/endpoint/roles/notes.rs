use crate::model::{id::Id, note::Note, role::Role};

use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
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
    pub since_id: Option<Id<Note>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<Note>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option"
    )]
    #[builder(default, setter(strip_option, into))]
    pub since_date: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option"
    )]
    #[builder(default, setter(strip_option, into))]
    pub until_date: Option<DateTime<Utc>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "roles/notes";
}

impl_pagination!(Request, Note);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let role = client
            .admin
            .test(
                crate::endpoint::admin::roles::create::Request::builder()
                    .is_public(true)
                    .is_explorable(true)
                    .build(),
            )
            .await;

        client
            .test(Request {
                role_id: role.id,
                limit: None,
                since_id: None,
                until_id: None,
                since_date: None,
                until_date: None,
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
                    .is_explorable(true)
                    .build(),
            )
            .await;

        client
            .test(Request {
                role_id: role.id,
                limit: Some(100),
                since_id: None,
                until_id: None,
                since_date: None,
                until_date: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let (new_user, new_client) = client.admin.create_user().await;

        let role = client
            .admin
            .test(
                crate::endpoint::admin::roles::create::Request::builder()
                    .is_public(true)
                    .is_explorable(true)
                    .build(),
            )
            .await;
        client
            .admin
            .test(
                crate::endpoint::admin::roles::assign::Request::builder()
                    .role_id(role.id)
                    .user_id(new_user.id)
                    .build(),
            )
            .await;
        let note = new_client
            .test(
                crate::endpoint::notes::create::Request::builder()
                    .text("some text")
                    .build(),
            )
            .await
            .created_note;

        client
            .test(Request {
                role_id: role.id,
                limit: None,
                since_id: Some(note.id.clone()),
                until_id: Some(note.id.clone()),
                since_date: None,
                until_date: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_date() {
        let client = TestClient::new();
        let role = client
            .admin
            .test(
                crate::endpoint::admin::roles::create::Request::builder()
                    .is_public(true)
                    .is_explorable(true)
                    .build(),
            )
            .await;
        let now = chrono::Utc::now();

        client
            .test(Request {
                role_id: role.id,
                limit: None,
                since_id: None,
                until_id: None,
                since_date: Some(now),
                until_date: Some(now),
            })
            .await;
    }
}
