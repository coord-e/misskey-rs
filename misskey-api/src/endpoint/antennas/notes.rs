use crate::model::{antenna::Antenna, id::Id, note::Note};

#[cfg(feature = "12-98-0")]
use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub antenna_id: Id<Antenna>,
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
    #[cfg(feature = "12-98-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-98-0")))]
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option"
    )]
    #[builder(default, setter(strip_option, into))]
    pub since_date: Option<DateTime<Utc>>,
    #[cfg(feature = "12-98-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-98-0")))]
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option"
    )]
    #[builder(default, setter(strip_option, into))]
    pub until_date: Option<DateTime<Utc>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "antennas/notes";
}

impl_pagination!(Request, Note);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let antenna = client
            .test(
                crate::endpoint::antennas::create::Request::builder()
                    .name("test")
                    .build(),
            )
            .await;

        client
            .user
            .test(Request {
                antenna_id: antenna.id,
                limit: None,
                since_id: None,
                until_id: None,
                #[cfg(feature = "12-98-0")]
                since_date: None,
                #[cfg(feature = "12-98-0")]
                until_date: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        let antenna = client
            .test(
                crate::endpoint::antennas::create::Request::builder()
                    .name("test")
                    .keywords("hello awesome")
                    .build(),
            )
            .await;

        client
            .user
            .test(Request {
                antenna_id: antenna.id,
                limit: Some(100),
                since_id: None,
                until_id: None,
                #[cfg(feature = "12-98-0")]
                since_date: None,
                #[cfg(feature = "12-98-0")]
                until_date: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let antenna = client
            .test(
                crate::endpoint::antennas::create::Request::builder()
                    .name("test")
                    .keywords("hello awesome")
                    .build(),
            )
            .await;
        let note = client
            .admin
            .create_note(Some("hello, world"), None, None)
            .await;

        client
            .user
            .test(Request {
                antenna_id: antenna.id,
                limit: None,
                since_id: Some(note.id.clone()),
                until_id: Some(note.id.clone()),
                #[cfg(feature = "12-98-0")]
                since_date: None,
                #[cfg(feature = "12-98-0")]
                until_date: None,
            })
            .await;
    }

    #[cfg(feature = "12-98-0")]
    #[tokio::test]
    async fn request_with_date() {
        let client = TestClient::new();
        let antenna = client
            .test(
                crate::endpoint::antennas::create::Request::builder()
                    .name("test")
                    .keywords("hello awesome")
                    .build(),
            )
            .await;
        let now = chrono::Utc::now();

        client
            .user
            .test(Request {
                antenna_id: antenna.id,
                limit: None,
                since_id: None,
                until_id: None,
                since_date: Some(now),
                until_date: Some(now),
            })
            .await;
    }
}
