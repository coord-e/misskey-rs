use crate::model::user_list::UserList;

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// [ 1 .. 100 ] characters
    pub name: String,
}

impl ApiRequest for Request {
    type Response = UserList;
    const ENDPOINT: &'static str = "users/lists/create";
}
