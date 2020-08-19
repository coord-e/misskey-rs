use crate::model::user_group::{UserGroup, UserGroupId};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub group_id: UserGroupId,
    /// [ 1 .. 100 ] characters
    pub name: String,
}

impl ApiRequest for Request {
    type Response = UserGroup;
    const ENDPOINT: &'static str = "users/groups/update";
}
