//! Slang source file (.sl) type definition.

use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::fmt;

/// Error type for source file validation and creation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceFileError {
    /// Invalid file extension
    InvalidExtension {
        expected: String,
        found: Option<String>,
    },
    /// IO error during file operations
    Io(String),
}

impl fmt::Display for SourceFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceFileError::InvalidExtension { expected, found } => {
                match found {
                    Some(ext) => write!(f, "Invalid file extension '{ext}'. Expected '.{expected}'"),
                    None => write!(f, "Missing file extension. Expected '.{expected}'"),
                }
            }
            SourceFileError::Io(msg) => write!(f, "IO error: {msg}"),
        }
    }
}

impl std::error::Error for SourceFileError {}

impl From<io::Error> for SourceFileError {
    fn from(error: io::Error) -> Self {
        SourceFileError::Io(error.to_string())
    }
}

/// Represents a Slang source file (.sl) with type safety and validation.
///
/// This type ensures that files with the correct extension are used
/// and provides convenient access to both the file path and content.
///
/// # Examples
/// ```rust
/// use slang_compilation_pipeline::SlangSourceFile;
/// 
/// // Create from content and path
/// let source_file = SlangSourceFile::new("hello.sl", "let x = 42;".to_string()).unwrap();
/// 
/// // Access the content and metadata
/// println!("File: {}", source_file.file_name());
/// println!("Content: {}", source_file.content());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlangSourceFile {
    /// The file path (including the .sl extension)
    path: PathBuf,
    /// The source code content
    content: String,
}

impl SlangSourceFile {
    /// The expected file extension for Slang source files
    pub const EXTENSION: &'static str = "sl";
    
    /// Create a new SlangSourceFile with the given path and content.
    ///
    /// # Arguments
    /// * `path` - The file path (must have .sl extension)
    /// * `content` - The source code content
    ///
    /// # Returns
    /// Result containing a new SlangSourceFile instance or a SourceFileError
    ///
    /// # Errors
    /// Returns SourceFileError::InvalidExtension if the path doesn't have a .sl extension
    pub fn new<P: AsRef<Path>>(path: P, content: String) -> Result<Self, SourceFileError> {
        let path = path.as_ref().to_path_buf();
        Self::validate_extension(&path)?;
        
        Ok(Self { path, content })
    }
    
    /// Create a SlangSourceFile for development tools and utilities.
    /// 
    /// This constructor is intended for use by development tools (like AST printers,
    /// token printers, etc.) that need to process Slang source code from files that
    /// may not have the standard .sl extension (temporary files, stdin, etc.).
    /// 
    /// For production compilation, use `new()` or `from_path()` instead.
    ///
    /// # Arguments
    /// * `path` - The path to the file (any extension allowed)
    /// * `content` - The source code content
    ///
    /// # Returns
    /// A new SlangSourceFile instance
    pub fn for_tooling<P: AsRef<Path>>(path: P, content: String) -> Self {
        let path = path.as_ref().to_path_buf();
        Self { path, content }
    }
    
    /// Create a SlangSourceFile by reading from a file path.
    ///
    /// # Arguments
    /// * `path` - The path to the .sl file
    ///
    /// # Returns
    /// A SlangSourceFile instance or a SourceFileError
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, SourceFileError> {
        let path = path.as_ref().to_path_buf();
        Self::validate_extension(&path)?;
        
        let content = fs::read_to_string(&path)?;
        Ok(Self { path, content })
    }
    
    /// Get the file path as a string.
    ///
    /// # Returns
    /// The file path as a string slice
    pub fn file_path(&self) -> &str {
        self.path.to_str().unwrap_or("")
    }
    
    /// Get the file name (including extension).
    ///
    /// # Returns
    /// The file name as a string slice
    pub fn file_name(&self) -> &str {
        self.path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
    }
    
    /// Get the source code content.
    ///
    /// # Returns
    /// The source code content as a string slice
    pub fn content(&self) -> &str {
        &self.content
    }
    
    /// Get the source code content as a mutable reference.
    ///
    /// # Returns
    /// A mutable reference to the source code content
    pub fn content_mut(&mut self) -> &mut String {
        &mut self.content
    }
    
    /// Save the source file to disk.
    ///
    /// # Returns
    /// An IO result indicating success or failure
    pub fn save(&self) -> io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.path, &self.content)
    }
    
    /// Validate that the given path has the correct .sl extension.
    ///
    /// # Arguments
    /// * `path` - The path to validate
    ///
    /// # Returns
    /// Ok(()) if valid, Err with SourceFileError if invalid
    fn validate_extension(path: &Path) -> Result<(), SourceFileError> {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) if ext == Self::EXTENSION => Ok(()),
            Some(ext) => Err(SourceFileError::InvalidExtension {
                expected: Self::EXTENSION.to_string(),
                found: Some(ext.to_string()),
            }),
            None => Err(SourceFileError::InvalidExtension {
                expected: Self::EXTENSION.to_string(),
                found: None,
            }),
        }
    }
}
