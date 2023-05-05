#[cfg(feature = "13-10-0")]
use crate::model::{drive::DriveFileSortKey, sort::SortOrder};
use crate::model::{
    drive::{DriveFile, DriveFolder},
    id::Id,
};

use mime::Mime;
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
    #[serde(
        rename = "type",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serde::serialize_string_option"
    )]
    #[builder(default, setter(strip_option))]
    pub type_: Option<Mime>,
    #[builder(default, setter(strip_option))]
    pub folder_id: Option<Id<DriveFolder>>,
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
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<SortOrder<DriveFileSortKey>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<DriveFile>;
    const ENDPOINT: &'static str = "drive/files";
}

impl_pagination!(Request, DriveFile);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    use mime::TEXT_PLAIN;

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.create_text_file("test.txt", "test").await;

        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_folder() {
        let client = TestClient::new();
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
                #[cfg(feature = "13-10-0")]
                sort: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        client.create_text_file("test.txt", "test").await;

        client
            .test(Request {
                type_: Some(TEXT_PLAIN),
                folder_id: None,
                limit: Some(100),
                since_id: None,
                until_id: None,
                #[cfg(feature = "13-10-0")]
                sort: None,
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
                folder_id: None,
                limit: None,
                since_id: Some(file.id.clone()),
                until_id: Some(file.id.clone()),
                #[cfg(feature = "13-10-0")]
                sort: None,
            })
            .await;
    }

    #[cfg(feature = "13-10-0")]
    #[tokio::test]
    async fn request_with_sort() {
        use crate::model::{drive::DriveFileSortKey, sort::SortOrder};

        let client = TestClient::new();
        client.create_text_file("test.txt", "test").await;

        client
            .test(Request {
                type_: None,
                folder_id: None,
                limit: None,
                since_id: None,
                until_id: None,
                sort: Some(SortOrder::Ascending(DriveFileSortKey::CreatedAt)),
            })
            .await;
        client
            .test(Request {
                type_: None,
                folder_id: None,
                limit: None,
                since_id: None,
                until_id: None,
                sort: Some(SortOrder::Ascending(DriveFileSortKey::Name)),
            })
            .await;
        client
            .test(Request {
                type_: None,
                folder_id: None,
                limit: None,
                since_id: None,
                until_id: None,
                sort: Some(SortOrder::Descending(DriveFileSortKey::Size)),
            })
            .await;
    }
}
