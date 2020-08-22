use crate::model::{
    sort::SortOrder,
    user::{User, UserOrigin, UserState},
};

use derive_more::{Display, Error};
use serde::Serialize;

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

#[derive(PartialEq, Eq, Clone, Debug, Copy, Display)]
pub enum Sort {
    #[display(fmt = "follower")]
    Follower,
    #[display(fmt = "createdAt")]
    CreatedAt,
    #[display(fmt = "updatedAt")]
    UpdatedAt,
}

#[derive(Debug, Display, Error, Clone)]
#[display(fmt = "invalid sort key")]
pub struct ParseSortError;

impl std::str::FromStr for Sort {
    type Err = ParseSortError;

    fn from_str(s: &str) -> Result<Sort, Self::Err> {
        match s {
            "follower" | "Follower" => Ok(Sort::Follower),
            "createdAt" | "CreatedAt" => Ok(Sort::CreatedAt),
            "updatedAt" | "UpdatedAt" => Ok(Sort::UpdatedAt),
            _ => Err(ParseSortError),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortOrder<Sort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<UserState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<UserOrigin>,
}

impl misskey_core::Request for Request {
    type Response = Vec<User>;
    const ENDPOINT: &'static str = "users";
}

#[cfg(test)]
mod tests {
    use super::{Request, Sort};
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: None,
                origin: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let mut client = TestClient::new();
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
        let mut client = TestClient::new();
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
        use crate::model::sort::SortOrder;

        let mut client = TestClient::new();

        client
            .test(Request {
                limit: None,
                offset: None,
                sort: Some(SortOrder::Ascending(Sort::Follower)),
                state: None,
                origin: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: Some(SortOrder::Ascending(Sort::CreatedAt)),
                state: None,
                origin: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                sort: Some(SortOrder::Descending(Sort::UpdatedAt)),
                state: None,
                origin: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_state() {
        use crate::model::user::UserState;

        let mut client = TestClient::new();

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

        let mut client = TestClient::new();

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
