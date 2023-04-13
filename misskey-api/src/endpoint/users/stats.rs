use crate::model::{id::Id, user::User};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserStats {
    pub notes_count: u64,
    pub replies_count: u64,
    pub renotes_count: u64,
    pub replied_count: u64,
    pub renoted_count: u64,
    pub poll_votes_count: u64,
    pub poll_voted_count: u64,
    pub local_following_count: u64,
    pub remote_following_count: u64,
    pub local_followers_count: u64,
    pub remote_followers_count: u64,
    pub following_count: u64,
    pub followers_count: u64,
    pub sent_reactions_count: u64,
    pub received_reactions_count: u64,
    pub note_favorites_count: u64,
    pub page_likes_count: u64,
    pub page_liked_count: u64,
    pub drive_files_count: u64,
    pub drive_usage: u64,
    #[cfg(not(feature = "12-102-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-102-0"))))]
    pub reversi_count: u64,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: Id<User>,
}

impl misskey_core::Request for Request {
    type Response = UserStats;
    const ENDPOINT: &'static str = "users/stats";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_simple() {
        let client = TestClient::new();
        let user = client.admin.me().await;
        client.user.test(Request { user_id: user.id }).await;
    }
}
