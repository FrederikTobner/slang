use std::error::Error;
use std::fmt;
use std::io;
use slang_backend::SlangArtifactFileError;
use slang_compilation_pipeline::SourceFileError;

use crate::exit;

/// A custom error type for the Slang CLI
#[derive(Debug)]
pub enum CliError {
    /// Error related to reading/writing files
    Io {
        source: io::Error,
        path: String,
        exit_code: exit::Code,
    },
    /// Generic error with custom message
    Generic {
        message: String,
        exit_code: exit::Code,
    },
}

/// Type alias for Result with SlangError as the error type
pub type CliResult<T> = Result<T, CliError>;

impl CliError {
    /// Get the exit code associated with this error
    pub fn exit_code(&self) -> exit::Code {
        match self {
            CliError::Io { exit_code, .. } => *exit_code,
            CliError::Generic { exit_code, .. } => *exit_code,
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

        CliError::Io {
            source: error,
            path: path.to_string(),
            exit_code,
        }
    }
}

impl fmt::Display for CliError {
    /// Formats the error for display
    /// ### Arguments
    /// * `f` - The formatter to write the error to
    ///
    /// ### Returns
    /// A Result indicating success or failure of formatting
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Io { source, path, .. } => {
                write!(f, "Error reading file '{path}': {source}")
            }
            CliError::Generic { message, .. } => {
                write!(f, "{message}")
            }
        }
    }
}

impl Error for CliError {
    /// Get the source error if available
    ///
    /// ### Returns
    ///  An Option containing the source error if it exists, otherwise None
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CliError::Io { source, .. } => Some(source),
            CliError::Generic { .. } => None,
        }
    }
}

impl From<SourceFileError> for CliError {
    fn from(err: SourceFileError) -> Self {
        match err {
            SourceFileError::InvalidExtension { expected, found } => CliError::Generic {
                message: match found {
                    Some(ext) => format!("Invalid file extension '{ext}'. Expected '{expected}'"),
                    None => format!("Missing file extension. Expected '{expected}'"),
                },
                exit_code: exit::Code::Usage,
            },
            SourceFileError::Io(msg) => CliError::Generic {
                message: format!("File I/O error: {msg}"),
                exit_code: exit::Code::IoErr,
            },
        }
    }
}

impl From<SlangArtifactFileError> for CliError {
    fn from(err: SlangArtifactFileError) -> Self {
         match err {
        slang_backend::SlangArtifactFileError::Io { source, path } => {
            let exit_code = if source.kind() == std::io::ErrorKind::PermissionDenied {
                exit::Code::NoPerm
            } else {
                exit::Code::CantCreat
            };
            CliError::Io {
                source,
                path: path.as_path().to_str().unwrap_or("unknown").to_string(),
                exit_code,
            }
        },
        slang_backend::SlangArtifactFileError::Zip { source, context, .. } => CliError::Generic {
            message: format!("ZIP error: {context} - {source}"),
            exit_code: exit::Code::IoErr,
        },
        slang_backend::SlangArtifactFileError::Serialization { source, context, .. } => CliError::Generic {
            message: format!("Serialization error: {context} - {source}"),
            exit_code: exit::Code::Software,
        },
        other => CliError::Generic {
            message: format!("Failed to write bytecode: {other}"),
            exit_code: exit::Code::Software,
        },
    }
}
}
