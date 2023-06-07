#[cfg(feature = "12-9-0")]
use crate::model::drive::DriveFile;
#[cfg(feature = "13-13-0")]
use crate::model::role::Role;
use crate::model::{emoji::Emoji, id::Id};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[cfg(any(docsrs, not(feature = "12-9-0")))]
use url::Url;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[cfg(feature = "12-9-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-9-0")))]
    pub file_id: Id<DriveFile>,
    #[cfg(any(docsrs, not(feature = "12-9-0"), feature = "13-13-0"))]
    #[cfg_attr(docsrs, doc(cfg(any(not(feature = "12-9-0"), feature = "13-13-0"))))]
    #[builder(setter(into))]
    pub name: String,
    #[cfg(any(docsrs, not(feature = "12-9-0")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-9-0"))))]
    pub url: Url,
    #[cfg(any(docsrs, not(feature = "12-9-0"), feature = "13-13-0"))]
    #[cfg_attr(docsrs, doc(cfg(any(not(feature = "12-9-0"), feature = "13-13-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub category: Option<String>,
    #[cfg(any(docsrs, not(feature = "12-9-0"), feature = "13-13-0"))]
    #[cfg_attr(docsrs, doc(cfg(any(not(feature = "12-9-0"), feature = "13-13-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub aliases: Option<Vec<String>>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub license: Option<String>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_sensitive: Option<bool>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub local_only: Option<bool>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub role_ids_that_can_be_used_this_emoji_as_reaction: Option<Vec<Id<Role>>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: Id<Emoji>,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "admin/emoji/add";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[cfg(any(not(feature = "12-9-0"), feature = "13-13-0"))]
    use ulid_crate::Ulid;

    #[tokio::test]
    #[cfg(all(feature = "12-9-0", not(feature = "13-13-0")))]
    async fn request() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let file = client.upload_from_url(image_url).await;
        client.admin.test(Request { file_id: file.id }).await;
    }

    #[tokio::test]
    #[cfg(not(feature = "12-9-0"))]
    async fn request() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let name = Ulid::new().to_string();

        client
            .admin
            .test(Request {
                name,
                url: image_url,
                category: None,
                aliases: None,
            })
            .await;
    }

    #[tokio::test]
    #[cfg(not(feature = "12-9-0"))]
    async fn request_with_options() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let name = Ulid::new().to_string();

        client
            .admin
            .test(Request {
                name,
                url: image_url,
                category: Some("nice".to_string()),
                aliases: Some(vec!["test2".to_string()]),
            })
            .await;
    }

    #[tokio::test]
    #[cfg(feature = "13-13-0")]
    async fn request() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let file = client.upload_from_url(image_url).await;
        let name = Ulid::new().to_string();

        client
            .admin
            .test(Request {
                file_id: file.id,
                name,
                aliases: None,
                category: None,
                is_sensitive: None,
                license: None,
                local_only: None,
                role_ids_that_can_be_used_this_emoji_as_reaction: None,
            })
            .await;
    }

    #[tokio::test]
    #[cfg(feature = "13-13-0")]
    async fn request_with_options() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let file = client.upload_from_url(image_url).await;
        let name = Ulid::new().to_string();
        let role = client
            .admin
            .test(crate::endpoint::admin::roles::create::Request::default())
            .await;

        client
            .admin
            .test(Request {
                file_id: file.id,
                name,
                aliases: Some(vec!["alias".to_string()]),
                category: Some("cat".to_string()),
                is_sensitive: Some(true),
                license: Some("license".to_string()),
                local_only: Some(true),
                role_ids_that_can_be_used_this_emoji_as_reaction: Some(vec![role.id]),
            })
            .await;
    }
}
