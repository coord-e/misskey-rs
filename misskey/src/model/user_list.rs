use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct UserListId(pub String);
