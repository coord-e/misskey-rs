#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
use crate::model::{
    sort::SortOrder,
    user::{User, UserOrigin, UserSortKey},
};

#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
use serde::Serialize;
#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
use thiserror::Error;
#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
use typed_builder::TypedBuilder;

pub mod followers;
pub mod following;
pub mod get_frequently_replied_users;
pub mod groups;
pub mod lists;
pub mod notes;
pub mod relation;
pub mod report_abuse;
pub mod search;
pub mod search_by_username_and_host;
pub mod show;

#[cfg(feature = "12-60-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-60-0")))]
pub mod stats;

#[cfg(feature = "12-61-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-61-0")))]
pub mod clips;

#[cfg(feature = "12-61-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-61-0")))]
pub mod pages;

#[cfg(feature = "12-79-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-79-0")))]
pub mod gallery;

// misskey-dev/misskey#7656
#[cfg(not(feature = "12-88-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-88-0"))))]
pub mod recommendation;

#[cfg(feature = "12-93-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-93-0")))]
pub mod reactions;

#[cfg(feature = "13-1-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-1-0")))]
pub mod achievements;

#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
#[derive(Serialize, PartialEq, Eq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum UserState {
    All,
    Alive,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    Admin,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    Moderator,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    AdminOrModerator,
}

#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
#[derive(Debug, Error, Clone)]
#[error("invalid user state")]
pub struct ParseUserStateError {
    _priv: (),
}

#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
impl std::str::FromStr for UserState {
    type Err = ParseUserStateError;

    fn from_str(s: &str) -> Result<UserState, Self::Err> {
        match s {
            "all" | "All" => Ok(UserState::All),
            "alive" | "Alive" => Ok(UserState::Alive),
            #[cfg(not(feature = "13-0-0"))]
            "admin" | "Admin" => Ok(UserState::Admin),
            #[cfg(not(feature = "13-0-0"))]
            "moderator" | "Moderator" => Ok(UserState::Moderator),
            #[cfg(not(feature = "13-0-0"))]
            "adminOrModerator" | "AdminOrModerator" => Ok(UserState::AdminOrModerator),
            _ => Err(ParseUserStateError { _priv: () }),
        }
    }
}

// misskey-dev/misskey#7656
#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
#[cfg_attr(docsrs, doc(cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))))]
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
    pub sort: Option<SortOrder<UserSortKey>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<UserState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub origin: Option<UserOrigin>,
    #[cfg(feature = "12-112-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-112-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub hostname: Option<String>,
}

#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
impl misskey_core::Request for Request {
    type Response = Vec<User>;
    const ENDPOINT: &'static str = "users";
}

#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
impl_offset_pagination!(Request, User);

#[cfg(any(not(feature = "12-88-0"), feature = "12-89-0"))]
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
                #[cfg(feature = "12-112-0")]
                hostname: None,
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
                #[cfg(feature = "12-112-0")]
                hostname: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_sort() {
        use crate::model::{sort::SortOrder, user::UserSortKey};

        let client = TestClient::new();

        client
            .test(Request {
                limit: None,
                offset: None,
                sort: Some(SortOrder::Ascending(UserSortKey::Follower)),
                state: None,
                origin: None,
                #[cfg(feature = "12-112-0")]
                hostname: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: Some(SortOrder::Ascending(UserSortKey::CreatedAt)),
                state: None,
                origin: None,
                #[cfg(feature = "12-112-0")]
                hostname: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: Some(SortOrder::Descending(UserSortKey::UpdatedAt)),
                state: None,
                origin: None,
                #[cfg(feature = "12-112-0")]
                hostname: None,
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
                #[cfg(feature = "12-112-0")]
                hostname: None,
            })
            .await;
        #[cfg(not(feature = "13-0-0"))]
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: Some(UserState::Admin),
                origin: None,
                #[cfg(feature = "12-112-0")]
                hostname: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: Some(UserState::Alive),
                origin: None,
                #[cfg(feature = "12-112-0")]
                hostname: None,
            })
            .await;
        #[cfg(not(feature = "13-0-0"))]
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: Some(UserState::Moderator),
                origin: None,
                #[cfg(feature = "12-112-0")]
                hostname: None,
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
                #[cfg(feature = "12-112-0")]
                hostname: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: None,
                origin: Some(UserOrigin::Remote),
                #[cfg(feature = "12-112-0")]
                hostname: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: None,
                origin: Some(UserOrigin::Combined),
                #[cfg(feature = "12-112-0")]
                hostname: None,
            })
            .await;
    }

    #[cfg(feature = "12-112-0")]
    #[tokio::test]
    async fn request_with_hostname() {
        let client = TestClient::new();

        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: None,
                origin: None,
                hostname: Some("host".to_string()),
            })
            .await;
    }
}
