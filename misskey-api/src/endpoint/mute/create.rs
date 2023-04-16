use crate::model::{id::Id, user::User};

#[cfg(feature = "12-108-0")]
use chrono::serde::ts_milliseconds_option;
#[cfg(feature = "12-108-0")]
use chrono::{DateTime, Utc};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: Id<User>,
    #[cfg(feature = "12-108-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-108-0")))]
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option"
    )]
    #[builder(default, setter(strip_option, into))]
    pub expires_at: Option<DateTime<Utc>>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "mute/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let (user, _) = client.admin.create_user().await;

        client
            .user
            .test(Request {
                user_id: user.id,
                #[cfg(feature = "12-108-0")]
                expires_at: None,
            })
            .await;
    }

    #[cfg(feature = "12-108-0")]
    #[tokio::test]
    async fn request_with_expires_at() {
        let client = TestClient::new();
        let (user, _) = client.admin.create_user().await;

        client
            .user
            .test(Request {
                user_id: user.id,
                expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            })
            .await;
    }
}
