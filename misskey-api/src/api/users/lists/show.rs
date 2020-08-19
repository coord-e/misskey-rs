use crate::model::user_list::{UserList, UserListId};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub list_id: UserListId,
}

impl ApiRequest for Request {
    type Response = UserList;
    const ENDPOINT: &'static str = "users/lists/show";
}
