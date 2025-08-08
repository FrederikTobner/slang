use slang_compilation_pipeline::{SlangSourceFile, SourceFileError};
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn new_valid_extension() {
    let source_file = SlangSourceFile::new("test.sl", "let x = 42;".to_string()).unwrap();
    assert_eq!(source_file.file_name(), "test.sl");
    assert_eq!(source_file.content(), "let x = 42;");
}

#[test]
fn new_invalid_extension() {
    let result = SlangSourceFile::new("test.txt", "let x = 42;".to_string());
    assert!(result.is_err());
    match result.unwrap_err() {
        SourceFileError::InvalidExtension { expected, found } => {
            assert_eq!(expected, "sl");
            assert_eq!(found, Some("txt".to_string()));
        }
        _ => panic!("Expected InvalidExtension error"),
    }
}

#[test]
fn from_path() -> Result<(), SourceFileError> {
    let temp_file = NamedTempFile::new().map_err(SourceFileError::from)?;
    let temp_path = temp_file.path();

    // Create a temporary .sl file
    let sl_path = temp_path.with_extension("sl");
    let content = "let x = 42;\nlet y = x + 1;";
    fs::write(&sl_path, content).map_err(SourceFileError::from)?;

    let source_file = SlangSourceFile::from_path(&sl_path)?;
    assert_eq!(source_file.content(), content);
    assert!(source_file.file_name().ends_with(".sl"));

    Ok(())
}

#[test]
fn from_path_invalid_extension() {
    let temp_file = NamedTempFile::new().unwrap();
    let result = SlangSourceFile::from_path(temp_file.path());
    assert!(result.is_err());
    match result.unwrap_err() {
        SourceFileError::InvalidExtension { .. } => {
            // Expected
        }
        _ => panic!("Expected InvalidExtension error"),
    }
}

#[test]
fn save() -> Result<(), SourceFileError> {
    let temp_dir = tempfile::tempdir().map_err(SourceFileError::from)?;
    let file_path = temp_dir.path().join("test.sl");

    let source_file = SlangSourceFile::new(&file_path, "let x = 42;".to_string())?;
    source_file.save().map_err(SourceFileError::from)?;

    let content = fs::read_to_string(&file_path).map_err(SourceFileError::from)?;
    assert_eq!(content, "let x = 42;");

    Ok(())
}
