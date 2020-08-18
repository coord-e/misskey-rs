use crate::api::ApiRequest;
use crate::model::user_list::UserList;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl ApiRequest for Request {
    type Response = Vec<UserList>;
    const ENDPOINT: &'static str = "users/lists/list";
}
