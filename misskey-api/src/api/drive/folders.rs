use crate::model::drive::{DriveFolder, DriveFolderId};

use misskey_core::ApiRequest;
use serde::Serialize;

pub mod create;
pub mod delete;
pub mod find;
pub mod show;
pub mod update;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub folder_id: Option<DriveFolderId>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<DriveFolderId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<DriveFolderId>,
}

impl ApiRequest for Request {
    type Response = Vec<DriveFolder>;
    const ENDPOINT: &'static str = "drive/folders";
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
                folder_id: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_folder() {
        let mut client = TestClient::new();
        let folder1 = client
            .test(crate::api::drive::folders::create::Request {
                name: None,
                parent_id: None,
            })
            .await;
        client
            .test(crate::api::drive::folders::create::Request {
                name: None,
                parent_id: Some(folder1.id.clone()),
            })
            .await;

        client
            .test(Request {
                folder_id: Some(folder1.id),
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
        let folder = client
            .test(crate::api::drive::folders::create::Request {
                name: None,
                parent_id: None,
            })
            .await;

        client
            .test(Request {
                folder_id: None,
                limit: None,
                since_id: Some(folder.id.clone()),
                until_id: Some(folder.id.clone()),
            })
            .await;
    }
}
