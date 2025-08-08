//! Slang artifact file (.sip) type definition.

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use zip::{ZipArchive, ZipWriter, write::FileOptions};
use crate::bytecode::Chunk;

/// Represents a Slang compiled artifact file (.sip) with type safety and validation.
///
/// SIP (Slang Intermediate Program) files are compressed archives containing
/// compiled bytecode and metadata for Slang programs.
///
/// # Examples
/// ```rust
/// use slang_backend::SlangArtifactFile;
/// 
/// // Create for output
/// let output_path = SlangArtifactFile::create_output_path("hello.sl");
/// assert_eq!(output_path.to_str().unwrap(), "hello.sip");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlangArtifactFile {
    /// The file path (including the .sip extension)
    path: PathBuf,
}

impl SlangArtifactFile {
    /// The expected file extension for Slang artifact files
    pub const EXTENSION: &'static str = "sip";
    
    /// Create a new SlangArtifactFile with the given path.
    ///
    /// # Arguments
    /// * `path` - The file path (must have .sip extension)
    ///
    /// # Returns
    /// A new SlangArtifactFile instance
    ///
    /// # Panics
    /// Panics if the path doesn't have a .sip extension
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        Self::validate_extension(&path).expect("Invalid file extension");
        
        Self { path }
    }
    
    /// Create a SlangArtifactFile by validating an existing file path.
    ///
    /// # Arguments
    /// * `path` - The path to the .sip file
    ///
    /// # Returns
    /// A SlangArtifactFile instance or an error
    ///
    /// # Errors
    /// * Returns `InvalidExtension` if the file doesn't have a .sip extension
    /// * Returns `FileNotFound` if the file doesn't exist
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, SlangArtifactFileError> {
        let path = path.as_ref().to_path_buf();
        Self::validate_extension(&path)?;
        
        if !path.exists() {
            return Err(SlangArtifactFileError::FileNotFound {
                path: path.clone(),
            });
        }
        
        Ok(Self { path })
    }
    
    /// Create an output path for a compiled artifact based on a source file path.
    ///
    /// This takes a source file path (e.g., "hello.sl") and returns the corresponding
    /// artifact path (e.g., "hello.sip").
    ///
    /// # Arguments
    /// * `source_path` - The source file path
    ///
    /// # Returns
    /// A PathBuf with the .sip extension
    pub fn create_output_path<P: AsRef<Path>>(source_path: P) -> PathBuf {
        let source_path = source_path.as_ref();
        source_path.with_extension(Self::EXTENSION)
    }
    
    /// Get the file path.
    pub fn path(&self) -> &Path {
        &self.path
    }
    
    /// Get the file name (without directory path).
    pub fn file_name(&self) -> &str {
        self.path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown.sip")
    }
    
    /// Check if the artifact file exists on disk.
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
    
    /// Get the file size in bytes, if the file exists.
    pub fn size(&self) -> Result<u64, SlangArtifactFileError> {
        let metadata = fs::metadata(&self.path)
            .map_err(|e| SlangArtifactFileError::Io {
                source: e,
                path: self.path.clone(),
            })?;
        
        Ok(metadata.len())
    }
    
    /// Read the raw bytes from the artifact file.
    ///
    /// # Returns
    /// The raw file contents as bytes
    ///
    /// # Errors
    /// Returns IO errors if the file cannot be read
    pub fn read_bytes(&self) -> Result<Vec<u8>, SlangArtifactFileError> {
        fs::read(&self.path)
            .map_err(|e| SlangArtifactFileError::Io {
                source: e,
                path: self.path.clone(),
            })
    }
    
    /// Write bytes to the artifact file.
    ///
    /// # Arguments
    /// * `data` - The bytes to write
    ///
    /// # Errors
    /// Returns IO errors if the file cannot be written
    pub fn write_bytes(&self, data: &[u8]) -> Result<(), SlangArtifactFileError> {
        fs::write(&self.path, data)
            .map_err(|e| SlangArtifactFileError::Io {
                source: e,
                path: self.path.clone(),
            })
    }
    
    /// Delete the artifact file from disk.
    ///
    /// # Errors
    /// Returns IO errors if the file cannot be deleted
    pub fn delete(&self) -> Result<(), SlangArtifactFileError> {
        fs::remove_file(&self.path)
            .map_err(|e| SlangArtifactFileError::Io {
                source: e,
                path: self.path.clone(),
            })
    }
    
    /// Write a bytecode chunk to the artifact file as a compressed ZIP archive.
    ///
    /// The chunk will be serialized and stored as "bytecode.bin" within the ZIP archive
    /// using deflate compression for optimal file size.
    ///
    /// # Arguments
    /// * `chunk` - The bytecode chunk to write
    ///
    /// # Returns
    /// Ok(()) on success, or an error if the operation fails
    ///
    /// # Errors
    /// * Returns IO errors if the file cannot be created or written
    /// * Returns Zip errors if the archive creation fails
    /// * Returns Serialization errors if the chunk cannot be serialized
    pub fn write_chunk(&self, chunk: &Chunk) -> Result<(), SlangArtifactFileError> {
        let file = File::create(&self.path)
            .map_err(|e| SlangArtifactFileError::Io {
                source: e,
                path: self.path.clone(),
            })?;

        let mut zip = ZipWriter::new(file);
        let options = FileOptions::<()>::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        zip.start_file("bytecode.bin", options)
            .map_err(|e| SlangArtifactFileError::Zip {
                source: e,
                context: "Failed to create zip entry".to_string(),
                path: self.path.clone(),
            })?;

        // Serialize the chunk to a temporary buffer
        let mut cursor = std::io::Cursor::new(Vec::new());
        chunk
            .serialize(&mut cursor)
            .map_err(|e| SlangArtifactFileError::Serialization {
                source: Box::new(e),
                context: "Failed to serialize bytecode chunk".to_string(),
                path: self.path.clone(),
            })?;

        // Write the serialized data to the zip
        zip.write_all(&cursor.into_inner())
            .map_err(|e| SlangArtifactFileError::Io {
                source: e,
                path: self.path.clone(),
            })?;

        // Finalize the zip file
        zip.finish()
            .map_err(|e| SlangArtifactFileError::Zip {
                source: e,
                context: "Failed to finalize zip file".to_string(),
                path: self.path.clone(),
            })?;

        Ok(())
    }
    
    /// Read a bytecode chunk from the artifact file's ZIP archive.
    ///
    /// This method opens the .sip file as a ZIP archive and extracts the "bytecode.bin"
    /// entry, deserializing it back into a Chunk.
    ///
    /// # Returns
    /// The deserialized bytecode chunk, or an error if the operation fails
    ///
    /// # Errors
    /// * Returns IO errors if the file cannot be read
    /// * Returns Zip errors if the archive is invalid or cannot be read
    /// * Returns FileNotFound if "bytecode.bin" is missing from the archive
    /// * Returns Serialization errors if the chunk cannot be deserialized
    pub fn read_chunk(&self) -> Result<Chunk, SlangArtifactFileError> {
        let file = File::open(&self.path)
            .map_err(|e| SlangArtifactFileError::Io {
                source: e,
                path: self.path.clone(),
            })?;

        let mut archive = ZipArchive::new(file)
            .map_err(|e| SlangArtifactFileError::Zip {
                source: e,
                context: "Failed to read zip archive".to_string(),
                path: self.path.clone(),
            })?;

        let mut bytecode_file = archive.by_name("bytecode.bin")
            .map_err(|e| match e {
                zip::result::ZipError::FileNotFound => {
                    SlangArtifactFileError::MissingBytecode {
                        path: self.path.clone(),
                    }
                }
                other => SlangArtifactFileError::Zip {
                    source: other,
                    context: "Failed to access bytecode.bin from archive".to_string(),
                    path: self.path.clone(),
                }
            })?;

        // Read the bytecode data
        let mut buffer = Vec::new();
        bytecode_file.read_to_end(&mut buffer)
            .map_err(|e| SlangArtifactFileError::Io {
                source: e,
                path: self.path.clone(),
            })?;

        // Deserialize the chunk
        let mut cursor = std::io::Cursor::new(buffer);
        let chunk = Chunk::deserialize(&mut cursor)
            .map_err(|e| SlangArtifactFileError::Serialization {
                source: Box::new(e),
                context: "Failed to deserialize bytecode chunk".to_string(),
                path: self.path.clone(),
            })?;

        Ok(chunk)
    }
    
    /// List all entries in the ZIP archive.
    ///
    /// This method provides introspection into the contents of the .sip file,
    /// which can be useful for debugging or validation purposes.
    ///
    /// # Returns
    /// A vector of entry names in the ZIP archive
    ///
    /// # Errors
    /// * Returns IO errors if the file cannot be read
    /// * Returns Zip errors if the archive is invalid
    pub fn list_entries(&self) -> Result<Vec<String>, SlangArtifactFileError> {
        let file = File::open(&self.path)
            .map_err(|e| SlangArtifactFileError::Io {
                source: e,
                path: self.path.clone(),
            })?;

        let mut archive = ZipArchive::new(file)
            .map_err(|e| SlangArtifactFileError::Zip {
                source: e,
                context: "Failed to read zip archive".to_string(),
                path: self.path.clone(),
            })?;

        let mut entries = Vec::new();
        for i in 0..archive.len() {
            let file = archive.by_index(i)
                .map_err(|e| SlangArtifactFileError::Zip {
                    source: e,
                    context: format!("Failed to access entry at index {i}"),
                    path: self.path.clone(),
                })?;
            entries.push(file.name().to_string());
        }

        Ok(entries)
    }
    
    /// Validate that a path has the correct .sip extension.
    fn validate_extension(path: &Path) -> Result<(), SlangArtifactFileError> {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(Self::EXTENSION) => Ok(()),
            Some(other) => Err(SlangArtifactFileError::InvalidExtension {
                expected: Self::EXTENSION.to_string(),
                found: other.to_string(),
                path: path.to_path_buf(),
            }),
            None => Err(SlangArtifactFileError::MissingExtension {
                expected: Self::EXTENSION.to_string(),
                path: path.to_path_buf(),
            }),
        }
    }
}

