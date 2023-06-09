use crate::model::chart::{ChartSpan, FederationChart};

#[cfg(not(feature = "12-104-0"))]
use serde::Deserialize;
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

#[cfg(not(feature = "12-104-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-104-0"))))]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub instance: FederationChart,
}

impl misskey_core::Request for Request {
    #[cfg(not(feature = "12-104-0"))]
    type Response = Response;
    #[cfg(feature = "12-104-0")]
    type Response = FederationChart;
    const ENDPOINT: &'static str = "charts/federation";
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
