use chrono::{DateTime as ChronoDateTime, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::ValidationError;

const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime(NaiveDateTime);

impl DateTime {
    pub fn new(value: NaiveDateTime) -> Self {
        Self(value)
    }

    pub fn value(self) -> NaiveDateTime {
        self.0
    }

    pub fn date(self) -> NaiveDate {
        self.0.date()
    }
}

impl From<NaiveDateTime> for DateTime {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<NaiveDate> for DateTime {
    fn from(value: NaiveDate) -> Self {
        Self(value.and_time(chrono::NaiveTime::MIN))
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(FORMAT))
    }
}

impl Serialize for DateTime {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

impl std::str::FromStr for DateTime {
    type Err = ValidationError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let Ok(naive) = NaiveDateTime::parse_from_str(raw, FORMAT) {
            return Ok(Self(naive));
        }
        if let Ok(date) = NaiveDate::parse_from_str(raw, "%Y-%m-%d") {
            return Ok(date.into());
        }
        ChronoDateTime::parse_from_rfc3339(raw)
            .map(|dt| Self(dt.naive_local()))
            .map_err(|_| ValidationError::new("date-time", "unrecognized format"))
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        raw.parse()
            .map_err(|_| serde::de::Error::custom(format!("unrecognized date-time: {raw}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Result<DateTime, serde_json::Error> {
        serde_json::from_str(&format!("\"{s}\""))
    }

    #[test]
    fn reads_the_format_snelstart_documents() {
        let t = parse("2018-08-07T06:24:48.12").unwrap();
        assert_eq!(t.to_string(), "2018-08-07T06:24:48.120");
    }

    #[test]
    fn reads_without_fraction_and_plain_dates() {
        assert_eq!(
            parse("2018-08-07T06:24:48").unwrap().to_string(),
            "2018-08-07T06:24:48"
        );
        assert_eq!(
            parse("2018-08-07").unwrap().date().to_string(),
            "2018-08-07"
        );
    }

    #[test]
    fn offsets_are_dropped_not_converted() {
        assert_eq!(
            parse("2018-08-07T06:24:48+02:00").unwrap().to_string(),
            "2018-08-07T06:24:48"
        );
    }

    #[test]
    fn parses_from_a_plain_string() {
        assert_eq!(
            "2024-01-31".parse::<DateTime>().unwrap().date().to_string(),
            "2024-01-31"
        );
        assert!("31-01-2024".parse::<DateTime>().is_err());
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse("yesterday").is_err());
    }

    #[test]
    fn roundtrips() {
        let t = parse("2020-01-02T03:04:05").unwrap();
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(serde_json::from_str::<DateTime>(&json).unwrap(), t);
    }
}
