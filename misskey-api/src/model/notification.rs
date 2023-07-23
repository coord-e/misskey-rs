#[cfg(not(feature = "13-7-0"))]
use crate::model::user_group::UserGroupInvitation;
use crate::model::{
    id::Id,
    note::{Note, Reaction},
    user::User,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumDiscriminants;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: Id<Notification>,
    pub created_at: DateTime<Utc>,
    /// This field is [`Id<User>`] (i.e. not [`Option`]) on <span class="module-item stab portability" style="display: inline-block; font-size: 80%;"><strong>non-<code style="background-color: transparent;">feature="12-17-0"</code></strong></span>.
    #[cfg(feature = "12-27-0")]
    pub user_id: Option<Id<User>>,
    /// This field is [`User`] (i.e. not [`Option`]) on <span class="module-item stab portability" style="display: inline-block; font-size: 80%;"><strong>non-<code style="background-color: transparent;">feature="12-17-0"</code></strong></span>.
    #[cfg(feature = "12-27-0")]
    pub user: Option<User>,
    #[cfg(not(feature = "12-27-0"))]
    pub user_id: Id<User>,
    #[cfg(not(feature = "12-27-0"))]
    pub user: User,
    #[cfg(feature = "12-39-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-39-0")))]
    pub is_read: bool,
    #[serde(flatten)]
    pub body: NotificationBody,
}

impl_entity!(Notification);

#[derive(Serialize, Deserialize, Debug, Clone, EnumDiscriminants)]
#[serde(rename_all = "camelCase", tag = "type")]
#[strum_discriminants(name(NotificationType))]
#[strum_discriminants(derive(Serialize, Deserialize, Hash))]
#[strum_discriminants(serde(rename_all = "camelCase"))]
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
    #[cfg(feature = "12-108-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-108-0")))]
    PollEnded {
        note: Note,
    },
    #[cfg(not(feature = "13-7-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-7-0"))))]
    GroupInvited {
        invitation: UserGroupInvitation,
    },
    #[cfg(feature = "13-1-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-1-0")))]
    AchievementEarned {
        achievement: Option<String>,
    },
    App {
        body: Option<String>,
        header: Option<String>,
        icon: Option<String>,
    },
}

#[derive(Debug, Error, Clone)]
#[error("invalid notification type")]
pub struct ParseNotificationTypeError {
    _priv: (),
}

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
            #[cfg(feature = "12-108-0")]
            "pollEnded" | "PollEnded" => Ok(NotificationType::PollEnded),
            #[cfg(not(feature = "13-7-0"))]
            "groupInvited" | "GroupInvited" => Ok(NotificationType::GroupInvited),
            "app" | "App" => Ok(NotificationType::App),
            _ => Err(ParseNotificationTypeError { _priv: () }),
        }
    }
}
