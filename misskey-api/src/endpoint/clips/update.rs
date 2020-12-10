use crate::model::{clip::Clip, id::Id};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub clip_id: Id<Clip>,
    /// [ 1 .. 100 ] characters
    pub name: String,
    #[cfg(feature = "12-57-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
    #[builder(default)]
    pub is_public: bool,
    #[cfg(feature = "12-57-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
}

impl misskey_core::Request for Request {
    type Response = Clip;
    const ENDPOINT: &'static str = "clips/update";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let clip = client
            .test(crate::endpoint::clips::create::Request {
                name: "clip".to_string(),
                #[cfg(feature = "12-57-0")]
                is_public: None,
                #[cfg(feature = "12-57-0")]
                description: None,
            })
            .await;

        client
            .test(Request {
                clip_id: clip.id,
                name: "updated".to_string(),
                #[cfg(feature = "12-57-0")]
                is_public: true,
                #[cfg(feature = "12-57-0")]
                description: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        let clip = client
            .test(crate::endpoint::clips::create::Request {
                name: "clip".to_string(),
                #[cfg(feature = "12-57-0")]
                is_public: None,
                #[cfg(feature = "12-57-0")]
                description: None,
            })
            .await;

        client
            .test(Request {
                clip_id: clip.id,
                name: "updated".to_string(),
                #[cfg(feature = "12-57-0")]
                is_public: true,
                #[cfg(feature = "12-57-0")]
                description: Some("description".to_string()),
            })
            .await;
    }
}
