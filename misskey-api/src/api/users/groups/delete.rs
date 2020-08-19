use crate::model::user_group::UserGroupId;

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub group_id: UserGroupId,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "users/groups/delete";
}
