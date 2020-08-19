use crate::model::user_list::UserList;

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl ApiRequest for Request {
    type Response = Vec<UserList>;
    const ENDPOINT: &'static str = "users/lists/list";
}
