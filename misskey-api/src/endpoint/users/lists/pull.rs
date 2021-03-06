use crate::model::{id::Id, user::User, user_list::UserList};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub list_id: Id<UserList>,
    pub user_id: Id<User>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "users/lists/pull";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let admin = client.admin.me().await;
        let list = client
            .user
            .test(crate::endpoint::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;
        client
            .user
            .test(crate::endpoint::users::lists::push::Request {
                list_id: list.id.clone(),
                user_id: admin.id.clone(),
            })
            .await;

        client
            .user
            .test(Request {
                list_id: list.id,
                user_id: admin.id,
            })
            .await;
    }
}
