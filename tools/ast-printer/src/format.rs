use crate::formatter::{AstFormatter, PrettyFormatter, JsonFormatter, CompactFormatter};

/// Available output formats for AST display
#[derive(Debug, Clone, PartialEq)]
pub enum AstFormat {
    /// Pretty-printed hierarchical format with indentation
    Pretty,
    /// JSON format for structured data exchange
    Json,
    /// Compact single-line format for quick overview
    Compact,
}

impl AstFormat {
    /// Create the appropriate formatter for this format
    pub fn create_formatter(&self) -> Box<dyn AstFormatter> {
        match self {
            AstFormat::Pretty => Box::new(PrettyFormatter),
            AstFormat::Json => Box::new(JsonFormatter),
            AstFormat::Compact => Box::new(CompactFormatter),
        }
    }
}

impl std::str::FromStr for AstFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pretty" => Ok(AstFormat::Pretty),
            "json" => Ok(AstFormat::Json),
            "compact" => Ok(AstFormat::Compact),
            _ => Err(format!(
                "Invalid format '{}'. Valid formats: pretty, json, compact", 
                s
            )),
        }
    }
}

impl std::fmt::Display for AstFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstFormat::Pretty => write!(f, "pretty"),
            AstFormat::Json => write!(f, "json"),
            AstFormat::Compact => write!(f, "compact"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_format_from_str() {
        assert!(matches!(
            "pretty".parse::<AstFormat>().unwrap(),
            AstFormat::Pretty
        ));
        assert!(matches!(
            "json".parse::<AstFormat>().unwrap(),
            AstFormat::Json
        ));
        assert!(matches!(
            "compact".parse::<AstFormat>().unwrap(),
            AstFormat::Compact
        ));
    }

    #[test]
    fn test_ast_format_from_str_case_insensitive() {
        assert!(matches!(
            "PRETTY".parse::<AstFormat>().unwrap(),
            AstFormat::Pretty
        ));
        assert!(matches!(
            "Json".parse::<AstFormat>().unwrap(),
            AstFormat::Json
        ));
    }

    #[test]
    fn test_ast_format_from_str_invalid() {
        assert!("invalid".parse::<AstFormat>().is_err());
        assert!("".parse::<AstFormat>().is_err());
    }

    #[test]
    fn test_ast_format_display() {
        assert_eq!(AstFormat::Pretty.to_string(), "pretty");
        assert_eq!(AstFormat::Json.to_string(), "json");
        assert_eq!(AstFormat::Compact.to_string(), "compact");
    }
}
