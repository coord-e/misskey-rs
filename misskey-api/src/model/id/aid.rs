use std::fmt::{self, Display};
use std::str::FromStr;

use chrono::{DateTime, TimeZone, Utc};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Aid {
    pub timestamp: i64,
    /// 0 - 1295
    pub random: u16,
}

impl Aid {
    pub fn datetime(&self) -> DateTime<Utc> {
        // NOTE: this does not panic when parsed from valid Aid since the following does not panic
        // `Utc.timestamp_millis_opt(36_i64.pow(8) - 1).unwrap()`
        Utc.timestamp_millis_opt(self.timestamp).unwrap()
    }
}

// https://github.com/misskey-dev/misskey/blob/12.75.1/src/misc/id/aid.ts#L6
const TIME2000: i64 = 946684800000;

#[derive(Debug, Error, Clone)]
#[error("invalid aid")]
pub struct ParseAidError {
    _priv: (),
}

impl FromStr for Aid {
    type Err = ParseAidError;

    fn from_str(s: &str) -> Result<Aid, Self::Err> {
        let (timestamp_str, random_str) = s.split_at(s.len() - 2);

        let timestamp = match i64::from_str_radix(timestamp_str, 36) {
            Ok(x) => x + TIME2000,
            Err(_) => return Err(ParseAidError { _priv: () }),
        };

        let random = match u16::from_str_radix(random_str, 36) {
            Ok(x) => x,
            Err(_) => return Err(ParseAidError { _priv: () }),
        };

        Ok(Aid { timestamp, random })
    }
}

struct Radix36(u64);

impl Radix36 {
    fn new(x: impl Into<u64>) -> Radix36 {
        Radix36(x.into())
    }
}

impl Display for Radix36 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;

        let width = f
            .width()
            .unwrap_or_else(|| (self.0 as f64).log(36.0).floor() as usize + 1);

        (0..width)
            .rev()
            .map(|i| self.0 / 36_u64.pow(i.try_into().unwrap()) % 36)
            .map(|d| std::char::from_digit(d.try_into().unwrap(), 36).unwrap())
            .try_for_each(|c| f.write_char(c))
    }
}

impl Display for Aid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let since_2000 = u64::try_from(self.timestamp - TIME2000).unwrap_or(0);
        let timestamp_fmt = Radix36::new(since_2000);
        let random_fmt = Radix36::new(self.random % 1296);
        write!(f, "{:08}{:02}", timestamp_fmt, random_fmt)
    }
}

impl Serialize for Aid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Aid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::Aid;
    use chrono::{DateTime, Duration, TimeZone, Utc};
    use rand::{self, Rng};

    fn new() -> Aid {
        from_datetime(Utc::now())
    }

    fn from_datetime<Tz>(datetime: DateTime<Tz>) -> Aid
    where
        Tz: TimeZone,
    {
        from_datetime_with_source(datetime, &mut rand::thread_rng())
    }

    fn from_datetime_with_source<Tz, R>(datetime: DateTime<Tz>, source: &mut R) -> Aid
    where
        Tz: TimeZone,
        R: Rng,
    {
        let timestamp = datetime.timestamp_millis();
        let random = source.gen::<u16>() % 1296;
        Aid { timestamp, random }
    }

    #[test]
    fn test_deserialize_const() {
        let string = "8dhemt9ubf";
        let aid: Aid = string.parse().expect("failed to parse");
        assert_eq!(aid.datetime(), Utc.timestamp_millis(1602948787122));
    }

    #[test]
    fn test_serialize_deserialize() {
        let aid1 = new();
        let string = aid1.to_string();
        let aid2: Aid = string.parse().expect("failed to parse");
        assert_eq!(aid1, aid2);
    }

    #[test]
    fn test_deserialize_serialize() {
        let string1 = "8dhe5zqidm";
        let aid: Aid = string1.parse().expect("failed to parse");
        let string2 = aid.to_string();
        assert_eq!(string1, string2);
    }

    #[test]
    fn test_deserialize_serialize2() {
        let string1 = "8ejiidh50m";
        let aid: Aid = string1.parse().expect("failed to parse");
        let string2 = aid.to_string();
        assert_eq!(string1, string2);
    }

    #[test]
    fn test_order() {
        let time = Utc::now();
        let aid1 = from_datetime(time);
        let aid2 = from_datetime(time + Duration::milliseconds(1));
        assert!(aid1 < aid2);
    }
}
