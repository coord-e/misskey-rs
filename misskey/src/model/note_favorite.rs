use crate::model::note::{Note, NoteId};

use chrono::{DateTime, Utc};
use derive_more::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug)]
#[serde(transparent)]
pub struct NoteFavoriteId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoteFavorite {
    pub id: NoteFavoriteId,
    pub created_at: DateTime<Utc>,
    pub note_id: NoteId,
    pub note: Note,
}