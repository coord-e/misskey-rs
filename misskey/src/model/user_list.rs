use derivative::Derivative;
use derive_more::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct UserListId(pub String);
