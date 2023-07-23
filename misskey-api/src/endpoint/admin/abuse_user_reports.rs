#[cfg(feature = "12-49-0")]
use crate::model::user::UserOrigin;
use crate::model::{abuse_user_report::AbuseUserReport, id::Id};

use serde::Serialize;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Serialize, PartialEq, Eq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum AbuseUserReportState {
    Resolved,
    Unresolved,
}

#[derive(Debug, Error, Clone)]
#[error("invalid user state")]
pub struct ParseAbuseUserReportStateError {
    _priv: (),
}

impl std::str::FromStr for AbuseUserReportState {
    type Err = ParseAbuseUserReportStateError;

    fn from_str(s: &str) -> Result<AbuseUserReportState, Self::Err> {
        match s {
            "resolved" | "Resolved" => Ok(AbuseUserReportState::Resolved),
            "unresolved" | "Unresolved" => Ok(AbuseUserReportState::Unresolved),
            _ => Err(ParseAbuseUserReportStateError { _priv: () }),
        }
    }
}

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[cfg(feature = "12-49-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-49-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<AbuseUserReportState>,
    #[cfg(feature = "12-49-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-49-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub reporter_origin: Option<UserOrigin>,
    #[cfg(feature = "12-49-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-49-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub target_user_origin: Option<UserOrigin>,
    #[cfg(feature = "12-102-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-102-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub forwarded: Option<bool>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<AbuseUserReport>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<AbuseUserReport>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<AbuseUserReport>;
    const ENDPOINT: &'static str = "admin/abuse-user-reports";
}

impl_pagination!(Request, AbuseUserReport);

#[cfg(test)]
mod tests {
    #[cfg(feature = "12-49-0")]
    use super::AbuseUserReportState;
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
                #[cfg(feature = "12-49-0")]
                state: None,
                #[cfg(feature = "12-49-0")]
                reporter_origin: None,
                #[cfg(feature = "12-49-0")]
                target_user_origin: None,
                #[cfg(feature = "12-102-0")]
                forwarded: None,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        #[cfg(feature = "12-49-0")]
        use crate::model::user::UserOrigin;

        let client = TestClient::new();

        client
            .admin
            .test(Request {
                #[cfg(feature = "12-49-0")]
                state: Some(AbuseUserReportState::Unresolved),
                #[cfg(feature = "12-49-0")]
                reporter_origin: Some(UserOrigin::Remote),
                #[cfg(feature = "12-49-0")]
                target_user_origin: Some(UserOrigin::Combined),
                #[cfg(feature = "12-102-0")]
                forwarded: None,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;

        client
            .admin
            .test(Request {
                #[cfg(feature = "12-49-0")]
                state: Some(AbuseUserReportState::Resolved),
                #[cfg(feature = "12-49-0")]
                reporter_origin: Some(UserOrigin::Combined),
                #[cfg(feature = "12-49-0")]
                target_user_origin: Some(UserOrigin::Local),
                #[cfg(feature = "12-102-0")]
                forwarded: None,
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
                #[cfg(feature = "12-49-0")]
                state: None,
                #[cfg(feature = "12-49-0")]
                reporter_origin: None,
                #[cfg(feature = "12-49-0")]
                target_user_origin: None,
                #[cfg(feature = "12-102-0")]
                forwarded: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;

        client
            .admin
            .test(Request {
                #[cfg(feature = "12-49-0")]
                state: None,
                #[cfg(feature = "12-49-0")]
                reporter_origin: None,
                #[cfg(feature = "12-49-0")]
                target_user_origin: None,
                #[cfg(feature = "12-102-0")]
                forwarded: None,
                limit: None,
                since_id: Some(reports[0].id.clone()),
                until_id: Some(reports[0].id.clone()),
            })
            .await;
    }
}
