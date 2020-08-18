use crate::api::ApiRequest;
use crate::model::user_list::UserListId;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub list_id: UserListId,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "users/lists/delete";
}
