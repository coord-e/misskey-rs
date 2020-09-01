use crate::model::{
    channel::ChannelId,
    note::{Note, NoteId},
};

use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub channel_id: ChannelId,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<NoteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<NoteId>,
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
    const ENDPOINT: &'static str = "channels/timeline";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let channel = client
            .test(crate::endpoint::channels::create::Request {
                name: "test".to_string(),
                description: None,
                banner_id: None,
            })
            .await;

        client
            .test(Request {
                channel_id: channel.id,
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
        let channel = client
            .test(crate::endpoint::channels::create::Request {
                name: "test".to_string(),
                description: None,
                banner_id: None,
            })
            .await;

        client
            .test(Request {
                channel_id: channel.id,
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
        let channel = client
            .test(crate::endpoint::channels::create::Request {
                name: "test".to_string(),
                description: None,
                banner_id: None,
            })
            .await;
        let note = client
            .test(crate::endpoint::notes::create::Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("some text".to_string()),
                cw: None,
                via_mobile: None,
                local_only: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                channel_id: Some(channel.id.clone()),
            })
            .await
            .created_note;

        client
            .test(Request {
                channel_id: channel.id,
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
        let channel = client
            .test(crate::endpoint::channels::create::Request {
                name: "test".to_string(),
                description: None,
                banner_id: None,
            })
            .await;
        let now = chrono::Utc::now();

        client
            .test(Request {
                channel_id: channel.id,
                limit: None,
                since_id: None,
                until_id: None,
                since_date: Some(now),
                until_date: Some(now),
            })
            .await;
    }
}
