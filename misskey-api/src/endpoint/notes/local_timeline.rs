use crate::model::{id::Id, note::Note};

use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use mime::Mime;
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
    pub exclude_nsfw: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serde::serialize_string_vec_option"
    )]
    #[builder(default, setter(strip_option))]
    pub file_type: Option<Vec<Mime>>,
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
    const ENDPOINT: &'static str = "notes/local-timeline";
}

impl_pagination!(Request, Note);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    use mime::IMAGE_PNG;

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
                exclude_nsfw: Some(true),
                file_type: None,
                limit: None,
                since_id: None,
                until_id: None,
                since_date: None,
                until_date: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_file_type() {
        let client = TestClient::new();
        client
            .test(Request {
                with_files: None,
                #[cfg(feature = "13-13-0")]
                with_replies: None,
                exclude_nsfw: None,
                file_type: Some(vec![IMAGE_PNG]),
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
                exclude_nsfw: None,
                file_type: None,
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
                exclude_nsfw: None,
                file_type: None,
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
                exclude_nsfw: None,
                file_type: None,
                limit: None,
                since_id: None,
                until_id: None,
                since_date: Some(now),
                until_date: Some(now),
            })
            .await;
    }
}
