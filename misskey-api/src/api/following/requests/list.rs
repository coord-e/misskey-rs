use crate::model::user::FollowRequest;

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl ApiRequest for Request {
    type Response = Vec<FollowRequest>;
    const ENDPOINT: &'static str = "following/requests/list";
}
