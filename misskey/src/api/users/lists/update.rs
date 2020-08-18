use crate::api::ApiRequest;
use crate::model::user_list::{UserList, UserListId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub list_id: UserListId,
    /// [ 1 .. 100 ] characters
    pub name: String,
}

impl ApiRequest for Request {
    type Response = UserList;
    const ENDPOINT: &'static str = "users/lists/update";
}