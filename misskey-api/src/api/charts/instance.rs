use crate::model::chart::{
    ChartSpan, DriveChart, FollowersChart, FollowingChart, NotesChart, RequestsChart, UsersChart,
};

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

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "charts/instance";
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
                // TODO: use proper host string
                host: "localhost:3000".to_string(),
                limit: None,
                offset: None,
            })
            .await;
        client
            .test(Request {
                span: ChartSpan::Hour,
                host: "localhost:3000".to_string(),
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
                host: "localhost:3000".to_string(),
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
                host: "localhost:3000".to_string(),
                limit: None,
                offset: Some(5),
            })
            .await;
    }
}
