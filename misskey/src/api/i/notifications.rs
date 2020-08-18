use std::collections::HashSet;

use crate::api::ApiRequest;
use crate::model::notification::{Notification, NotificationId, NotificationType};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<NotificationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<NotificationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_as_read: Option<bool>,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub include_types: HashSet<NotificationType>,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub exclude_types: HashSet<NotificationType>,
}

impl ApiRequest for Request {
    type Response = Vec<Notification>;
    const ENDPOINT: &'static str = "i/notifications";
}
