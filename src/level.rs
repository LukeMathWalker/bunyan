use std::str::FromStr;

/// Bunyan log level.
/// Although "named" log levels are specified (see `NamedLogLevel`) arbitrary integer values are
/// accepted (e.g. 32).
pub struct LogLevel(u8);

impl FromStr for LogLevel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u8>() {
            Ok(level) => Ok(LogLevel(level)),
            Err(_) => match s.parse::<NamedLogLevel>() {
                Ok(level) => Ok(LogLevel(level as u8)),
                Err(_) => Err(anyhow::anyhow!(format!("Invalid level value: '{}'", s))),
            },
        }
    }
}

/// Canonical interpretation of different log levels.  
/// Although arbitrary integer values are accepted as log levels (see `LogLevel`) the usage of
/// named log levels is preferred.
#[repr(u8)]
pub enum NamedLogLevel {
    /// The service/app is going to stop or become unusable now.
    /// An operator should definitely look into this soon.
    Fatal = 60,
    /// Fatal for a particular request, but the service/app continues servicing other requests.
    /// An operator should look at this soon(ish).
    Error = 50,
    /// A note on something that should probably be looked at by an operator eventually.
    Warn = 40,
    /// Detail on regular operation.
    Info = 30,
    /// Anything else, i.e. too verbose to be included in "info" level.
    Debug = 20,
    /// Logging from external libraries used by your app or very detailed application logging.
    Trace = 10,
}

impl FromStr for NamedLogLevel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(NamedLogLevel::Trace),
            "debug" => Ok(NamedLogLevel::Debug),
            "info" => Ok(NamedLogLevel::Info),
            "warn" => Ok(NamedLogLevel::Warn),
            "error" => Ok(NamedLogLevel::Error),
            "fatal" => Ok(NamedLogLevel::Fatal),
            _ => Err(anyhow::anyhow!(format!("Invalid level value: '{}'", s))),
        }
    }
}
