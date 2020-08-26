use crate::model::drive::{DriveFile, DriveFileId, DriveFolderId};

use serde::Serialize;
use typed_builder::TypedBuilder;

pub mod attached_notes;
pub mod check_existence;
pub mod create;
pub mod delete;
pub mod find;
pub mod find_by_hash;
pub mod show;
pub mod update;
pub mod upload_from_url;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub type_: Option<String>,
    #[builder(default, setter(strip_option))]
    pub folder_id: Option<DriveFolderId>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<DriveFileId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<DriveFileId>,
}

impl misskey_core::Request for Request {
    type Response = Vec<DriveFile>;
    const ENDPOINT: &'static str = "drive/files";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client.create_text_file("test.txt", "test").await;

        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_folder() {
        let mut client = TestClient::new();
        let folder = client
            .test(crate::endpoint::drive::folders::create::Request {
                name: None,
                parent_id: None,
            })
            .await;

        client
            .test(Request {
                type_: None,
                folder_id: Some(folder.id),
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
                folder_id: None,
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
                folder_id: None,
                limit: None,
                since_id: Some(file.id.clone()),
                until_id: Some(file.id.clone()),
            })
            .await;
    }
}
