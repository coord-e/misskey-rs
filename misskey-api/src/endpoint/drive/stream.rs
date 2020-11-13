use crate::model::{drive::DriveFile, id::Id};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub type_: Option<String>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<DriveFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<DriveFile>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<DriveFile>;
    const ENDPOINT: &'static str = "drive/stream";
}

impl_pagination!(Request, DriveFile);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.create_text_file("test.txt", "test").await;

        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        client.create_text_file("test.txt", "test").await;

        client
            .test(Request {
                type_: Some("text/plain".to_string()),
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let file = client.create_text_file("test.txt", "test").await;

        client
            .test(Request {
                type_: None,
                limit: None,
                since_id: Some(file.id.clone()),
                until_id: Some(file.id.clone()),
            })
            .await;
    }
}
