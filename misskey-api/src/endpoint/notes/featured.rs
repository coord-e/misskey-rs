use crate::model::note::Note;
#[cfg(feature = "13-8-0")]
use crate::model::{channel::Channel, id::Id};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub offset: Option<u64>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub limit: Option<u8>,
    #[cfg(feature = "13-8-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-8-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub channel_id: Option<Id<Channel>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes/featured";
}

impl_offset_pagination!(Request, Note);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_offset() {
        let client = TestClient::new();
        client
            .test(Request {
                offset: Some(5),
                limit: None,
                #[cfg(feature = "13-8-0")]
                channel_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .test(Request {
                offset: None,
                limit: Some(100),
                #[cfg(feature = "13-8-0")]
                channel_id: None,
            })
            .await;
    }

    #[cfg(feature = "13-8-0")]
    #[tokio::test]
    async fn request_with_channel_id() {
        let client = TestClient::new();
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test")
                    .build(),
            )
            .await;

        client
            .test(Request {
                offset: None,
                limit: None,
                channel_id: Some(channel.id),
            })
            .await;
    }
}
