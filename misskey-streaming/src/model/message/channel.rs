use crate::model::ChannelId;

use misskey::model::{
    note::{Note, NoteId},
    user::User,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum NotePostedMessage {
    Note {
        #[serde(flatten)]
        note: Note,
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum MainStreamEvent {
    ClientSettingUpdated {},
    ReceiveFollowRequest {},
    Notification {},
    MeUpdated {},
    UnreadMention(NoteId),
    UnreadSpecifiedNote(NoteId),
    UnreadMessagingMessage {},
    UnreadNotification {},
    MessagingMessage {},
    ReadAntenna {},
    ReadAllNotifications,
    ReadAllUnreadMentions,
    ReadAllAntennas,
    ReadAllUnreadSpecifiedNotes,
    ReversiNoInvites,
    ReversiInvited {},
    MyTokenRegenerated,
    PageEvent {},
    Signin {},
    ReadAllMessagingMessages,
    ReadAllAnnouncements,
    Unfollow {},
    UnreadAntenna {},
    Follow {},
    Followed(User),
    Reply(Note),
    Mention(Note),
    Renote(Note),
    DriveFileCreated {},
}
