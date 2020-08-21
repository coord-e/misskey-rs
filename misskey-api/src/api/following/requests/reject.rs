use crate::model::user::UserId;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: UserId,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "following/requests/reject";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let (new_user, mut new_client) = client.admin.create_user().await;

        new_client
            .test(crate::api::i::update::Request {
                name: None,
                description: None,
                lang: None,
                location: None,
                birthday: None,
                avatar_id: None,
                banner_id: None,
                fields: None,
                is_locked: Some(true),
                careful_bot: None,
                auto_accept_followed: Some(false),
                is_bot: None,
                is_cat: None,
                auto_watch: None,
                inject_featured_note: None,
                always_mark_nsfw: None,
                pinned_page_id: None,
                muted_words: None,
            })
            .await;
        client
            .user
            .test(crate::api::following::create::Request {
                user_id: new_user.id,
            })
            .await;

        let user = client.user.me().await;
        new_client.test(Request { user_id: user.id }).await;
    }
}