/// Errors that can occur when working with SlangArtifactFile.
#[derive(Debug, thiserror::Error)]
pub enum SlangArtifactFileError {
    /// IO error when reading or writing the file
    #[error("IO error for file '{path}': {source}")]
    Io {
        #[source]
        source: io::Error,
        path: PathBuf,
    },
    
    /// Invalid file extension
    #[error("Invalid file extension for '{path}': expected '.{expected}', found '.{found}'")]
    InvalidExtension {
        expected: String,
        found: String,
        path: PathBuf,
    },
    
    /// Missing file extension
    #[error("Missing file extension for '{path}': expected '.{expected}'")]
    MissingExtension {
        expected: String,
        path: PathBuf,
    },
    
    /// File not found
    #[error("Artifact file not found: '{path}'")]
    FileNotFound {
        path: PathBuf,
    },
    
    /// ZIP archive related errors
    #[error("ZIP error for file '{path}': {context} - {source}")]
    Zip {
        #[source]
        source: zip::result::ZipError,
        context: String,
        path: PathBuf,
    },
    
    /// Serialization/deserialization errors
    #[error("Serialization error for file '{path}': {context} - {source}")]
    Serialization {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
        context: String,
        path: PathBuf,
    },
    
    /// Missing bytecode.bin entry in ZIP archive
    #[error("Missing bytecode.bin entry in artifact file: '{path}'")]
    MissingBytecode {
        path: PathBuf,
    },
}
