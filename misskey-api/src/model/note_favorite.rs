use crate::model::{id::Id, note::Note};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoteFavorite {
    pub id: Id<NoteFavorite>,
    pub created_at: DateTime<Utc>,
    pub note_id: Id<Note>,
    pub note: Note,
}

impl_entity!(NoteFavorite);
