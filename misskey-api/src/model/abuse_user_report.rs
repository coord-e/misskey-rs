use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbuseUserReport {
    pub id: Id<AbuseUserReport>,
    pub created_at: DateTime<Utc>,
    pub comment: String,
    pub reporter_id: Id<User>,
    pub reporter: User,
    #[cfg(any(docsrs, not(feature = "12-49-0")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-49-0"))))]
    pub user_id: Id<User>,
    #[cfg(any(docsrs, not(feature = "12-49-0")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-49-0"))))]
    pub user: User,
    #[cfg(feature = "12-49-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-49-0")))]
    pub target_user_id: Id<User>,
    #[cfg(feature = "12-49-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-49-0")))]
    pub target_user: User,
    #[cfg(feature = "12-49-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-49-0")))]
    pub assignee_id: Option<Id<User>>,
    #[cfg(feature = "12-49-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-49-0")))]
    pub assignee: Option<User>,
    #[cfg(feature = "12-49-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-49-0")))]
    pub resolved: bool,
    #[cfg(feature = "12-102-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-102-0")))]
    pub forwarded: bool,
}

impl_entity!(AbuseUserReport);
