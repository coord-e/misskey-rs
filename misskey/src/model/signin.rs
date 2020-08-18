use std::net::IpAddr;

use crate::model::user::UserId;

use chrono::{DateTime, Utc};
use derivative::Derivative;
use derive_more::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct SigninId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Signin {
    pub user_id: UserId,
    pub success: bool,
    pub ip: IpAddr,
    pub id: SigninId,
    pub created_at: DateTime<Utc>,
}
