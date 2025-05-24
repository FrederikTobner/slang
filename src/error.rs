use std::error::Error;
use std::fmt;
use std::io;

use crate::exit;

/// A custom error type for the Slang CLI
#[derive(Debug)]
pub enum SlangError {
    /// Error related to reading/writing files
    Io {
        source: io::Error,
        path: String,
        exit_code: exit::Code,
    },
    /// Error related to reading/writing ZIP files
    Zip {
        source: zip::result::ZipError,
        context: String,
        exit_code: exit::Code,
    },
    /// Error related to serialization/deserialization
    Serialization {
        source: Box<dyn std::error::Error + Send + Sync>,
        context: String,
        exit_code: exit::Code,
    },
    /// Generic error with custom message
    Generic {
        message: String,
        exit_code: exit::Code,
    },
}

impl SlangError {
    /// Get the exit code associated with this error
    pub fn exit_code(&self) -> exit::Code {
        match self {
            SlangError::Io { exit_code, .. } => *exit_code,
            SlangError::Zip { exit_code, .. } => *exit_code,
            SlangError::Serialization { exit_code, .. } => *exit_code,
            SlangError::Generic { exit_code, .. } => *exit_code,
        }
    }

    /// Convert from io::Error to SlangError with appropriate exit code and path
    ///
    /// ### Arguments
    /// * `error` - The io::Error to convert
    /// * `path` - The path associated with the error
    ///
    /// ### Returns
    /// A SlangError with the appropriate exit code and path
    pub fn from_io_error(error: io::Error, path: &str) -> Self {
        let exit_code = match error.kind() {
            io::ErrorKind::NotFound => exit::Code::NoInput,
            io::ErrorKind::PermissionDenied => exit::Code::NoPerm,
            _ => exit::Code::IoErr,
        };

        SlangError::Io {
            source: error,
            path: path.to_string(),
            exit_code,
        }
    }
}

impl fmt::Display for SlangError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlangError::Io { source, path, .. } => {
                write!(f, "Error reading file '{}': {}", path, source)
            }
            SlangError::Zip {
                source, context, ..
            } => {
                write!(f, "{}: {}", context, source)
            }
            SlangError::Serialization {
                source, context, ..
            } => {
                write!(f, "{}: {}", context, source)
            }
            SlangError::Generic { message, .. } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl Error for SlangError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SlangError::Io { source, .. } => Some(source),
            SlangError::Zip { source, .. } => Some(source),
            SlangError::Serialization { source, .. } => Some(source.as_ref()),
            SlangError::Generic { .. } => None,
        }
    }
}

/// Type alias for Result with SlangError as the error type
pub type SlangResult<T> = Result<T, SlangError>;
