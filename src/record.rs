use crate::{Format, NamedLogLevel};
use chrono::{DateTime, SecondsFormat, Utc};
use colored::Colorize;
use itertools::Itertools;
use std::convert::TryFrom;

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
    pub extras: serde_json::Map<String, serde_json::Value>,
}

impl<'a> LogRecord<'a> {
    pub fn format(&self, _format: Format) -> String {
        let level = format_level(self.level);
        let formatted = format!(
            "[{}]  {}: {}/{} on {}: {}{}",
            self.time.to_rfc3339_opts(SecondsFormat::Millis, true),
            level,
            self.name,
            self.process_identifier,
            self.hostname,
            self.message.cyan(),
            format_extras(&self.extras)
        );
        dbg!(&formatted);
        formatted
    }
}

pub fn format_level(level: u8) -> String {
    if let Some(level) = NamedLogLevel::try_from(level).ok() {
        match level {
            // Following the orignal bunyan here with the leading whitespace!
            NamedLogLevel::Fatal => "FATAL".reversed(),
            NamedLogLevel::Error => "ERROR".red(),
            NamedLogLevel::Warn => "WARN".magenta(),
            NamedLogLevel::Info => "INFO".cyan(),
            NamedLogLevel::Debug => "DEBUG".yellow(),
            NamedLogLevel::Trace => "TRACE".white(),
        }
        .to_string()
    } else {
        format!("LVL{}", level)
    }
}

pub fn format_extras(extra_fields: &serde_json::Map<String, serde_json::Value>) -> String {
    let mut details = Vec::new();
    let mut extras = Vec::new();
    for (key, value) in extra_fields {
        let stringified = if let serde_json::Value::String(s) = value {
            s.to_owned()
        } else {
            serde_json::to_string(&value).unwrap()
        };
        if stringified.contains("\n") || stringified.len() > 50 {
            details.push(indent(&format!("{}:{}", key, value)));
        } else {
            extras.push(format!("{}={}", key, stringified));
        }
    }
    let formatted_details = if details.len() > 0 {
        details.into_iter().join("\n    --\n").to_string()
    } else {
        "".into()
    };
    let formatted_extras = if extras.len() > 0 {
        format!(" ({})", extras.into_iter().join(","))
    } else {
        "".into()
    };
    format!("{}\n{}", formatted_extras, formatted_details)
}

pub fn indent(s: &str) -> String {
    format!("    {}", s.lines().join("\n    "))
}
