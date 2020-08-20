use crate::model::{
    drive::DriveFileId, messaging::MessagingMessage, user::UserId, user_group::UserGroupId,
};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<UserGroupId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<DriveFileId>,
}

impl ApiRequest for Request {
    type Response = MessagingMessage;
    const ENDPOINT: &'static str = "messaging/messages/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_with_text() {
        let mut client = TestClient::new();
        let admin = client.admin.me().await;
        client
            .user
            .test(Request {
                text: Some("hi".to_string()),
                user_id: Some(admin.id),
                group_id: None,
                file_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_group() {
        let mut client = TestClient::new();
        let group = client
            .test(crate::api::users::groups::create::Request {
                name: "test".to_string(),
            })
            .await;
        client
            .test(Request {
                text: Some("hi".to_string()),
                user_id: None,
                group_id: Some(group.id),
                file_id: None,
            })
            .await;
    }

    // TODO: request_with_file
}
