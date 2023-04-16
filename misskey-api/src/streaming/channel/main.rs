#[cfg(feature = "12-67-0")]
use crate::model::registry::{RegistryKey, RegistryScope, RegistryValue};
use crate::model::{
    antenna::Antenna, drive::DriveFile, id::Id, messaging::MessagingMessage, note::Note,
    notification::Notification, signin::Signin, user::User,
};
use crate::streaming::channel::NoOutgoing;

use serde::{Deserialize, Serialize};
#[cfg(not(feature = "12-67-0"))]
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum MainStreamEvent {
    #[cfg(not(feature = "12-67-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-67-0"))))]
    ClientSettingUpdated {
        key: String,
        value: Value,
    },
    #[cfg(feature = "12-67-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-67-0")))]
    RegistryUpdated {
        scope: RegistryScope,
        key: RegistryKey,
        value: RegistryValue,
    },
    ReceiveFollowRequest(User),
    Notification(Notification),
    MeUpdated(User),
    MessagingMessage(MessagingMessage),
    ReadAllNotifications,
    ReadAllUnreadMentions,
    ReadAllAntennas,
    ReadAllUnreadSpecifiedNotes,
    ReadAllMessagingMessages,
    ReadAllAnnouncements,
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    ReadAllChannels,
    MyTokenRegenerated,
    #[cfg(not(feature = "12-102-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-102-0"))))]
    ReversiNoInvites,
    /// TODO: Implement
    #[cfg(not(feature = "12-102-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-102-0"))))]
    ReversiInvited {},
    /// TODO: Implement
    PageEvent {},
    Signin(Signin),
    Unfollow(User),
    Follow(User),
    Followed(User),
    Reply(Note),
    Mention(Note),
    Renote(Note),
    ReadAntenna(Antenna),
    UnreadMention(Id<Note>),
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    UnreadChannel(Id<Note>),
    UnreadSpecifiedNote(Id<Note>),
    UnreadMessagingMessage(MessagingMessage),
    UnreadNotification(Notification),
    UnreadAntenna(Antenna),
    DriveFileCreated(DriveFile),
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    UrlUploadFinished {
        marker: Option<String>,
        file: DriveFile,
    },
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct Request {}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = MainStreamEvent;
    type Outgoing = NoOutgoing;

    const NAME: &'static str = "main";
}

#[cfg(test)]
mod tests {
    use super::{MainStreamEvent, Request};
    use crate::test::{http::TestClient as HttpTestClient, websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;
        let mut stream = client.channel(Request::default()).await.unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn reply() {
        let admin_client = HttpTestClient::new().admin;
        // create fresh user (for new main stream) to avoid events
        // to be captured by other test cases
        let (_, http_client, client) = admin_client.create_http_and_ws_client().await;

        let mut stream = client.channel(Request::default()).await.unwrap();

        future::join(
            async {
                let note = http_client.create_note(Some("awesome"), None, None).await;
                admin_client
                    .create_note(Some("nice"), None, Some(note.id))
                    .await;
            },
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        MainStreamEvent::Reply(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }

    #[tokio::test]
    async fn mention() {
        let http_client = HttpTestClient::new();
        // ditto
        let (me, client) = http_client.admin.create_streaming_user().await;

        let mut stream = client.channel(Request::default()).await.unwrap();

        futures::future::join(
            http_client
                .user
                .create_note(Some(&format!("@{} hello", me.username)), None, None),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        MainStreamEvent::Mention(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }

    #[cfg(feature = "12-48-0")]
    #[tokio::test]
    async fn url_upload_finished() {
        use crate::model::drive::DriveFile;

        // ditto
        let (_, http_client, client) = HttpTestClient::new()
            .admin
            .create_http_and_ws_client()
            .await;

        let mut stream = client.channel(Request::default()).await.unwrap();

        let url = http_client.avatar_url().await;
        let expected_marker = ulid_crate::Ulid::new().to_string();
        let expected_comment = ulid_crate::Ulid::new().to_string();

        futures::future::join(
            http_client.test(crate::endpoint::drive::files::upload_from_url::Request {
                url,
                folder_id: None,
                is_sensitive: None,
                force: None,
                marker: Some(expected_marker.clone()),
                comment: Some(expected_comment.clone()),
            }),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        MainStreamEvent::UrlUploadFinished {
                            marker: Some(marker),
                            file:
                                DriveFile {
                                    comment: Some(comment),
                                    ..
                                },
                        } if marker == expected_marker && comment == expected_comment => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }

    #[cfg(feature = "12-67-0")]
    #[tokio::test]
    async fn registry_updated() {
        use serde_json::json;

        let http_client = HttpTestClient::new();
        // ditto
        let (_, http_client, client) = http_client.admin.create_http_and_ws_client().await;

        let mut stream = client.channel(Request::default()).await.unwrap();

        future::join(
            http_client.test(crate::endpoint::i::registry::set::Request {
                key: "stream_test".into(),
                value: json!({ "test": [] }),
                scope: None,
            }),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        MainStreamEvent::RegistryUpdated { .. } => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }

    // TODO: Test the other events
}
