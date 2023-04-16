use std::{collections::HashMap, net::IpAddr};

use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Signin {
    pub user_id: Id<User>,
    pub success: bool,
    pub ip: IpAddr,
    pub id: Id<Signin>,
    pub created_at: DateTime<Utc>,
    pub user: Option<User>,
    pub headers: HashMap<String, serde_json::Value>,
}

impl_entity!(Signin);
