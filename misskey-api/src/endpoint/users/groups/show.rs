use crate::model::user_group::{UserGroup, UserGroupId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub group_id: UserGroupId,
}

impl misskey_core::Request for Request {
    type Response = UserGroup;
    const ENDPOINT: &'static str = "users/groups/show";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let group = client
            .test(crate::endpoint::users::groups::create::Request {
                name: "test".to_string(),
            })
            .await;

        client.test(Request { group_id: group.id }).await;
    }
}
