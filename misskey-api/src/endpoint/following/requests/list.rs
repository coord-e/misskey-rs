use crate::model::following::FollowRequest;

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl misskey_core::Request for Request {
    type Response = Vec<FollowRequest>;
    const ENDPOINT: &'static str = "following/requests/list";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let (new_user, new_client) = client.admin.create_user().await;

        new_client
            .test(crate::endpoint::i::update::Request {
                name: None,
                description: None,
                lang: None,
                location: None,
                birthday: None,
                avatar_id: None,
                banner_id: None,
                fields: None,
                is_locked: Some(true),
                #[cfg(feature = "12-63-0")]
                is_explorable: None,
                careful_bot: None,
                auto_accept_followed: None,
                is_bot: None,
                is_cat: None,
                #[cfg(not(feature = "12-55-0"))]
                auto_watch: None,
                inject_featured_note: None,
                always_mark_nsfw: None,
                pinned_page_id: None,
                muted_words: None,
                #[cfg(feature = "12-60-0")]
                no_crawle: None,
            })
            .await;
        client
            .user
            .test(crate::endpoint::following::create::Request {
                user_id: new_user.id,
            })
            .await;

        new_client.test(Request::default()).await;
    }
}
