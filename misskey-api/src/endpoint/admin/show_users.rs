use crate::model::{
    sort::SortOrder,
    user::{User, UserOrigin, UserSort},
};

use derive_more::{Display, Error};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, PartialEq, Eq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum UserState {
    All,
    Available,
    // Alive,
    Admin,
    Moderator,
    AdminOrModerator,
    Silenced,
    Suspended,
}

#[derive(Debug, Display, Error, Clone)]
#[display(fmt = "invalid user state")]
pub struct ParseUserStateError;

impl std::str::FromStr for UserState {
    type Err = ParseUserStateError;

    fn from_str(s: &str) -> Result<UserState, Self::Err> {
        match s {
            "all" | "All" => Ok(UserState::All),
            // "alive" | "Alive" => Ok(UserState::Alive),
            "available" | "Available" => Ok(UserState::Available),
            "admin" | "Admin" => Ok(UserState::Admin),
            "moderator" | "Moderator" => Ok(UserState::Moderator),
            "adminOrModerator" | "AdminOrModerator" => Ok(UserState::AdminOrModerator),
            "silenced" | "Silenced" => Ok(UserState::Silenced),
            "suspended" | "Suspended" => Ok(UserState::Suspended),
            _ => Err(ParseUserStateError),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub hostname: Option<String>,
}

impl misskey_core::Request for Request {
    type Response = Vec<User>;
    const ENDPOINT: &'static str = "admin/show-users";
}

impl_offset_pagination!(Request, User);

#[cfg(test)]
mod tests {
    use super::{Request, UserState};
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.admin.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .admin
            .test(Request {
                limit: Some(100),
                offset: None,
                sort: None,
                state: None,
                origin: None,
                username: None,
                hostname: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_offset() {
        let client = TestClient::new();
        client
            .admin
            .test(Request {
                limit: None,
                offset: Some(5),
                sort: None,
                state: None,
                origin: None,
                username: None,
                hostname: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_sort() {
        use crate::model::{sort::SortOrder, user::UserSort};

        let client = TestClient::new();

        client
            .admin
            .test(Request {
                limit: None,
                offset: None,
                sort: Some(SortOrder::Ascending(UserSort::Follower)),
                state: None,
                origin: None,
                username: None,
                hostname: None,
            })
            .await;
        client
            .admin
            .test(Request {
                limit: None,
                offset: None,
                sort: Some(SortOrder::Ascending(UserSort::CreatedAt)),
                state: None,
                origin: None,
                username: None,
                hostname: None,
            })
            .await;
        client
            .admin
            .test(Request {
                limit: None,
                offset: None,
                sort: Some(SortOrder::Descending(UserSort::UpdatedAt)),
                state: None,
                origin: None,
                username: None,
                hostname: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_state() {
        let client = TestClient::new();

        client
            .admin
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: Some(UserState::All),
                origin: None,
                username: None,
                hostname: None,
            })
            .await;
        client
            .admin
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: Some(UserState::Admin),
                origin: None,
                username: None,
                hostname: None,
            })
            .await;
        client
            .admin
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: Some(UserState::Available),
                origin: None,
                username: None,
                hostname: None,
            })
            .await;
        // client
        //     .admin
        //     .test(Request {
        //         limit: None,
        //         offset: None,
        //         sort: None,
        //         state: Some(UserState::Alive),
        //         origin: None,
        //         username: None,
        //         hostname: None,
        //     })
        //     .await;
        client
            .admin
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: Some(UserState::Moderator),
                origin: None,
                username: None,
                hostname: None,
            })
            .await;
        // TODO: Uncomment with cfg when `adminOrModerator` value is fixed in Misskey
        // client
        //     .admin
        //     .test(Request {
        //         limit: None,
        //         offset: None,
        //         sort: None,
        //         state: Some(UserState::AdminOrModerator),
        //         origin: None,
        //     })
        //     .await;
        client
            .admin
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: Some(UserState::Silenced),
                origin: None,
                username: None,
                hostname: None,
            })
            .await;
        client
            .admin
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: Some(UserState::Suspended),
                origin: None,
                username: None,
                hostname: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_origin() {
        use crate::model::user::UserOrigin;

        let client = TestClient::new();

        client
            .admin
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: None,
                origin: Some(UserOrigin::Local),
                username: None,
                hostname: None,
            })
            .await;
        client
            .admin
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: None,
                origin: Some(UserOrigin::Remote),
                username: None,
                hostname: None,
            })
            .await;
        client
            .admin
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: None,
                origin: Some(UserOrigin::Combined),
                username: None,
                hostname: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_username_hostname() {
        let client = TestClient::new();

        client
            .admin
            .test(Request {
                limit: None,
                offset: None,
                sort: None,
                state: None,
                origin: None,
                username: Some("admin".to_string()),
                hostname: Some("host".to_string()),
            })
            .await;
    }
}
