use crate::model::{id::Id, note::Note};

use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub with_files: Option<bool>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub with_replies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_my_renotes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_renoted_my_notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_local_renotes: Option<bool>,
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
    const ENDPOINT: &'static str = "notes/hybrid-timeline";
}

impl_pagination!(Request, Note);

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
    async fn request_with_options() {
        let client = TestClient::new();
        client
            .test(Request {
                with_files: Some(true),
                #[cfg(feature = "13-13-0")]
                with_replies: Some(true),
                include_my_renotes: Some(false),
                include_renoted_my_notes: Some(false),
                include_local_renotes: Some(false),
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
        client
            .test(Request {
                with_files: None,
                #[cfg(feature = "13-13-0")]
                with_replies: None,
                include_my_renotes: None,
                include_renoted_my_notes: None,
                include_local_renotes: None,
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
        let note = client.create_note(Some("test"), None, None).await;

        client
            .test(Request {
                with_files: None,
                #[cfg(feature = "13-13-0")]
                with_replies: None,
                include_my_renotes: None,
                include_renoted_my_notes: None,
                include_local_renotes: None,
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
        let now = chrono::Utc::now();

        client
            .test(Request {
                with_files: None,
                #[cfg(feature = "13-13-0")]
                with_replies: None,
                include_my_renotes: None,
                include_renoted_my_notes: None,
                include_local_renotes: None,
                limit: None,
                since_id: None,
                until_id: None,
                since_date: Some(now),
                until_date: Some(now),
            })
            .await;
    }
}
