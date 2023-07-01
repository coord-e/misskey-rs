use crate::model::{abuse_user_report::AbuseUserReport, id::Id};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub report_id: Id<AbuseUserReport>,
    #[cfg(feature = "12-102-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-102-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub forward: Option<bool>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/resolve-abuse-user-report";
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
            .test(crate::endpoint::admin::abuse_user_reports::Request::default())
            .await;

        client
            .admin
            .test(Request {
                report_id: reports[0].id.clone(),
                #[cfg(feature = "12-102-0")]
                forward: None,
            })
            .await;
    }

    #[cfg(feature = "12-102-0")]
    #[tokio::test]
    async fn request_with_forward() {
        let client = TestClient::new();
        let (user, _) = client.admin.create_user().await;

        client
            .user
            .test(crate::endpoint::users::report_abuse::Request {
                user_id: user.id.clone(),
                comment: "damedesu".to_string(),
            })
            .await;

        #[cfg(feature = "12-102-0")]
        let reports = client
            .admin
            .test(crate::endpoint::admin::abuse_user_reports::Request::default())
            .await;

        client
            .admin
            .test(Request {
                report_id: reports[0].id.clone(),
                #[cfg(feature = "12-102-0")]
                forward: Some(true),
            })
            .await;
    }
}
