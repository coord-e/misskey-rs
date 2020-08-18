use crate::api::ApiRequest;
use crate::model::user_group::UserGroup;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// [ 1 .. 100 ] characters
    pub name: String,
}

impl ApiRequest for Request {
    type Response = UserGroup;
    const ENDPOINT: &'static str = "users/groups/create";
}
