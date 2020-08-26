use crate::model::note_favorite::{NoteFavorite, NoteFavoriteId};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<NoteFavoriteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<NoteFavoriteId>,
}

impl misskey_core::Request for Request {
    type Response = Vec<NoteFavorite>;
    const ENDPOINT: &'static str = "i/favorites";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let mut client = TestClient::new();
        client
            .test(Request {
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let mut client = TestClient::new();
        let note = client.admin.create_note(Some("test"), None, None).await;
        client
            .user
            .test(crate::api::notes::favorites::create::Request { note_id: note.id })
            .await;

        let favorites = client
            .test(Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;

        client
            .test(Request {
                limit: None,
                since_id: Some(favorites[0].id.clone()),
                until_id: Some(favorites[0].id.clone()),
            })
            .await;
    }
}
