use crate::api::ApiRequest;
use crate::model::{
    chart::{ChartSpan, ReactionsChart},
    user::UserId,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub span: ChartSpan,
    pub user_id: UserId,
    /// 1 .. 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub local: ReactionsChart,
    pub remote: ReactionsChart,
}

impl ApiRequest for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "charts/user/reactions";
}
