use chrono::{DateTime, Utc};

#[derive(serde::Deserialize)]
pub struct LogRecord<'a> {
    /// This is the bunyan log format version. The log version is a single integer.
    /// It is meant to be 0 until version "1.0.0" of `node-bunyan` is released.
    /// Thereafter, starting with 1, this will be incremented if there is any backward incompatible
    /// change to the log record format.
    #[serde(rename = "v")]
    version: u8,
    /// See `LogLevel`
    level: u8,
    /// Name of the service/application emitting logs in bunyan format.
    name: &'a str,
    /// Name of the operating system host.
    hostname: &'a str,
    /// Process identifier.
    #[serde(rename = "pid")]
    process_identifier: u32,
    /// The time of the event captured by the log in [ISO 8601 extended format](http://en.wikipedia.org/wiki/ISO_8601).
    time: DateTime<Utc>,
    /// Log message.
    #[serde(rename = "msg")]
    message: &'a str,
    /// Any extra contextual piece of information in the log record.
    #[serde(flatten)]
    extras: serde_json::Value,
}
