use crate::model::note_favorite::{NoteFavorite, NoteFavoriteId};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<NoteFavoriteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<NoteFavoriteId>,
}

impl ApiRequest for Request {
    type Response = Vec<NoteFavorite>;
    const ENDPOINT: &'static str = "i/favorites";
}
