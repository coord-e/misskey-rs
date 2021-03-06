use crate::model::{
    id::Id,
    note::{Note, Visibility},
};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub following: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub visibility: Option<Visibility>,
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
    const ENDPOINT: &'static str = "notes/mentions";
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
    async fn request_with_option() {
        let client = TestClient::new();
        client
            .test(Request {
                following: Some(true),
                visibility: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_visibilty() {
        use crate::model::note::Visibility;

        let client = TestClient::new();

        client
            .test(Request {
                following: None,
                visibility: Some(Visibility::Home),
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
        client
            .test(Request {
                following: None,
                visibility: Some(Visibility::Public),
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
        client
            .test(Request {
                following: None,
                visibility: Some(Visibility::Followers),
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
        client
            .test(Request {
                following: None,
                visibility: Some(Visibility::Specified),
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .test(Request {
                following: None,
                visibility: None,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let me = client.user.me().await;
        let text = format!("hey @{}", me.username);
        let note = client.admin.create_note(Some(&text), None, None).await;

        client
            .user
            .test(Request {
                following: None,
                visibility: None,
                limit: None,
                since_id: Some(note.id.clone()),
                until_id: Some(note.id.clone()),
            })
            .await;
    }
}
