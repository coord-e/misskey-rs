use crate::model::{emoji::Emoji, id::Id};

use serde::Serialize;
use typed_builder::TypedBuilder;
#[cfg(any(docsrs, not(feature = "12-9-0")))]
use url::Url;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub id: Id<Emoji>,
    #[builder(setter(into))]
    pub name: String,
    #[builder(default, setter(strip_option, into))]
    pub category: Option<String>,
    #[builder(default)]
    pub aliases: Vec<String>,
    #[cfg(any(docsrs, not(feature = "12-9-0")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-9-0"))))]
    pub url: Url,
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub license: Option<String>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/emoji/update";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let id = client.admin.add_emoji_from_url(image_url.clone()).await;
        let name = ulid_crate::Ulid::new().to_string();

        client
            .admin
            .test(Request {
                id,
                name,
                category: Some("great".to_string()),
                aliases: vec!["namename".to_string()],
                #[cfg(not(feature = "12-9-0"))]
                url: image_url,
                #[cfg(feature = "13-10-0")]
                license: Some("license".to_string()),
            })
            .await;
    }
}
