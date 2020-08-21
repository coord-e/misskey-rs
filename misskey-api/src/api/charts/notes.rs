use crate::model::chart::{ChartSpan, NotesChart};

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
    pub local: NotesChart,
    pub remote: NotesChart,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "charts/notes";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        use crate::model::chart::ChartSpan;

        let mut client = TestClient::new();
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

        let mut client = TestClient::new();
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

        let mut client = TestClient::new();
        client
            .test(Request {
                span: ChartSpan::Day,
                limit: None,
                offset: Some(5),
            })
            .await;
    }
}
