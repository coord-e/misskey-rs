use crate::api::ApiRequest;
use crate::model::user::{User, UserId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: UserId,
}

impl ApiRequest for Request {
    type Response = User;
    const ENDPOINT: &'static str = "following/requests/cancel";
}
