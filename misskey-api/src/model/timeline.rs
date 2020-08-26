use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Timeline {
    #[serde(rename = "homeTimeline")]
    Home,
    #[serde(rename = "localTimeline")]
    Local,
    #[serde(rename = "hybridTimeline")]
    Social,
    #[serde(rename = "globalTimeline")]
    Global,
}

#[derive(Debug, Display, Error, Clone)]
#[display(fmt = "invalid timeline type")]
pub struct ParseTimelineError;

impl std::str::FromStr for Timeline {
    type Err = ParseTimelineError;

    fn from_str(s: &str) -> Result<Timeline, Self::Err> {
        match s {
            "home" | "Home" => Ok(Timeline::Home),
            "local" | "Local" => Ok(Timeline::Local),
            "social" | "Social" => Ok(Timeline::Social),
            "global" | "Global" => Ok(Timeline::Global),
            _ => Err(ParseTimelineError),
        }
    }
}
