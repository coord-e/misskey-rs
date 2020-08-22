use crate::model::user_group::UserGroupInvitationId;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[cfg(feature = "12-8-0")]
    pub invitation_id: UserGroupInvitationId,
    #[cfg(not(feature = "12-8-0"))]
    #[serde(rename = "inviteId")]
    pub invitation_id: UserGroupInvitationId,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "users/groups/invitations/reject";
}
