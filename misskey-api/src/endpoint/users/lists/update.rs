use crate::model::{id::Id, user_list::UserList};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub list_id: Id<UserList>,
    /// [ 1 .. 100 ] characters
    pub name: String,
}

impl misskey_core::Request for Request {
    type Response = UserList;
    const ENDPOINT: &'static str = "users/lists/update";
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
            .test(Request {
                list_id: list.id,
                name: "test2".to_string(),
            })
            .await;
    }
}
