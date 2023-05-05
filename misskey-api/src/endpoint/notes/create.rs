#[cfg(feature = "12-47-0")]
use crate::model::channel::Channel;
#[cfg(feature = "13-10-0")]
use crate::model::note::ReactionAcceptance;
use crate::model::{
    drive::DriveFile,
    id::Id,
    note::{Note, Visibility},
    user::User,
};

use chrono::{serde::ts_milliseconds_option, DateTime, Duration, Utc};
use serde::{Deserialize, Serialize, Serializer};
use typed_builder::TypedBuilder;

fn serialize_duration_milliseconds_option<S>(
    duration: &Option<Duration>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match duration {
        None => serializer.serialize_none(),
        Some(x) => serializer.serialize_some(&x.num_milliseconds()),
    }
}

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct PollRequest {
    pub choices: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub multiple: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option"
    )]
    #[builder(default, setter(strip_option, into))]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_duration_milliseconds_option"
    )]
    #[builder(default, setter(strip_option))]
    pub expired_after: Option<Duration>,
}

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub visibility: Option<Visibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub visible_user_ids: Option<Vec<Id<User>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub text: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub cw: Option<String>,
    #[cfg(not(feature = "12-96-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-96-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub via_mobile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub local_only: Option<bool>,
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub reaction_acceptance: Option<ReactionAcceptance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub no_extract_mentions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub no_extract_hashtags: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub no_extract_emojis: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub file_ids: Option<Vec<Id<DriveFile>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub reply_id: Option<Id<Note>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub renote_id: Option<Id<Note>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub poll: Option<PollRequest>,
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub channel_id: Option<Id<Channel>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub created_note: Note,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "notes/create";
}

#[cfg(test)]
mod tests {
    use super::{PollRequest, Request};
    use crate::test::{ClientExt, HttpClientExt, TestClient};

