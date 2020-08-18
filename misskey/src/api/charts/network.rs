use crate::api::ApiRequest;
use crate::model::chart::{ChartSpan, NetworkChart};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub span: ChartSpan,
    /// 1 .. 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl ApiRequest for Request {
    type Response = NetworkChart;
    const ENDPOINT: &'static str = "charts/network";
}
