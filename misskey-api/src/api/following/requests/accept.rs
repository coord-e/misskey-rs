use crate::model::user::UserId;

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: UserId,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "following/requests/accept";
}
