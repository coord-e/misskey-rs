use crate::model::abuse_user_report::{AbuseUserReport, AbuseUserReportId};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<AbuseUserReportId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<AbuseUserReportId>,
}

impl misskey_core::Request for Request {
    type Response = Vec<AbuseUserReport>;
    const ENDPOINT: &'static str = "admin/abuse-user-reports";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.admin.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();

        client
            .admin
            .test(Request {
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let (user, _) = client.admin.create_user().await;

        client
            .user
            .test(crate::endpoint::users::report_abuse::Request {
                user_id: user.id.clone(),
                comment: "damedesu".to_string(),
            })
            .await;

        let reports = client
            .admin
            .test(Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;

        client
            .admin
            .test(Request {
                limit: None,
                since_id: Some(reports[0].id.clone()),
                until_id: Some(reports[0].id.clone()),
            })
            .await;
    }
}
