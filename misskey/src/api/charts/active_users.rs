use crate::api::ApiRequest;
use crate::model::chart::{ActiveUsersChart, ChartSpan};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub span: ChartSpan,
    /// 1 .. 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub local: ActiveUsersChart,
    pub remote: ActiveUsersChart,
}

impl ApiRequest for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "charts/active-users";
}
