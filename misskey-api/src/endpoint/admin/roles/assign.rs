use crate::model::{id::Id, role::Role, user::User};

#[cfg(feature = "13-9-0")]
use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub role_id: Id<Role>,
    pub user_id: Id<User>,
    #[cfg(feature = "13-9-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-9-0")))]
    #[serde(with = "ts_milliseconds_option")]
    #[builder(default, setter(into))]
    pub expires_at: Option<DateTime<Utc>>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/roles/assign";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let (user, _) = client.admin.create_user().await;
        let role = client
            .admin
            .test(crate::endpoint::admin::roles::create::Request::default())
            .await;

        client
            .admin
            .test(Request {
                role_id: role.id,
                user_id: user.id,
                #[cfg(feature = "13-9-0")]
                expires_at: None,
            })
            .await;
    }

    #[cfg(feature = "13-9-0")]
    #[tokio::test]
    async fn request_with_expires_at() {
        let client = TestClient::new();
        let (user, _) = client.admin.create_user().await;
        let role = client
            .admin
            .test(crate::endpoint::admin::roles::create::Request::default())
            .await;

        client
            .admin
            .test(Request {
                role_id: role.id,
                user_id: user.id,
                expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            })
            .await;
    }
}
