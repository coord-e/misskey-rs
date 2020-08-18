use crate::api::ApiRequest;
use crate::model::{user::UserId, user_group::UserGroupId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub group_id: UserGroupId,
    pub user_id: UserId,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "users/groups/pull";
}
