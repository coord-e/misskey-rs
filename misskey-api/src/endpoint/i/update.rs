#[cfg(feature = "12-48-0")]
use std::collections::HashSet;

#[cfg(feature = "12-48-0")]
use crate::model::notification::NotificationType;
#[cfg(feature = "12-70-0")]
use crate::model::user::UserEmailNotificationType;
use crate::model::{drive::DriveFile, id::Id, page::Page, query::Query, user::User};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone)]
pub struct UserFieldRequest {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub lang: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub location: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub birthday: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub avatar_id: Option<Option<Id<DriveFile>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub banner_id: Option<Option<Id<DriveFile>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub fields: Option<[UserFieldRequest; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_locked: Option<bool>,
    #[cfg(feature = "12-63-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-63-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_explorable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub careful_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub auto_accept_followed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_cat: Option<bool>,
    #[cfg(any(docsrs, not(feature = "12-55-0")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-55-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub auto_watch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub inject_featured_note: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub always_mark_nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub pinned_page_id: Option<Option<Id<Page>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub muted_words: Option<Query<String>>,
    #[cfg(feature = "12-60-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-60-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub no_crawle: Option<bool>,
    #[cfg(feature = "12-69-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-69-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub receive_announcement_email: Option<bool>,
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub muting_notification_types: Option<HashSet<NotificationType>>,
    #[cfg(feature = "12-70-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-70-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email_notification_types: Option<HashSet<UserEmailNotificationType>>,
}

impl misskey_core::Request for Request {
    type Response = User;
    const ENDPOINT: &'static str = "i/update";
}

#[cfg(test)]
mod tests {
    use super::{Request, UserFieldRequest};
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_options() {
        #[cfg(feature = "12-48-0")]
        use crate::model::notification::NotificationType;
        use crate::model::query::Query;
        #[cfg(feature = "12-70-0")]
        use crate::model::user::UserEmailNotificationType;

        let client = TestClient::new();
        client
            .test(Request {
                name: Some(Some("test".to_string())),
                description: Some(Some("test description".to_string())),
                lang: Some(Some("ja-JP".to_string())),
                location: Some(Some("somewhere".to_string())),
                birthday: None,
                avatar_id: None,
                banner_id: None,
                fields: Some([
                    UserFieldRequest {
                        name: Some("key".to_string()),
                        value: Some("value".to_string()),
                    },
                    Default::default(),
                    Default::default(),
                    Default::default(),
                ]),
                is_locked: Some(true),
                #[cfg(feature = "12-63-0")]
                is_explorable: Some(false),
                careful_bot: Some(true),
                auto_accept_followed: Some(true),
                is_bot: Some(true),
                is_cat: Some(true),
                #[cfg(not(feature = "12-55-0"))]
                auto_watch: Some(true),
                inject_featured_note: Some(true),
                always_mark_nsfw: Some(true),
                pinned_page_id: None,
                muted_words: Some(Query::from_vec(vec![
                    vec!["mute1".to_string(), "mute2".to_string()],
                    vec!["mute3".to_string()],
                ])),
                #[cfg(feature = "12-60-0")]
                no_crawle: Some(true),
                #[cfg(feature = "12-69-0")]
                receive_announcement_email: Some(true),
                #[cfg(feature = "12-48-0")]
                muting_notification_types: Some(
                    vec![NotificationType::Follow, NotificationType::Mention]
                        .into_iter()
                        .collect(),
                ),
                #[cfg(feature = "12-70-0")]
                email_notification_types: Some(
                    vec![
                        UserEmailNotificationType::Follow,
                        UserEmailNotificationType::Mention,
                    ]
                    .into_iter()
                    .collect(),
                ),
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_null_options() {
        let client = TestClient::new();
        client
            .test(Request {
                name: Some(None),
                description: Some(None),
                lang: Some(None),
                location: Some(None),
                birthday: Some(None),
                avatar_id: Some(None),
                banner_id: Some(None),
                fields: None,
                is_locked: None,
                #[cfg(feature = "12-63-0")]
                is_explorable: None,
                careful_bot: None,
                auto_accept_followed: None,
                is_bot: None,
                is_cat: None,
                #[cfg(not(feature = "12-55-0"))]
                auto_watch: None,
                inject_featured_note: None,
                always_mark_nsfw: None,
                pinned_page_id: Some(None),
                muted_words: None,
                #[cfg(feature = "12-60-0")]
                no_crawle: None,
                #[cfg(feature = "12-69-0")]
                receive_announcement_email: None,
                #[cfg(feature = "12-48-0")]
                muting_notification_types: None,
                #[cfg(feature = "12-70-0")]
                email_notification_types: None,
            })
            .await;
    }
}