    #[tokio::test]
    async fn request_with_text() {
        let client = TestClient::new();
        client
            .test(Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("some text".to_string()),
                cw: None,
                #[cfg(not(feature = "12-96-0"))]
                via_mobile: None,
                local_only: None,
                #[cfg(feature = "13-10-0")]
                reaction_acceptance: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        client
            .test(Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("aww yeah".to_string()),
                cw: None,
                #[cfg(not(feature = "12-96-0"))]
                via_mobile: Some(true),
                local_only: Some(true),
                #[cfg(feature = "13-10-0")]
                reaction_acceptance: None,
                no_extract_mentions: Some(true),
                no_extract_hashtags: Some(true),
                no_extract_emojis: Some(true),
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_cw() {
        let client = TestClient::new();
        client
            .test(Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("!".to_string()),
                cw: Some("nsfw".to_string()),
                #[cfg(not(feature = "12-96-0"))]
                via_mobile: None,
                local_only: None,
                #[cfg(feature = "13-10-0")]
                reaction_acceptance: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_visibilty() {
        use crate::model::note::Visibility;

        let client = TestClient::new();
        client
            .test(Request {
                visibility: Some(Visibility::Home),
                visible_user_ids: None,
                text: Some("hello home".to_string()),
                cw: None,
                #[cfg(not(feature = "12-96-0"))]
                via_mobile: None,
                local_only: None,
                #[cfg(feature = "13-10-0")]
                reaction_acceptance: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await;
        client
            .test(Request {
                visibility: Some(Visibility::Public),
                visible_user_ids: None,
                text: Some("hello public".to_string()),
                cw: None,
                #[cfg(not(feature = "12-96-0"))]
                via_mobile: None,
                local_only: None,
                #[cfg(feature = "13-10-0")]
                reaction_acceptance: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await;
        client
            .test(Request {
                visibility: Some(Visibility::Followers),
                visible_user_ids: None,
                text: Some("hello followers".to_string()),
                cw: None,
                #[cfg(not(feature = "12-96-0"))]
                via_mobile: None,
                local_only: None,
                #[cfg(feature = "13-10-0")]
                reaction_acceptance: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await;

        let admin = client.admin.me().await;
        client
            .user
            .test(Request {
                visibility: Some(Visibility::Specified),
                visible_user_ids: Some(vec![admin.id]),
                text: Some("hello specific person".to_string()),
                cw: None,
                #[cfg(not(feature = "12-96-0"))]
                via_mobile: None,
                local_only: None,
                #[cfg(feature = "13-10-0")]
                reaction_acceptance: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await;
    }

    #[cfg(feature = "13-10-0")]
    #[tokio::test]
    async fn request_with_reaction_acceptance() {
        use crate::model::note::ReactionAcceptance;

        let client = TestClient::new();
        client
            .test(Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("nobody can react me".to_string()),
                cw: None,
                local_only: None,
                reaction_acceptance: Some(ReactionAcceptance::LikeOnly),
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                channel_id: None,
            })
            .await;
        let client = TestClient::new();
        client
            .test(Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("remote cannot react me".to_string()),
                cw: None,
                local_only: None,
                reaction_acceptance: Some(ReactionAcceptance::LikeOnlyForRemote),
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                channel_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_renote() {
        let client = TestClient::new();
        let note = client
            .test(Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("renote".to_string()),
                cw: None,
                #[cfg(not(feature = "12-96-0"))]
                via_mobile: None,
                local_only: None,
                #[cfg(feature = "13-10-0")]
                reaction_acceptance: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await
            .created_note;
        client
            .test(Request {
                visibility: None,
                visible_user_ids: None,
                text: None,
                cw: None,
                #[cfg(not(feature = "12-96-0"))]
                via_mobile: None,
                local_only: None,
                #[cfg(feature = "13-10-0")]
                reaction_acceptance: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: Some(note.id),
                poll: None,
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_reply() {
        let client = TestClient::new();
        let note = client
            .test(Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("reply".to_string()),
                cw: None,
                #[cfg(not(feature = "12-96-0"))]
                via_mobile: None,
                local_only: None,
                #[cfg(feature = "13-10-0")]
                reaction_acceptance: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await
            .created_note;
        client
            .test(Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("hey".to_string()),
                cw: None,
                #[cfg(not(feature = "12-96-0"))]
                via_mobile: None,
                local_only: None,
                #[cfg(feature = "13-10-0")]
                reaction_acceptance: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: Some(note.id),
                renote_id: None,
                poll: None,
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_poll() {
        let client = TestClient::new();

        let poll1 = PollRequest {
            choices: vec!["a".to_string(), "b".to_string()],
            multiple: Some(true),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            expired_after: None,
        };
        let poll2 = PollRequest {
            choices: vec!["c".to_string(), "d".to_string()],
            multiple: Some(false),
            expires_at: None,
            expired_after: Some(chrono::Duration::hours(1)),
        };

        for poll in &[poll1, poll2] {
            client
                .test(Request {
                    visibility: None,
                    visible_user_ids: None,
                    text: Some("poll".to_string()),
                    cw: None,
                    #[cfg(not(feature = "12-96-0"))]
                    via_mobile: None,
                    local_only: None,
                    #[cfg(feature = "13-10-0")]
                    reaction_acceptance: None,
                    no_extract_mentions: None,
                    no_extract_hashtags: None,
                    no_extract_emojis: None,
                    file_ids: None,
                    reply_id: None,
                    renote_id: None,
                    poll: Some(poll.clone()),
                    #[cfg(feature = "12-47-0")]
                    channel_id: None,
                })
                .await;
        }
    }

    #[tokio::test]
    async fn request_with_files() {
        let client = TestClient::new();
        let file1 = client.create_text_file("test1.txt", "hi").await;
        let file2 = client.create_text_file("test2.txt", "hi").await;

        client
            .test(Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("some text".to_string()),
                cw: None,
                #[cfg(not(feature = "12-96-0"))]
                via_mobile: None,
                local_only: None,
                #[cfg(feature = "13-10-0")]
                reaction_acceptance: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: Some(vec![file1.id, file2.id]),
                reply_id: None,
                renote_id: None,
                poll: None,
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await;
    }

    #[cfg(feature = "12-47-0")]
    #[tokio::test]
    async fn request_with_channel_id() {
        let client = TestClient::new();
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test channel")
                    .build(),
            )
            .await;

        client
            .test(Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("hi channel".to_string()),
                cw: None,
                #[cfg(not(feature = "12-96-0"))]
                via_mobile: None,
                local_only: None,
                #[cfg(feature = "13-10-0")]
                reaction_acceptance: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: None,
                channel_id: Some(channel.id),
            })
            .await;
    }
}
