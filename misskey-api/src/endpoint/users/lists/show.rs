use crate::model::{id::Id, user_list::UserList};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub list_id: Id<UserList>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub for_public: Option<bool>,
}

impl misskey_core::Request for Request {
    type Response = UserList;
    const ENDPOINT: &'static str = "users/lists/show";
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
                #[cfg(feature = "13-13-0")]
                for_public: None,
            })
            .await;
    }

    #[cfg(feature = "13-13-0")]
    #[tokio::test]
    async fn request_with_option() {
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

        client
            .test(Request {
                list_id: list.id,
                for_public: Some(true),
            })
            .await;
    }
}
