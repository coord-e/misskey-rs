use crate::model::drive::{DriveFile, DriveFileId};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<DriveFileId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<DriveFileId>,
}

impl ApiRequest for Request {
    type Response = Vec<DriveFile>;
    const ENDPOINT: &'static str = "drive/stream";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client.create_text_file("test.txt", "test").await;

        client
            .test(Request {
                type_: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let mut client = TestClient::new();
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
        let mut client = TestClient::new();
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
