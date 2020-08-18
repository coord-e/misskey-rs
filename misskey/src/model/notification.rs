use crate::model::{
    note::{Note, Reaction},
    user::{User, UserId},
    user_group::UserGroupInvitation,
};

use chrono::{DateTime, Utc};
use derive_more::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug)]
#[serde(transparent)]
pub struct NotificationId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: NotificationId,
    pub created_at: DateTime<Utc>,
    pub user_id: UserId,
    pub user: User,
    pub is_read: bool,
    #[serde(flatten)]
    pub body: NotificationBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum NotificationBody {
    Follow,
    FollowRequestAccepted,
    ReceiveFollowRequest,
    Mention {
        note: Note,
    },
    Reply {
        note: Note,
    },
    Renote {
        note: Note,
    },
    Quote {
        note: Note,
    },
    Reaction {
        note: Note,
        reaction: Reaction,
    },
    PollVote {
        note: Note,
        choice: u64,
    },
    GroupInvited {
        invitation: UserGroupInvitation,
    },
    /// TODO: Implement
    App {},
}
