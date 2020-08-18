use crate::model::ChannelId;

use misskey::model::{
    antenna::Antenna,
    file::DriveFile,
    messaging::MessagingMessage,
    note::{Note, NoteId},
    notification::Notification,
    signin::Signin,
    user::User,
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChannelMessage {
    MainStream {
        id: ChannelId,
        #[serde(flatten)]
        event: MainStreamEvent,
    },
    Timeline {
        id: ChannelId,
        #[serde(flatten)]
        note_posted: NotePostedMessage,
    },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", rename = "note", tag = "type")]
pub struct NotePostedMessage {
    pub body: Note,
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
