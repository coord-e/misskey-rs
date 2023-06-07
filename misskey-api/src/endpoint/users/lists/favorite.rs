use crate::model::{id::Id, user_list::UserList};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub list_id: Id<UserList>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "users/lists/favorite";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let list = client
            .test(crate::endpoint::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;
        client
            .test(
                crate::endpoint::users::lists::update::Request::builder()
                    .list_id(list.id)
                    .is_public(true)
                    .build(),
            )
            .await;

        client.test(Request { list_id: list.id }).await;
    }
}
