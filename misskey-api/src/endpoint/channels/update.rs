#[cfg(feature = "13-11-0")]
use crate::model::note::Note;
use crate::model::{channel::Channel, drive::DriveFile, id::Id};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub channel_id: Id<Channel>,
    /// [ 1 .. 128 ] characters
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,
    /// [ 1 .. 2048 ] characters
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub banner_id: Option<Option<Id<DriveFile>>>,
    #[cfg(feature = "13-12-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-12-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_archived: Option<bool>,
    #[cfg(feature = "13-11-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-11-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub pinned_note_ids: Option<Vec<Id<Note>>>,
    /// [ 1 .. 16 ] characters
    #[cfg(feature = "13-12-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-12-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub color: Option<String>,
}

impl misskey_core::Request for Request {
    type Response = Channel;
    const ENDPOINT: &'static str = "channels/update";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_with_name() {
        let client = TestClient::new();
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test channel")
                    .build(),
            )
            .await;

        client.test(Request {
            channel_id: channel.id,
            // random 128 chars
            name: Some("yCdKKHkRAmqhE49j3TBCVnnsQiZ2KkCK83z6NNKoGaiqQNOALsAOIz6XVJAcaV9lNbQYuuhSe7nAM8qdvUtokhWYWDO999z07N4ajtikDmzANpgwRh7rxMgb8PLsgcbm".to_string()),
            description: None,
            banner_id: None,
            #[cfg(feature = "13-12-0")]
            is_archived: None,
            #[cfg(feature = "13-11-0")]
            pinned_note_ids: None,
            #[cfg(feature = "13-12-0")]
            color: None
        }).await;
    }

    #[tokio::test]
    async fn request_with_description() {
        let client = TestClient::new();
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test channel")
                    .build(),
            )
            .await;

        client
            .test(Request {
                channel_id: channel.id.clone(),
                name: None,
                description: Some(Some("description".to_string())),
                banner_id: None,
                #[cfg(feature = "13-12-0")]
                is_archived: None,
                #[cfg(feature = "13-11-0")]
                pinned_note_ids: None,
                #[cfg(feature = "13-12-0")]
                color: None,
            })
            .await;
        client
            .test(Request {
                channel_id: channel.id,
                name: None,
                description: Some(None),
                banner_id: None,
                #[cfg(feature = "13-12-0")]
                is_archived: None,
                #[cfg(feature = "13-11-0")]
                pinned_note_ids: None,
                #[cfg(feature = "13-12-0")]
                color: None,
            })
            .await;
    }

    #[cfg(feature = "13-12-0")]
    #[tokio::test]
    async fn request_with_is_archived() {
        let client = TestClient::new();
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test channel")
                    .build(),
            )
            .await;

        client
            .test(Request {
                channel_id: channel.id.clone(),
                name: None,
                description: None,
                banner_id: None,
                is_archived: Some(true),
                pinned_note_ids: None,
                color: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_banner() {
        let client = TestClient::new();
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test channel")
                    .build(),
            )
            .await;
        let url = client.avatar_url().await;
        let file = client.upload_from_url(url).await;

        client
            .test(Request {
                channel_id: channel.id.clone(),
                name: None,
                description: None,
                banner_id: Some(Some(file.id)),
                #[cfg(feature = "13-12-0")]
                is_archived: None,
                #[cfg(feature = "13-11-0")]
                pinned_note_ids: None,
                #[cfg(feature = "13-12-0")]
                color: None,
            })
            .await;
        // bug in misskey
        // client
        //     .test(Request {
        //         channel_id: channel.id,
        //         name: None,
        //         description: None,
        //         banner_id: Some(None),
        //     })
        //     .await;
    }

    #[cfg(feature = "13-11-0")]
    #[tokio::test]
    async fn request_with_pinned_note_ids() {
        let client = TestClient::new();
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test channel")
                    .build(),
            )
            .await;
        let note = client.create_note(Some("test"), None, None).await;

        client
            .test(Request {
                channel_id: channel.id,
                name: None,
                description: None,
                banner_id: None,
                #[cfg(feature = "13-12-0")]
                is_archived: None,
                pinned_note_ids: Some(vec![note.id]),
                #[cfg(feature = "13-12-0")]
                color: None,
            })
            .await;
    }

    #[cfg(feature = "13-12-0")]
    #[tokio::test]
    async fn request_with_color() {
        let client = TestClient::new();
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test channel")
                    .build(),
            )
            .await;

        client
            .test(Request {
                channel_id: channel.id.clone(),
                name: None,
                description: None,
                banner_id: None,
                is_archived: None,
                pinned_note_ids: None,
                color: Some("#ff0000".to_string()),
            })
            .await;
    }
}
