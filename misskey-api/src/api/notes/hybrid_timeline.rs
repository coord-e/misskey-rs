use crate::model::note::{Note, NoteId};

use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_files: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_my_renotes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_renoted_my_notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_local_renotes: Option<bool>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<NoteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<NoteId>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option"
    )]
    pub since_date: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option"
    )]
    pub until_date: Option<DateTime<Utc>>,
}

impl ApiRequest for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes/hybrid-timeline";
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
                with_files: None,
                include_my_renotes: None,
                include_renoted_my_notes: None,
                include_local_renotes: None,
                limit: None,
                since_id: None,
                until_id: None,
                since_date: None,
                until_date: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let mut client = TestClient::new();
        client
            .test(Request {
                with_files: Some(true),
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
        let mut client = TestClient::new();
        client
            .test(Request {
                with_files: None,
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
        let mut client = TestClient::new();
        let note = client.create_note(Some("test"), None, None).await;

        client
            .test(Request {
                with_files: None,
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
        let mut client = TestClient::new();
        let now = chrono::Utc::now();

        client
            .test(Request {
                with_files: None,
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
