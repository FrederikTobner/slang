use crate::formatter::{TokenFormatter, PrettyFormatter, DebugFormatter};

/// Available output formats for token display
#[derive(Clone)]
pub enum TokenFormat {
    /// Colored formatted output using the existing TokenPrinter
    Pretty,
    /// Debug format using built-in Debug trait
    Debug,
}

impl TokenFormat {
    /// Create the appropriate formatter for this format
    pub fn create_formatter(&self) -> Box<dyn TokenFormatter> {
        match self {
            TokenFormat::Pretty => Box::new(PrettyFormatter),
            TokenFormat::Debug => Box::new(DebugFormatter),
        }
    }
}

impl std::str::FromStr for TokenFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pretty" => Ok(TokenFormat::Pretty),
            "debug" => Ok(TokenFormat::Debug),
            _ => Err(format!("Invalid format '{}'. Valid formats: pretty, debug", s)),
        }
    }
}

impl std::fmt::Display for TokenFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenFormat::Pretty => write!(f, "pretty"),
            TokenFormat::Debug => write!(f, "debug"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_format_from_str() {
        assert!(matches!(
            "pretty".parse::<TokenFormat>().unwrap(),
            TokenFormat::Pretty
        ));
        assert!(matches!(
            "debug".parse::<TokenFormat>().unwrap(),
            TokenFormat::Debug
        ));
        assert!("invalid".parse::<TokenFormat>().is_err());
    }

    #[test]
    fn test_token_format_display() {
        assert_eq!(TokenFormat::Pretty.to_string(), "pretty");
        assert_eq!(TokenFormat::Debug.to_string(), "debug");
    }
}
