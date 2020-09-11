use crate::model::user::User;

use serde::Serialize;

pub mod favorites;
pub mod notifications;
pub mod pin;
pub mod read_all_messaging_messages;
pub mod read_all_unread_notes;
pub mod read_announcement;
pub mod unpin;
pub mod update;
pub mod user_group_invites;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl misskey_core::Request for Request {
    type Response = User;
    const ENDPOINT: &'static str = "i";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client.test(Request::default()).await;
    }
}
