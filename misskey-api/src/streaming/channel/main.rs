use crate::model::{
    antenna::Antenna,
    drive::DriveFile,
    messaging::MessagingMessage,
    note::{Note, NoteId},
    notification::Notification,
    signin::Signin,
    user::User,
};

use serde::Deserialize;
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
    UnreadMention(NoteId),
    UnreadSpecifiedNote(NoteId),
    UnreadMessagingMessage(MessagingMessage),
    UnreadNotification(Notification),
    UnreadAntenna(Antenna),
    DriveFileCreated(DriveFile),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Main {
    #[serde(flatten)]
    pub event: MainStreamEvent,
}

impl crate::streaming::channel::Channel for Main {
    const NAME: &'static str = "main";
}

#[cfg(test)]
mod tests {
    use super::{Main, MainStreamEvent};
    use crate::streaming::channel::ConnectRequest;
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};
    use misskey_core::streaming::SubscriptionClient;

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let mut client = TestClient::new().await;
        let mut stream = client
            .subscribe(ConnectRequest::<Main>::new())
            .await
            .unwrap();
        stream.unsubscribe().await.unwrap();
    }

    #[tokio::test]
    async fn reply() {
        let mut client = TestClient::new().await;

        let mut stream = client
            .user
            .subscribe(ConnectRequest::<Main>::new())
            .await
            .unwrap();

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
                    match stream.next().await.unwrap().unwrap().into_inner().event {
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
        let mut client = TestClient::new().await;
        let me = client.user.me().await;

        let mut stream = client
            .user
            .subscribe(ConnectRequest::<Main>::new())
            .await
            .unwrap();

        futures::future::join(
            client
                .admin
                .create_note(Some(&format!("@{} hello", me.username)), None, None),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap().into_inner().event {
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
