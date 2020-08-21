use crate::model::{
    chart::{ChartSpan, NotesChart},
    user::UserId,
};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub span: ChartSpan,
    pub user_id: UserId,
    /// 1 .. 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl misskey_core::Request for Request {
    type Response = NotesChart;
    const ENDPOINT: &'static str = "charts/user/notes";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        use crate::model::chart::ChartSpan;

        let mut client = TestClient::new();
        let user = client.user.me().await;

        client
            .test(Request {
                span: ChartSpan::Day,
                user_id: user.id.clone(),
                limit: None,
                offset: None,
            })
            .await;
        client
            .test(Request {
                span: ChartSpan::Hour,
                user_id: user.id.clone(),
                limit: None,
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        use crate::model::chart::ChartSpan;

        let mut client = TestClient::new();
        let user = client.user.me().await;

        client
            .test(Request {
                span: ChartSpan::Day,
                user_id: user.id,
                limit: Some(500),
                offset: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_offset() {
        use crate::model::chart::ChartSpan;

        let mut client = TestClient::new();
        let user = client.user.me().await;

        client
            .test(Request {
                span: ChartSpan::Day,
                user_id: user.id,
                limit: None,
                offset: Some(5),
            })
            .await;
    }
}
