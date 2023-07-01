use crate::model::user_list::UserList;
#[cfg(feature = "13-13-0")]
use crate::model::{id::Id, user::User};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub user_id: Option<Id<User>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<UserList>;
    const ENDPOINT: &'static str = "users/lists/list";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client
            .test(crate::endpoint::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;

        client.test(Request::default()).await;
    }

    #[cfg(feature = "13-13-0")]
    #[tokio::test]
    async fn request_with_option() {
        let client = TestClient::new();
        let user = client.me().await;
        client
            .test(crate::endpoint::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;

        client
            .test(Request {
                user_id: Some(user.id),
            })
            .await;
    }
}
