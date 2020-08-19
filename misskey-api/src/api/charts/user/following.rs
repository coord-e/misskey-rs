use crate::model::{
    chart::{ChartSpan, FollowersChart, FollowingChart},
    user::UserId,
};

use misskey_core::ApiRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub span: ChartSpan,
    pub user_id: UserId,
    /// 1 .. 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub local: FollowFollowerChart,
    pub remote: FollowFollowerChart,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowFollowerChart {
    pub followings: FollowingChart,
    pub followers: FollowersChart,
}

impl ApiRequest for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "charts/user/following";
}
