use crate::model::{id::Id, user_list::UserList};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub list_id: Id<UserList>,
    /// [ 1 .. 100 ] characters
    #[cfg(not(feature = "13-13-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-13-0"))))]
    pub name: String,
    /// [ 1 .. 100 ] characters
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_public: Option<bool>,
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
                #[cfg(not(feature = "13-13-0"))]
                name: "test2".to_string(),
                #[cfg(feature = "13-13-0")]
                name: Some("test2".to_string()),
                #[cfg(feature = "13-13-0")]
                is_public: None,
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
            .test(Request {
                list_id: list.id,
                name: None,
                is_public: Some(true),
            })
            .await;
    }
}
