use crate::model::user::{User, UserId};

use chrono::{DateTime, Utc};
use derive_more::{Display, Error, FromStr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "camelCase")]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Success,
    Debug,
}

#[derive(Debug, Display, Error, Clone)]
#[display(fmt = "invalid log level")]
pub struct ParseLogLevelError;

impl std::str::FromStr for LogLevel {
    type Err = ParseLogLevelError;

    fn from_str(s: &str) -> Result<LogLevel, Self::Err> {
        match s {
            "error" | "Error" => Ok(LogLevel::Error),
            "warning" | "Warning" => Ok(LogLevel::Warning),
            "info" | "Info" => Ok(LogLevel::Info),
            "success" | "Success" => Ok(LogLevel::Success),
            "debug" | "Debug" => Ok(LogLevel::Debug),
            _ => Err(ParseLogLevelError),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct LogId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub id: LogId,
    pub created_at: DateTime<Utc>,
    pub domain: Vec<String>,
    pub level: LogLevel,
    pub worker: String,
    pub machine: String,
    pub message: String,
    pub data: serde_json::Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct ModerationLogId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModerationLog {
    pub id: ModerationLogId,
    pub created_at: DateTime<Utc>,
    pub user_id: UserId,
    pub user: User,
    #[serde(rename = "type")]
    pub type_: String,
    pub info: serde_json::Map<String, serde_json::Value>,
}
