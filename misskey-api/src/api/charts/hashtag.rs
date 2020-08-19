use crate::model::{
    chart::{ChartSpan, HashtagChart},
    note::Tag,
};

use misskey_core::ApiRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub span: ChartSpan,
    pub tag: Tag,
    /// 1 .. 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub local: HashtagChart,
    pub remote: HashtagChart,
}

impl ApiRequest for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "charts/hashtag";
}
