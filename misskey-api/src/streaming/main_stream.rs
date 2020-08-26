use crate::model::{
    antenna::Antenna,
    drive::DriveFile,
    messaging::MessagingMessage,
    note::{Note, NoteId},
    notification::Notification,
    signin::Signin,
    user::User,
};

use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Request {}

impl Serialize for Request {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Request", 1)?;
        state.serialize_field("channel", "main")?;
        state.end()
    }
}

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

impl misskey_core::streaming::SubscriptionRequest for Request {
    type Item = MainStreamEvent;
    const TYPE: &'static str = "connect";
}

impl misskey_core::streaming::SubscriptionItem for MainStreamEvent {
    const TYPE: &'static str = "channel";
    const UNSUBSCRIBE_REQUEST_TYPE: &'static str = "disconnect";
}
