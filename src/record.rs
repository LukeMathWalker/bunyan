use chrono::{DateTime, Utc};

pub struct LogRecord<'a> {
    #[serde(rename = "v")]
    version: u8,
    level: u8,
    name: &'a str,
    hostname: &'a str,
    #[serde(rename = "pid")]
    process_identifier: u32,
    time: DateTime<Utc>,
    #[serde(rename = "msg")]
    message: &'a str,
    #[serde(flatten)]
    extras: serde_json::Value,
}
