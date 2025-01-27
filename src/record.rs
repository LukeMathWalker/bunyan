use crate::{Format, NamedLogLevel};
use chrono::{DateTime, SecondsFormat, Utc};
use colored::Colorize;
use itertools::Itertools;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::Serializer;
use std::borrow::Cow;
use std::convert::TryFrom;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LogRecord<'a> {
    /// This is the bunyan log format version. The log version is a single integer.
    /// It is meant to be 0 until version "1.0.0" of `node-bunyan` is released.
    /// Thereafter, starting with 1, this will be incremented if there is any backward incompatible
    /// change to the log record format.
    #[serde(rename = "v")]
    #[allow(dead_code)]
    pub version: u8,
    /// Name of the service/application emitting logs in bunyan format.
    pub name: &'a str,
    /// Log message.
    #[serde(rename = "msg")]
    pub message: Cow<'a, str>,
    /// See `LogLevel`
    pub level: u8,
    /// Name of the operating system host.
    pub hostname: &'a str,
    /// Process identifier.
    #[serde(rename = "pid")]
    pub process_identifier: u32,
    /// The time of the event captured by the log in [ISO 8601 extended format](http://en.wikipedia.org/wiki/ISO_8601).
    pub time: DateTime<Utc>,
    /// Any extra contextual piece of information in the log record.
    #[serde(flatten)]
    pub extras: serde_json::Map<String, serde_json::Value>,
}

impl<'a> LogRecord<'a> {
    pub fn format(&self, format: Format) -> String {
        let level = format_level(self.level);
        match format {
            Format::Long => {
                format!(
                    "[{}] {}: {}/{} on {}: {}{}",
                    self.time.to_rfc3339_opts(SecondsFormat::Millis, true),
                    level,
                    self.name,
                    self.process_identifier,
                    self.hostname,
                    self.message.cyan(),
                    format_extras(&self.extras)
                )
            }
            Format::Short => {
                format!(
                    "{} {} {}: {}{}",
                    self.time.format("%H:%M:%S%.3fZ"),
                    level,
                    self.name,
                    self.message.cyan(),
                    format_extras(&self.extras)
                )
            }
            Format::Json => serde_json::to_string_pretty(&self).expect("This should not happen"),
            Format::JsonN(l) => {
                let indent = " ".repeat(l.into());
                let value = serde_json::to_value(&self).expect("This should not happen");
                json_to_indented_string(&value, &indent)
            }
            Format::Bunyan => format!(
                "{}\n",
                serde_json::to_string(&self).expect("This should not happen")
            ),
        }
    }
}

pub fn format_level(level: u8) -> String {
    if let Ok(level) = NamedLogLevel::try_from(level) {
        match level {
            // Making sure all levels are 5 characters
            NamedLogLevel::Fatal => "FATAL".reversed(),
            NamedLogLevel::Error => "ERROR".red(),
            NamedLogLevel::Warn => " WARN".magenta(),
            NamedLogLevel::Info => " INFO".cyan(),
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
            // Preserve strings unless they contain whitespaces/are empty
            // In that case, we want surrounding quotes.
            if s.contains(' ') || s.is_empty() {
                format!("\"{}\"", s)
            } else {
                s.to_owned()
            }
        } else {
            json_to_indented_string(value, "  ")
        };

        if stringified.contains('\n') || stringified.len() > 50 {
            if let serde_json::Value::String(s) = value {
                details.push(indent(&format!("{}: {}", key.bold(), s)));
            } else {
                details.push(indent(&format!("{}: {}", key.bold(), stringified)));
            }
        } else {
            extras.push(format!("{}={}", key.bold(), stringified));
        }
    }
    let formatted_details = if !details.is_empty() {
        format!("{}\n", details.into_iter().join("\n    --\n"))
    } else {
        "".into()
    };
    let formatted_extras = if !extras.is_empty() {
        format!(" ({})", extras.into_iter().join(","))
    } else {
        "".into()
    };
    format!("{}\n{}", formatted_extras, formatted_details)
}

/// Serialize a JSON value to a string using the specified indentation.
///
/// It mimics the implementation of `serde_json::to_string_pretty`.
fn json_to_indented_string(value: &serde_json::Value, indent: &str) -> String {
    let mut writer = Vec::with_capacity(128);
    let formatter = PrettyFormatter::with_indent(indent.as_bytes());
    let mut serializer = Serializer::with_formatter(&mut writer, formatter);
    value.serialize(&mut serializer).unwrap();
    unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(writer)
    }
}

pub fn indent(s: &str) -> String {
    format!("    {}", s.lines().join("\n    "))
}
