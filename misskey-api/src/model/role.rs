use crate::model::{id::Id, user::User};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: Id<Role>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub color: Option<String>,
    #[cfg(feature = "13-4-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-4-0")))]
    pub icon_url: Option<String>,
    pub target: Target,
    #[serde(with = "cond_formula_option")]
    pub cond_formula: Option<RoleCondFormulaValue>,
    pub is_public: bool,
    pub is_moderator: bool,
    pub is_administrator: bool,
    #[cfg(feature = "13-12-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-12-0")))]
    pub is_explorable: bool,
    #[cfg(feature = "13-4-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-4-0")))]
    pub as_badge: bool,
    pub can_edit_members_by_moderator: bool,
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub display_order: i64,
    pub policies: Policies,
    pub users_count: u64,
    #[cfg(not(feature = "13-7-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-7-0"))))]
    #[serde(default)]
    pub users: Option<Vec<User>>,
}

impl_entity!(Role);

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Target {
    #[default]
    Manual,
    Conditional,
}

pub(crate) mod cond_formula_option {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::RoleCondFormulaValue;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(deny_unknown_fields)]
    struct Empty {}

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(untagged)]
    enum ValueOrEmpty {
        Value(RoleCondFormulaValue),
        Empty(Empty),
    }

    pub fn serialize<S>(
        opt: &Option<RoleCondFormulaValue>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt {
            Some(cond_formula) => {
                serializer.serialize_some(&ValueOrEmpty::Value(cond_formula.clone()))
            }
            None => serializer.serialize_some(&ValueOrEmpty::Empty(Empty {})),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<RoleCondFormulaValue>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Deserialize::deserialize(deserializer)? {
            ValueOrEmpty::Value(cond_formula) => Ok(Some(cond_formula)),
            ValueOrEmpty::Empty(_) => Ok(None),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum RoleCondFormulaValue {
    And {
        values: Vec<RoleCondFormulaValue>,
    },
    Or {
        values: Vec<RoleCondFormulaValue>,
    },
    Not {
        value: Box<RoleCondFormulaValue>,
    },
    IsLocal,
    IsRemote,
    CreatedLessThan {
        #[serde(rename = "sec", with = "duration_seconds")]
        duration: Duration,
    },
    CreatedMoreThan {
        #[serde(rename = "sec", with = "duration_seconds")]
        duration: Duration,
    },
    FollowersLessThanOrEq {
        value: u64,
    },
    FollowersMoreThanOrEq {
        value: u64,
    },
    FollowingLessThanOrEq {
        value: u64,
    },
    FollowingMoreThanOrEq {
        value: u64,
    },
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    NotesLessThanOrEq {
        value: u64,
    },
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    NotesMoreThanOrEq {
        value: u64,
    },
}

mod duration_seconds {
    use chrono::Duration;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(duration.num_seconds())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = i64::deserialize(deserializer)?;
        Ok(Duration::seconds(seconds))
    }
}

impl RoleCondFormulaValue {
    pub fn and(self, rhs: impl Into<Self>) -> Self {
        let rhs = rhs.into();
        match self {
            Self::And { values: mut v } => {
                v.push(rhs);
                Self::And { values: v }
            }
            lhs => Self::And {
                values: vec![lhs, rhs],
            },
        }
    }

    pub fn or(self, rhs: impl Into<Self>) -> Self {
        let rhs = rhs.into();
        match self {
            Self::Or { values: mut v } => {
                v.push(rhs);
                Self::Or { values: v }
            }
            lhs => Self::Or {
                values: vec![lhs, rhs],
            },
        }
    }
}

impl std::ops::Not for RoleCondFormulaValue {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::Not {
            value: Box::new(self),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Policies {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub gtl_available: Option<PolicyValue<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ltl_available: Option<PolicyValue<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub can_public_note: Option<PolicyValue<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub can_invite: Option<PolicyValue<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub can_manage_custom_emojis: Option<PolicyValue<bool>>,
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub can_search_notes: Option<PolicyValue<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub can_hide_ads: Option<PolicyValue<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub drive_capacity_mb: Option<PolicyValue<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub pin_limit: Option<PolicyValue<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub antenna_limit: Option<PolicyValue<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub word_mute_limit: Option<PolicyValue<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub webhook_limit: Option<PolicyValue<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub clip_limit: Option<PolicyValue<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub note_each_clips_limit: Option<PolicyValue<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub user_list_limit: Option<PolicyValue<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub user_each_user_lists_limit: Option<PolicyValue<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub rate_limit_factor: Option<PolicyValue<f64>>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct PolicyValue<T> {
    pub use_default: bool,
    #[serde(with = "priority_u8")]
    pub priority: Priority,
    pub value: T,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Priority {
    #[default]
    Low,
    Middle,
    High,
}

#[derive(Debug, Error, Clone)]
#[error("invalid priority")]
pub struct ParsePriorityError {
    _priv: (),
}

impl std::str::FromStr for Priority {
    type Err = ParsePriorityError;

    fn from_str(s: &str) -> Result<Priority, Self::Err> {
        match s {
            "low" | "Low" => Ok(Priority::Low),
            "middle" | "Middle" => Ok(Priority::Middle),
            "high" | "High" => Ok(Priority::High),
            _ => Err(ParsePriorityError { _priv: () }),
        }
    }
}

mod priority_u8 {
    use super::Priority;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(priority: &Priority, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(match priority {
            Priority::Low => 0,
            Priority::Middle => 1,
            Priority::High => 2,
        })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Priority, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(Priority::Low),
            1 => Ok(Priority::Middle),
            2 => Ok(Priority::High),
            _ => Err(de::Error::custom("invalid priority")),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct PoliciesSimple {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub gtl_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ltl_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub can_public_note: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub can_invite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub can_manage_custom_emojis: Option<bool>,
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub can_search_notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub can_hide_ads: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub drive_capacity_mb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub pin_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub antenna_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub word_mute_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub webhook_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub clip_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub note_each_clips_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub user_list_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub user_each_user_lists_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub rate_limit_factor: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoleAssignment {
    pub id: Id<RoleAssignment>,
    pub user: User,
}

impl_entity!(RoleAssignment);
