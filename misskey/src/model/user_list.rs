use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Derivative)]
#[derivative(Debug = "transparent")]
pub struct UserListId(pub String);
