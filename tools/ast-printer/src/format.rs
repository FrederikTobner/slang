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
