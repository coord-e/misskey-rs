use crate::model::note::{Note, NoteId, Visibility};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<NoteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<NoteId>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes/mentions";
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
                following: None,
                visibility: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_option() {
        let mut client = TestClient::new();
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

        let mut client = TestClient::new();

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
        let mut client = TestClient::new();
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
        let mut client = TestClient::new();
        // TODO: mention test user in the text
        let note = client.create_note(Some("test1"), None, None).await;

        client
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
