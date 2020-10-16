use std::str::FromStr;

/// Supported output formats.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Format {
    /// Prettified JSON.
    Long,
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "long" => Ok(Format::Long),
            _ => Err(anyhow::anyhow!(format!("Invalid format value: '{}'", s))),
        }
    }
}
