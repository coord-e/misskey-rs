use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ChartSpan {
    Day,
    Hour,
}

#[derive(Debug, Display, Error, Clone)]
#[display(fmt = "invalid chart span")]
pub struct ParseChartSpanError;

impl std::str::FromStr for ChartSpan {
    type Err = ParseChartSpanError;

    fn from_str(s: &str) -> Result<ChartSpan, Self::Err> {
        match s {
            "day" | "Day" => Ok(ChartSpan::Day),
            "hour" | "Hour" => Ok(ChartSpan::Hour),
            _ => Err(ParseChartSpanError),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DriveChart {
    #[serde(alias = "totalFiles")]
    pub total_count: Vec<u64>,
    #[serde(alias = "totalUsage")]
    pub total_size: Vec<u64>,
    #[serde(alias = "incFiles")]
    pub inc_count: Vec<u64>,
    #[serde(alias = "incUsage")]
    pub inc_size: Vec<u64>,
    #[serde(alias = "decFiles")]
    pub dec_count: Vec<u64>,
    #[serde(alias = "decUsage")]
    pub dec_size: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FederationChart {
    pub total: Vec<u64>,
    pub inc: Vec<u64>,
    pub dec: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActiveUsersChart {
    pub count: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HashtagChart {
    pub count: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestsChart {
    pub failed: Vec<u64>,
    pub succeeded: Vec<u64>,
    pub received: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NotesChart {
    pub total: Vec<u64>,
    pub inc: Vec<u64>,
    pub dec: Vec<u64>,
    pub diffs: NotesDiffsChart,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NotesDiffsChart {
    pub normal: Vec<u64>,
    pub reply: Vec<u64>,
    pub renote: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UsersChart {
    pub total: Vec<u64>,
    pub inc: Vec<u64>,
    pub dec: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowingChart {
    pub total: Vec<u64>,
    pub inc: Vec<u64>,
    pub dec: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowersChart {
    pub total: Vec<u64>,
    pub inc: Vec<u64>,
    pub dec: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkChart {
    pub incoming_requests: Vec<u64>,
    pub outgoing_requests: Vec<u64>,
    pub total_time: Vec<u64>,
    pub incoming_bytes: Vec<u64>,
    pub outgoing_bytes: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReactionsChart {
    pub count: Vec<u64>,
}
