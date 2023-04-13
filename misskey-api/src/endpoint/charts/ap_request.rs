use crate::model::chart::{ApRequestChart, ChartSpan};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub span: ChartSpan,
    /// 1 .. 500
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u64>,
    #[builder(default, setter(strip_option))]
    pub offset: Option<u64>,
}

impl misskey_core::Request for Request {
    type Response = ApRequestChart;
    const ENDPOINT: &'static str = "charts/ap-request";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        use crate::model::chart::ChartSpan;

        let client = TestClient::new();
        client
            .test(Request {
                span: ChartSpan::Day,
                limit: None,
                offset: None,
            })
            .await;
        client
            .test(Request {
                span: ChartSpan::Hour,
                limit: None,
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        use crate::model::chart::ChartSpan;

        let client = TestClient::new();
        client
            .test(Request {
                span: ChartSpan::Day,
                limit: Some(500),
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_offset() {
        use crate::model::chart::ChartSpan;

        let client = TestClient::new();
        client
            .test(Request {
                span: ChartSpan::Day,
                limit: None,
                offset: Some(5),
            })
            .await;
    }
}
