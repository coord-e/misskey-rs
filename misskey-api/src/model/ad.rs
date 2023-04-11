use std::fmt::{self, Display};

use crate::model::id::Id;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ad {
    pub id: Id<Ad>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub place: Place,
    pub priority: Priority,
    pub url: String,
    pub image_url: String,
    pub memo: String,
}

impl_entity!(Ad);

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Place {
    #[default]
    Square,
    Horizontal,
}

impl Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Place::Square => f.write_str("square"),
            Place::Horizontal => f.write_str("horizontal"),
        }
    }
}

#[derive(Debug, Error, Clone)]
#[error("invalid place")]
pub struct ParsePlaceError {
    _priv: (),
}

impl std::str::FromStr for Place {
    type Err = ParsePlaceError;

    fn from_str(s: &str) -> Result<Place, Self::Err> {
        match s {
            "square" | "Square" => Ok(Place::Square),
            "horizontal" | "Horizontal" => Ok(Place::Horizontal),
            _ => Err(ParsePlaceError { _priv: () }),
        }
    }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Priority {
    High,
    #[default]
    Middle,
    Low,
}

impl Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::High => f.write_str("high"),
            Priority::Middle => f.write_str("middle"),
            Priority::Low => f.write_str("low"),
        }
    }
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
            "high" | "High" => Ok(Priority::High),
            "middle" | "Middle" => Ok(Priority::Middle),
            "low" | "Low" => Ok(Priority::Low),
            _ => Err(ParsePriorityError { _priv: () }),
        }
    }
}
