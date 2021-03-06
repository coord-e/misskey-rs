use crate::model::{
    drive::DriveFile, id::Id, messaging::MessagingMessage, user::User, user_group::UserGroup,
};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub user_id: Option<Id<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub group_id: Option<Id<UserGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub file_id: Option<Id<DriveFile>>,
}

impl misskey_core::Request for Request {
    type Response = MessagingMessage;
    const ENDPOINT: &'static str = "messaging/messages/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request_with_text() {
        let client = TestClient::new();
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
        let client = TestClient::new();
        let group = client
            .test(crate::endpoint::users::groups::create::Request {
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

    #[tokio::test]
    async fn request_with_file() {
        let client = TestClient::new();
        let admin = client.admin.me().await;
        let file = client.create_text_file("test.txt", "hello").await;

        client
            .user
            .test(Request {
                text: Some("hi".to_string()),
                user_id: Some(admin.id),
                group_id: None,
                file_id: Some(file.id),
            })
            .await;
    }
}
