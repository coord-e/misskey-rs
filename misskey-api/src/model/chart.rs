use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ChartSpan {
    Day,
    Hour,
}

#[derive(Debug, Error, Clone)]
#[error("invalid chart span")]
pub struct ParseChartSpanError {
    _priv: (),
}

impl std::str::FromStr for ChartSpan {
    type Err = ParseChartSpanError;

    fn from_str(s: &str) -> Result<ChartSpan, Self::Err> {
        match s {
            "day" | "Day" => Ok(ChartSpan::Day),
            "hour" | "Hour" => Ok(ChartSpan::Hour),
            _ => Err(ParseChartSpanError { _priv: () }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DriveChart {
    #[cfg(not(feature = "12-104-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-104-0"))))]
    #[serde(alias = "totalFiles")]
    pub total_count: Vec<u64>,
    #[cfg(not(feature = "12-104-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-104-0"))))]
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

#[cfg(feature = "12-104-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-104-0")))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDriveChart {
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

#[cfg(not(feature = "12-104-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-104-0"))))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FederationChart {
    pub total: Vec<u64>,
    pub inc: Vec<u64>,
    pub dec: Vec<u64>,
}

#[cfg(all(feature = "12-104-0", not(feature = "12-106-0")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "12-104-0", not(feature = "12-106-0")))))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstanceChart {
    pub total: Vec<u64>,
    pub inc: Vec<u64>,
    pub dec: Vec<u64>,
}

#[cfg(feature = "12-104-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-104-0")))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FederationChart {
    #[cfg(not(feature = "12-106-0"))]
    pub instance: InstanceChart,
    pub delivered_instances: Vec<u64>,
    pub inbox_instances: Vec<u64>,
    pub stalled: Vec<u64>,
    #[cfg(feature = "12-106-0")]
    pub sub: Vec<u64>,
    #[cfg(feature = "12-106-0")]
    #[serde(rename = "pub")]
    pub pub_: Vec<u64>,
    #[cfg(feature = "12-108-0")]
    pub pubsub: Vec<u64>,
    #[cfg(feature = "12-108-0")]
    pub sub_active: Vec<u64>,
    #[cfg(feature = "12-108-0")]
    pub pub_active: Vec<u64>,
}

#[cfg(not(feature = "12-104-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "12-104-0"))))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActiveUsersChart {
    #[cfg(not(feature = "12-75-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-75-0"))))]
    pub count: Vec<u64>,
    #[cfg(feature = "12-75-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-75-0")))]
    pub users: Vec<u64>,
}

#[cfg(feature = "12-104-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-104-0")))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActiveUsersChart {
    pub read_write: Vec<u64>,
    pub read: Vec<u64>,
    pub write: Vec<u64>,
    pub registered_within_week: Vec<u64>,
    pub registered_within_month: Vec<u64>,
    pub registered_within_year: Vec<u64>,
    pub registered_outside_week: Vec<u64>,
    pub registered_outside_month: Vec<u64>,
    pub registered_outside_year: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HashtagChart {
    #[cfg(not(feature = "12-75-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-75-0"))))]
    pub count: Vec<u64>,
    #[cfg(feature = "12-75-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-75-0")))]
    pub users: Vec<u64>,
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
    #[cfg(feature = "12-104-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-104-0")))]
    pub with_file: Vec<u64>,
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

#[cfg(feature = "12-104-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-104-0")))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApRequestChart {
    pub deliver_failed: Vec<u64>,
    pub deliver_succeeded: Vec<u64>,
    pub inbox_received: Vec<u64>,
}
