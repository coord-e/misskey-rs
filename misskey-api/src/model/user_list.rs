use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserList {
    pub id: Id<UserList>,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub user_ids: Vec<Id<User>>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    pub is_public: bool,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(default)]
    pub liked_count: Option<u64>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(default)]
    pub is_liked: Option<bool>,
}

impl_entity!(UserList);
