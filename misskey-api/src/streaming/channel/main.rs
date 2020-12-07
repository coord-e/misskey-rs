use crate::model::{
    antenna::Antenna, drive::DriveFile, id::Id, messaging::MessagingMessage, note::Note,
    notification::Notification, signin::Signin, user::User,
};
use crate::streaming::channel::NoOutgoing;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum MainStreamEvent {
    ClientSettingUpdated {
        key: String,
        value: Value,
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
    MyTokenRegenerated,
    ReversiNoInvites,
    /// TODO: Implement
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
    UnreadSpecifiedNote(Id<Note>),
    UnreadMessagingMessage(MessagingMessage),
    UnreadNotification(Notification),
    UnreadAntenna(Antenna),
    DriveFileCreated(DriveFile),
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
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;
        let mut stream = client.channel(Request::default()).await.unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn reply() {
        let client = TestClient::new().await;

        let mut stream = client.user.channel(Request::default()).await.unwrap();

        future::join(
            async {
                let note = client.user.create_note(Some("awesome"), None, None).await;
                client
                    .admin
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
        let client = TestClient::new().await;
        let me = client.user.me().await;

        let mut stream = client.user.channel(Request::default()).await.unwrap();

        futures::future::join(
            client
                .admin
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

    // TODO: Test the other events
}
