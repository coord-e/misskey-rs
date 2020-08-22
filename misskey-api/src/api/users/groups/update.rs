use crate::model::user_group::{UserGroup, UserGroupId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub group_id: UserGroupId,
    /// [ 1 .. 100 ] characters
    pub name: String,
}

impl misskey_core::Request for Request {
    type Response = UserGroup;
    const ENDPOINT: &'static str = "users/groups/update";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let group = client
            .test(crate::api::users::groups::create::Request {
                name: "test".to_string(),
            })
            .await;

        client
            .test(Request {
                group_id: group.id,
                name: "test2".to_string(),
            })
            .await;
    }
}
