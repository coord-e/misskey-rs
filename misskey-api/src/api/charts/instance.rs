use crate::model::chart::{
    ChartSpan, DriveChart, FollowersChart, FollowingChart, NotesChart, RequestsChart, UsersChart,
};

use misskey_core::ApiRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub span: ChartSpan,
    pub host: String,
    /// 1 .. 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub requests: RequestsChart,
    pub notes: NotesChart,
    pub users: UsersChart,
    pub following: FollowingChart,
    pub followers: FollowersChart,
    pub drive: DriveChart,
}

impl ApiRequest for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "charts/instance";
}
