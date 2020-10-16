use crate::{Format, NamedLogLevel};
use chrono::{DateTime, Utc};
use std::convert::{Infallible, TryFrom};

#[derive(serde::Deserialize)]
pub struct LogRecord<'a> {
    /// This is the bunyan log format version. The log version is a single integer.
    /// It is meant to be 0 until version "1.0.0" of `node-bunyan` is released.
    /// Thereafter, starting with 1, this will be incremented if there is any backward incompatible
    /// change to the log record format.
    #[serde(rename = "v")]
    pub version: u8,
    /// See `LogLevel`
    pub level: u8,
    /// Name of the service/application emitting logs in bunyan format.
    pub name: &'a str,
    /// Name of the operating system host.
    pub hostname: &'a str,
    /// Process identifier.
    #[serde(rename = "pid")]
    pub process_identifier: u32,
    /// The time of the event captured by the log in [ISO 8601 extended format](http://en.wikipedia.org/wiki/ISO_8601).
    pub time: DateTime<Utc>,
    /// Log message.
    #[serde(rename = "msg")]
    pub message: &'a str,
    /// Any extra contextual piece of information in the log record.
    #[serde(flatten)]
    pub extras: serde_json::Value,
}

impl<'a> LogRecord<'a> {
    pub fn format(&self, format: Format) -> String {
        let level = format_level(self.level);
        format!(
            "[{}] {}: {}/{} on {}: {}",
            self.time, level, self.name, self.process_identifier, self.hostname, self.message
        )
    }
}

pub fn format_level(level: u8) -> String {
    if let Some(level) = NamedLogLevel::try_from(level).ok() {
        match level {
            NamedLogLevel::Fatal => "FATAL",
            NamedLogLevel::Error => "ERROR",
            NamedLogLevel::Warn => "WARN",
            NamedLogLevel::Info => "INFO",
            NamedLogLevel::Debug => "DEBUG",
            NamedLogLevel::Trace => "TRACE",
        }
        .into()
    } else {
        format!("LVL{}", level)
    }
}
