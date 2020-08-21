use crate::model::{
    chart::{ChartSpan, HashtagChart},
    note::Tag,
};

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

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "charts/hashtag";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        use crate::model::{chart::ChartSpan, note::Tag};

        let mut client = TestClient::new();
        client
            .test(Request {
                span: ChartSpan::Day,
                tag: Tag("tag".to_string()),
                limit: None,
                offset: None,
            })
            .await;
        client
            .test(Request {
                span: ChartSpan::Hour,
                tag: Tag("tag".to_string()),
                limit: None,
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        use crate::model::{chart::ChartSpan, note::Tag};

        let mut client = TestClient::new();
        client
            .test(Request {
                span: ChartSpan::Day,
                tag: Tag("tag".to_string()),
                limit: Some(500),
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_offset() {
        use crate::model::{chart::ChartSpan, note::Tag};

        let mut client = TestClient::new();
        client
            .test(Request {
                span: ChartSpan::Day,
                tag: Tag("tag".to_string()),
                limit: None,
                offset: Some(5),
            })
            .await;
    }
}
