use crate::api::ApiRequest;
use crate::model::user::FollowRequest;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl ApiRequest for Request {
    type Response = Vec<FollowRequest>;
    const ENDPOINT: &'static str = "following/requests/list";
}
