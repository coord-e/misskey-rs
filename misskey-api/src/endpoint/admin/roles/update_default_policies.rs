use crate::model::role::PoliciesSimple;

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub policies: PoliciesSimple,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/roles/update-default-policies";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::{
        model::role::PoliciesSimple,
        test::{ClientExt, TestClient},
    };

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.admin.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        client
            .admin
            .test(Request {
                policies: PoliciesSimple {
                    gtl_available: Some(true),
                    ltl_available: Some(true),
                    can_public_note: Some(true),
                    can_invite: Some(true),
                    can_manage_custom_emojis: Some(true),
                    can_hide_ads: Some(false),
                    drive_capacity_mb: Some(1000),
                    pin_limit: Some(100),
                    antenna_limit: Some(10),
                    word_mute_limit: Some(10000),
                    webhook_limit: Some(10),
                    clip_limit: Some(1000),
                    note_each_clips_limit: Some(10000),
                    user_list_limit: Some(100),
                    user_each_user_lists_limit: Some(1000),
                    rate_limit_factor: Some(0.5),
                },
            })
            .await;
    }
}
