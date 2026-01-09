
// from https://dev.to/thiagomg/another-way-to-deserialise-datetime-in-rust-kja
use chrono::NaiveDate;  
use serde::{self, Deserialize, Deserializer, Serializer};  

const FORMAT: &str = "%Y-%m-%d";  

/// Transforms a `NaiveDate` into a String
#[allow(dead_code,clippy::missing_errors_doc)]
pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{  
    let s = date.format(FORMAT).to_string();
    serializer.serialize_str(&s)
}

/// Transforms a String into a `NaiveDate`
#[allow(clippy::missing_errors_doc)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>  
    where
        D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
}
