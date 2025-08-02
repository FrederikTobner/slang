use crate::formatter::{BytecodeFormatter, PrettyFormatter, DebugFormatter, JsonFormatter};

/// Available output formats for bytecode display
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BytecodeFormat {
    /// Pretty-printed format with instruction names and operands
    Pretty,
    /// Debug format showing raw bytes and detailed information
    Debug,
    /// JSON format for structured data exchange
    Json,
}

impl BytecodeFormat {
    /// Create the appropriate formatter for this format
    pub fn create_formatter(&self) -> Box<dyn BytecodeFormatter> {
        match self {
            BytecodeFormat::Pretty => Box::new(PrettyFormatter),
            BytecodeFormat::Debug => Box::new(DebugFormatter),
            BytecodeFormat::Json => Box::new(JsonFormatter),
        }
    }
}

impl std::str::FromStr for BytecodeFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pretty" => Ok(BytecodeFormat::Pretty),
            "debug" => Ok(BytecodeFormat::Debug),
            "json" => Ok(BytecodeFormat::Json),
            _ => Err(format!(
                "Invalid format '{}'. Valid formats: pretty, debug, json", 
                s
            )),
        }
    }
}

impl std::fmt::Display for BytecodeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BytecodeFormat::Pretty => write!(f, "pretty"),
            BytecodeFormat::Debug => write!(f, "debug"),
            BytecodeFormat::Json => write!(f, "json"),
        }
    }
}
