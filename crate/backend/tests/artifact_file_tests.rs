use slang_backend::{SlangArtifactFile, SlangArtifactFileError};
use slang_backend::bytecode::{Chunk, OpCode};
use slang_backend::value::Value;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_new_valid_extension() {
    let artifact = SlangArtifactFile::new("test.sip");
    assert_eq!(artifact.file_name(), "test.sip");
}

#[test]
#[should_panic(expected = "Invalid file extension")]
fn test_new_invalid_extension() {
    SlangArtifactFile::new("test.bin");
}

#[test]
fn test_create_output_path() {
    let output_path = SlangArtifactFile::create_output_path("hello.sl");
    assert_eq!(output_path.to_str().unwrap(), "hello.sip");
    
    let output_path = SlangArtifactFile::create_output_path("/path/to/program.sl");
    assert_eq!(output_path.to_str().unwrap(), "/path/to/program.sip");
}

#[test]
fn test_from_path_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.sip");
    
    fs::write(&file_path, b"dummy bytecode")?;
    
    let artifact = SlangArtifactFile::from_path(&file_path)?;
    assert_eq!(artifact.file_name(), "test.sip");
    assert!(artifact.exists());
    
    Ok(())
}

#[test]
fn test_from_path_nonexistent_file() {
    let result = SlangArtifactFile::from_path("nonexistent.sip");
    assert!(matches!(result, Err(SlangArtifactFileError::FileNotFound { .. })));
}

#[test]
fn test_from_path_invalid_extension() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.bin");
    
    fs::write(&file_path, b"content").unwrap();
    
    let result = SlangArtifactFile::from_path(&file_path);
    assert!(matches!(result, Err(SlangArtifactFileError::InvalidExtension { .. })));
}

#[test]
fn test_read_write_bytes() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.sip");
    
    let artifact = SlangArtifactFile::new(&file_path);
    let test_data = b"test bytecode data";
    
    artifact.write_bytes(test_data)?;
    
    let read_data = artifact.read_bytes()?;
    assert_eq!(read_data, test_data);
    
    assert_eq!(artifact.size()?, test_data.len() as u64);
    
    Ok(())
}

#[test]
fn test_delete() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.sip");
    
    let artifact = SlangArtifactFile::new(&file_path);
    artifact.write_bytes(b"test data")?;
    
    assert!(artifact.exists());
    
    artifact.delete()?;
    
    assert!(!artifact.exists());
    
    Ok(())
}

#[test]
fn test_write_read_chunk() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.sip");
    
    // Create a test chunk
    let mut chunk = Chunk::new();
    chunk.add_constant(Value::I32(42));
    chunk.write_op(OpCode::Constant, 1);
    chunk.write_byte(0, 1);
    
    let artifact = SlangArtifactFile::new(&file_path);
    
    // Write the chunk to the artifact file
    artifact.write_chunk(&chunk)?;
    
    // Verify the file exists and is non-empty
    assert!(artifact.exists());
    assert!(artifact.size()? > 0);
    
    // Read the chunk back
    let read_chunk = artifact.read_chunk()?;
    
    // Verify the chunk content matches
    assert_eq!(read_chunk.constants.len(), chunk.constants.len());
    assert_eq!(read_chunk.code.len(), chunk.code.len());
    
    Ok(())
}

#[test]
fn test_list_entries() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.sip");
    
    // Create a test chunk
    let chunk = Chunk::new();
    let artifact = SlangArtifactFile::new(&file_path);
    
    // Write the chunk to create a ZIP file
    artifact.write_chunk(&chunk)?;
    
    // List entries in the ZIP archive
    let entries = artifact.list_entries()?;
    
    // Should contain bytecode.bin
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0], "bytecode.bin");
    
    Ok(())
}

#[test]
fn test_read_chunk_missing_bytecode() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("empty.sip");
    
    // Create an empty ZIP file
    use zip::ZipWriter;
    let file = std::fs::File::create(&file_path)?;
    let zip = ZipWriter::new(file);
    zip.finish()?;
    
    let artifact = SlangArtifactFile::new(&file_path);
    
    // Try to read a chunk from an empty ZIP
    let result = artifact.read_chunk();
    assert!(matches!(result, Err(SlangArtifactFileError::MissingBytecode { .. })));
    
    Ok(())
}

#[test]
fn test_read_chunk_invalid_zip() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("invalid.sip");
    
    // Create a file with invalid ZIP content
    std::fs::write(&file_path, b"not a zip file").unwrap();
    
    let artifact = SlangArtifactFile::new(&file_path);
    
    // Try to read a chunk from invalid ZIP
    let result = artifact.read_chunk();
    assert!(matches!(result, Err(SlangArtifactFileError::Zip { .. })));
}
