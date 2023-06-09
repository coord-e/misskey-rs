use crate::model::{id::Id, note_reaction::NoteReaction, user::User};

use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub user_id: Id<User>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<NoteReaction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<NoteReaction>>,
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
    type Response = Vec<NoteReaction>;
    const ENDPOINT: &'static str = "users/reactions";
}

impl_pagination!(Request, NoteReaction);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let user = client.me().await;

        client
            .test(Request {
                user_id: user.id,
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
        let user = client.me().await;

        client
            .test(Request {
                user_id: user.id,
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
        let user = client.user.me().await;
        let note = client.admin.create_note(Some("hello"), None, None).await;

        client
            .user
            .test(crate::endpoint::notes::reactions::create::Request {
                note_id: note.id,
                reaction: "👍".into(),
            })
            .await;

        let reactions = client
            .test(Request::builder().user_id(user.id).build())
            .await;

        client
            .test(Request {
                user_id: user.id,
                limit: None,
                since_id: Some(reactions[0].id.clone()),
                until_id: Some(reactions[0].id.clone()),
                since_date: None,
                until_date: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_date() {
        let client = TestClient::new();
        let user = client.me().await;
        let now = chrono::Utc::now();

        client
            .test(Request {
                user_id: user.id,
                limit: None,
                since_id: None,
                until_id: None,
                since_date: Some(now),
                until_date: Some(now),
            })
            .await;
    }
}
