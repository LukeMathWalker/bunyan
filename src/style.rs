use std::str::FromStr;

/// Supported output formats.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Format {
    /// Prettified JSON.
    Long,
    Short,
    Json,
    JsonN(u8),
    Bunyan,
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "long" => Ok(Format::Long),
            "short" => Ok(Format::Short),
            "json" => Ok(Format::Json),
            "bunyan" => Ok(Format::Bunyan),
            s => {
                if s.is_ascii() {
                    if let Some((prefix, len_str)) = s.split_at_checked(5) {
                        if prefix == "json-" {
                            if let Ok(len) = len_str.parse::<u8>() {
                                if len < 1 {
                                    return Ok(Format::Bunyan);
                                } else if len <= 10 {
                                    return Ok(Format::JsonN(len));
                                } else {
                                    return Ok(Format::JsonN(10));
                                }
                            }
                        }
                    };
                }

                Err(anyhow::anyhow!(format!("Invalid format value: '{}'", s)))
            }
        }
    }
}
