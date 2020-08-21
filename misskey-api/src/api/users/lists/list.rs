use crate::model::user_list::UserList;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl misskey_core::Request for Request {
    type Response = Vec<UserList>;
    const ENDPOINT: &'static str = "users/lists/list";
}
