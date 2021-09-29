use crate::Result;
use chrono::{Datelike, Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// A canonical Gregorian calendar date in years 0001 through 9999.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Date(NaiveDate);

impl Date {
    pub fn today() -> Self {
        Self(chrono::Local::now().date_naive())
    }

    pub fn add_days(self, days: i64) -> Result<Self> {
        let duration = Duration::try_days(days).ok_or("day offset is out of range")?;
        let value = self.0.checked_add_signed(duration).ok_or("date overflow")?;
        if !(1..=9999).contains(&value.year()) {
            return Err("date must be between 0001-01-01 and 9999-12-31".into());
        }
        Ok(Self(value))
    }

    pub fn days_until(self, end: Self) -> i64 {
        (end.0 - self.0).num_days()
    }

    pub fn year(self) -> i32 {
        self.0.year()
    }

    pub fn month(self) -> u32 {
        self.0.month()
    }
}

impl FromStr for Date {
    type Err = String;
    fn from_str(value: &str) -> Result<Self> {
        let bytes = value.as_bytes();
        if bytes.len() != 10
            || bytes[4] != b'-'
            || bytes[7] != b'-'
            || bytes
                .iter()
                .enumerate()
                .any(|(i, b)| i != 4 && i != 7 && !b.is_ascii_digit())
        {
            return Err("expected date in YYYY-MM-DD format".into());
        }
        let parsed = NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|e| e.to_string())?;
        if !(1..=9999).contains(&parsed.year()) {
            return Err("year must be between 0001 and 9999".into());
        }
        Ok(Self(parsed))
    }
}
impl TryFrom<String> for Date {
    type Error = String;
    fn try_from(value: String) -> Result<Self> {
        value.parse()
    }
}
impl From<Date> for String {
    fn from(value: Date) -> Self {
        value.to_string()
    }
}
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
