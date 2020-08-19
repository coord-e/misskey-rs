use crate::model::{user::UserId, user_list::UserListId};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub list_id: UserListId,
    pub user_id: UserId,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "users/lists/push";
}
