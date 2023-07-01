#[cfg(feature = "12-93-0")]
use crate::model::note::Note;
use crate::model::{id::Id, note::Reaction, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoteReaction {
    pub id: Id<NoteReaction>,
    pub created_at: DateTime<Utc>,
    pub user: User,
    #[serde(rename = "type")]
    pub type_: Reaction,
    #[cfg(feature = "12-93-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-93-0")))]
    #[serde(default)]
    pub note: Option<Note>,
}

impl_entity!(NoteReaction);
