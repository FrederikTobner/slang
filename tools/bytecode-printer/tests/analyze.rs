use bytecode_printer_lib::analyze_bytecode;
use bytecode_printer_lib::format::BytecodeFormat;
use slang_backend::SlangArtifactFile;
use slang_backend::bytecode::{Chunk, OpCode};
use slang_backend::value::Value;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

const VALID_PROGRAM: &str = r#"
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
print_value(add(1, 2));
"#;

fn write_file(name: &str, content: &str) -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join(name);
    fs::write(&file_path, content).expect("Failed to write file");
    (temp_dir, file_path)
}

fn error_message(result: Result<(), Box<dyn std::error::Error>>) -> String {
    result.expect_err("Expected analysis to fail").to_string()
}

#[test]
fn source_file_in_every_format() {
    // Arrange
    let (_temp_dir, path) = write_file("program.sl", VALID_PROGRAM);

    for format in [
        BytecodeFormat::Pretty,
        BytecodeFormat::Debug,
        BytecodeFormat::Json,
    ] {
        // Act
        let result = analyze_bytecode(path.to_str().unwrap(), format.clone(), false);

        // Assert
        assert!(result.is_ok(), "{format} format should succeed");
    }
}

#[test]
fn source_file_verbose() {
    // Arrange
    let (_temp_dir, path) = write_file("program.sl", VALID_PROGRAM);

    // Act
    let result = analyze_bytecode(path.to_str().unwrap(), BytecodeFormat::Pretty, true);

    // Assert
    assert!(result.is_ok());
}

#[test]
fn file_without_extension_is_treated_as_source() {
    // Arrange
    let (_temp_dir, path) = write_file("program", VALID_PROGRAM);

    // Act
    let result = analyze_bytecode(path.to_str().unwrap(), BytecodeFormat::Json, false);

    // Assert
    assert!(result.is_ok());
}

#[test]
fn compiled_sip_file() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("program.sip");
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(Value::I64(42));
    chunk.write_op(OpCode::Constant, 1);
    chunk.write_byte(constant as u8, 1);
    chunk.write_op(OpCode::Return, 1);
    SlangArtifactFile::new(&path).write_chunk(&chunk).unwrap();

    // Act
    let result = analyze_bytecode(path.to_str().unwrap(), BytecodeFormat::Pretty, true);

    // Assert
    assert!(result.is_ok());
}

#[test]
fn invalid_sip_file() {
    // Arrange
    let (_temp_dir, path) = write_file("broken.sip", "not a zip archive");

    // Act
    let result = analyze_bytecode(path.to_str().unwrap(), BytecodeFormat::Pretty, false);

    // Assert
    assert!(error_message(result).contains("Failed to"));
}

#[test]
fn nonexistent_file() {
    // Act
    let result = analyze_bytecode("/nonexistent/file.sl", BytecodeFormat::Pretty, false);

    // Assert
    assert!(error_message(result).contains("does not exist"));
}

#[test]
fn directory_instead_of_file() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();

    // Act
    let result = analyze_bytecode(
        temp_dir.path().to_str().unwrap(),
        BytecodeFormat::Pretty,
        false,
    );

    // Assert
    assert!(error_message(result).contains("is not a file"));
}

#[test]
fn unsupported_extension() {
    // Arrange
    let (_temp_dir, path) = write_file("program.txt", VALID_PROGRAM);

    // Act
    let result = analyze_bytecode(path.to_str().unwrap(), BytecodeFormat::Pretty, false);

    // Assert
    assert!(error_message(result).contains("Unsupported file extension '.txt'"));
}

#[test]
fn source_with_compilation_errors() {
    // Arrange
    let (_temp_dir, path) = write_file("broken.sl", "let x: i32 = ;");

    // Act
    let result = analyze_bytecode(path.to_str().unwrap(), BytecodeFormat::Pretty, true);

    // Assert
    assert!(error_message(result).contains("Failed to compile source code to bytecode"));
}
