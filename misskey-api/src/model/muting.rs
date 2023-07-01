use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Muting {
    pub id: Id<Muting>,
    pub created_at: DateTime<Utc>,
    #[cfg(feature = "12-108-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-108-0")))]
    pub expires_at: Option<DateTime<Utc>>,
    pub mutee_id: Id<User>,
    pub mutee: User,
}

impl_entity!(Muting);
