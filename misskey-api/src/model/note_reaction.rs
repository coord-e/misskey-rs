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
}

impl_entity!(NoteReaction);
