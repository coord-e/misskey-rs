use crate::model::{
    sort::SortOrder,
    user::{User, UserOrigin, UserSort},
};

use serde::Serialize;
use thiserror::Error;
use typed_builder::TypedBuilder;

pub mod followers;
pub mod following;
pub mod get_frequently_replied_users;
pub mod groups;
pub mod lists;
pub mod notes;
pub mod recommendation;
pub mod relation;
pub mod report_abuse;
pub mod search;
pub mod search_by_username_and_host;
pub mod show;

#[cfg(feature = "12-60-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-60-0")))]
pub mod stats;

#[derive(Serialize, PartialEq, Eq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum UserState {
    All,
    Alive,
    Admin,
    Moderator,
    AdminOrModerator,
}

#[derive(Debug, Error, Clone)]
#[error("invalid user state")]
pub struct ParseUserStateError {
    _priv: (),
}

impl std::str::FromStr for UserState {
    type Err = ParseUserStateError;

    fn from_str(s: &str) -> Result<UserState, Self::Err> {
        match s {
            "all" | "All" => Ok(UserState::All),
            "alive" | "Alive" => Ok(UserState::Alive),
            "admin" | "Admin" => Ok(UserState::Admin),
            "moderator" | "Moderator" => Ok(UserState::Moderator),
            "adminOrModerator" | "AdminOrModerator" => Ok(UserState::AdminOrModerator),
            _ => Err(ParseUserStateError { _priv: () }),
        }
    }
}

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<SortOrder<UserSort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<UserState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub origin: Option<UserOrigin>,
}

impl misskey_core::Request for Request {
    type Response = Vec<User>;
    const ENDPOINT: &'static str = "users";
}

impl_offset_pagination!(Request, User);

#[cfg(test)]
mod tests {
    use super::{Request, UserState};
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .test(Request {
                limit: Some(100),
                offset: None,
                sort: None,
                state: None,
                origin: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_offset() {
        let client = TestClient::new();
        client
            .test(Request {
                limit: None,
                offset: Some(5),
                sort: None,
                state: None,
                origin: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_sort() {
        use crate::model::{sort::SortOrder, user::UserSort};

        let client = TestClient::new();

        client
            .test(Request {
                limit: None,
                offset: None,
                sort: Some(SortOrder::Ascending(UserSort::Follower)),
                state: None,
                origin: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: Some(SortOrder::Ascending(UserSort::CreatedAt)),
                state: None,
                origin: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: Some(SortOrder::Descending(UserSort::UpdatedAt)),
                state: None,
                origin: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_state() {
        let client = TestClient::new();

        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: Some(UserState::All),
                origin: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: Some(UserState::Admin),
                origin: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: Some(UserState::Alive),
                origin: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: Some(UserState::Moderator),
                origin: None,
            })
            .await;
        // TODO: Uncomment with cfg when `adminOrModerator` value is fixed in Misskey
        // client
        //     .test(Request {
        //         limit: None,
        //         offset: None,
        //         sort: None,
        //         state: Some(UserState::AdminOrModerator),
        //         origin: None,
        //     })
        //     .await;
    }

    #[tokio::test]
    async fn request_with_origin() {
        use crate::model::user::UserOrigin;

        let client = TestClient::new();

        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: None,
                origin: Some(UserOrigin::Local),
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: None,
                origin: Some(UserOrigin::Remote),
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: None,
                origin: Some(UserOrigin::Combined),
            })
            .await;
    }
}
