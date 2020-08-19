use crate::model::user_group::UserGroupInvitationId;

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub invitation_id: UserGroupInvitationId,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "users/groups/invitations/reject";
}
