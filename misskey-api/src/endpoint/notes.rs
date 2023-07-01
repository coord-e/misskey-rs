use crate::model::{id::Id, note::Note};

use serde::Serialize;
use typed_builder::TypedBuilder;

pub mod children;
pub mod conversation;
pub mod create;
pub mod delete;
pub mod favorites;
pub mod featured;
pub mod global_timeline;
pub mod hybrid_timeline;
pub mod local_timeline;
pub mod mentions;
pub mod polls;
pub mod reactions;
pub mod renotes;
pub mod replies;
pub mod search;
pub mod search_by_tag;
pub mod show;
pub mod state;
pub mod timeline;
pub mod unrenote;
pub mod user_list_timeline;

#[cfg(feature = "12-58-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-58-0")))]
pub mod clips;

#[cfg(feature = "12-95-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-95-0")))]
pub mod thread_muting;

#[cfg(not(feature = "13-0-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
pub mod watching;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub local: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub renote: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub with_files: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub poll: Option<bool>,
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
}

impl misskey_core::Request for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes";
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
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .test(Request {
                local: None,
                reply: None,
                renote: None,
                with_files: None,
                poll: None,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let note = client.create_note(Some("test"), None, None).await;

        client
            .test(Request {
                local: None,
                reply: None,
                renote: None,
                with_files: None,
                poll: None,
                limit: None,
                since_id: Some(note.id.clone()),
                until_id: Some(note.id.clone()),
            })
            .await;
    }
}
