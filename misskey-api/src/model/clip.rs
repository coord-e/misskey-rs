use crate::model::id::Id;
#[cfg(feature = "12-57-0")]
use crate::model::user::User;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub id: Id<Clip>,
    pub created_at: DateTime<Utc>,
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub last_clipped_at: Option<DateTime<Utc>>,
    pub name: String,
    #[cfg(feature = "12-57-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
    pub user_id: Id<User>,
    #[cfg(feature = "12-57-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
    pub user: User,
    #[cfg(feature = "12-57-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
    pub description: Option<String>,
    #[cfg(feature = "12-57-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-57-0")))]
    pub is_public: bool,
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub favorited_count: u64,
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub is_favorited: Option<bool>,
}

impl_entity!(Clip);
