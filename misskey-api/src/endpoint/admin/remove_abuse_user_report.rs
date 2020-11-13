use crate::model::{abuse_user_report::AbuseUserReport, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub report_id: Id<AbuseUserReport>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/remove-abuse-user-report";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
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
            .test(crate::endpoint::admin::abuse_user_reports::Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;

        client
            .admin
            .test(Request {
                report_id: reports[0].id.clone(),
            })
            .await;
    }
}
