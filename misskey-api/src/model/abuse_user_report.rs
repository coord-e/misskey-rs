use crate::model::user::{User, UserId};

use chrono::{DateTime, Utc};
use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct AbuseUserReportId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbuseUserReport {
    pub id: AbuseUserReportId,
    pub created_at: DateTime<Utc>,
    pub comment: String,
    pub reporter_id: UserId,
    pub reporter: User,
    pub user_id: UserId,
    pub user: User,
}
