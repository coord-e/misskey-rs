use crate::model::{
    note::{Note, Reaction},
    user::{User, UserId},
    user_group::UserGroupInvitation,
};

use chrono::{DateTime, Utc};
use derive_more::{Display, Error, FromStr};
use serde::{Deserialize, Serialize};
use strum_macros::EnumDiscriminants;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct NotificationId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: NotificationId,
    pub created_at: DateTime<Utc>,
    pub user_id: UserId,
    pub user: User,
    #[cfg(feature = "12-40-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-40-0")))]
    pub is_read: bool,
    #[serde(flatten)]
    pub body: NotificationBody,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumDiscriminants)]
#[serde(rename_all = "camelCase", tag = "type")]
#[strum_discriminants(name(NotificationType))]
#[strum_discriminants(derive(Serialize, Deserialize, Hash))]
#[strum_discriminants(serde(rename_all = "camelCase"))]
pub enum NotificationBody {
    Follow,
    FollowRequestAccepted,
    ReceiveFollowRequest,
    Mention { note: Note },
    Reply { note: Note },
    Renote { note: Note },
    Quote { note: Note },
    Reaction { note: Note, reaction: Reaction },
    PollVote { note: Note, choice: u64 },
    GroupInvited { invitation: UserGroupInvitation },
    // TODO: Implement
    App {},
}

#[derive(Debug, Display, Error, Clone)]
#[display(fmt = "invalid notification type")]
pub struct ParseNotificationTypeError;

impl std::str::FromStr for NotificationType {
    type Err = ParseNotificationTypeError;

    fn from_str(s: &str) -> Result<NotificationType, Self::Err> {
        match s {
            "follow" | "Follow" => Ok(NotificationType::Follow),
            "followRequestAccepted" | "FollowRequestAccepted" => {
                Ok(NotificationType::FollowRequestAccepted)
            }
            "receiveFollowRequest" | "ReceiveFollowRequest" => {
                Ok(NotificationType::ReceiveFollowRequest)
            }
            "mention" | "Mention" => Ok(NotificationType::Mention),
            "reply" | "Reply" => Ok(NotificationType::Reply),
            "renote" | "Renote" => Ok(NotificationType::Renote),
            "quote" | "Quote" => Ok(NotificationType::Quote),
            "reaction" | "Reaction" => Ok(NotificationType::Reaction),
            "pollVote" | "PollVote" => Ok(NotificationType::PollVote),
            "groupInvited" | "GroupInvited" => Ok(NotificationType::GroupInvited),
            "app" | "App" => Ok(NotificationType::App),
            _ => Err(ParseNotificationTypeError),
        }
    }
}
