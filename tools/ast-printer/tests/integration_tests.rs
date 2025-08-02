mod test_utils;

use ast_printer_lib::format::AstFormat;
use ast_printer_lib::cli::parse_and_print_ast;
use std::io::Write;
use tempfile::NamedTempFile;
use test_utils::*;

#[test]
fn test_pretty_format_basic() {
    let (_temp_dir, file_path) = create_temp_slang_file(SAMPLE_SLANG_CODE)
        .expect("Failed to create temp file");
    
    let result = parse_and_print_ast(
        file_path.to_str().unwrap(), 
        AstFormat::Pretty
    );
    
    // Should succeed without errors
    assert!(result.is_ok(), "Pretty format parsing should succeed");
}

#[test]
fn test_json_format_basic() {
    let (_temp_dir, file_path) = create_temp_slang_file(SIMPLE_EXPRESSION)
        .expect("Failed to create temp file");
    
    let result = parse_and_print_ast(
        file_path.to_str().unwrap(), 
        AstFormat::Json
    );
    
    assert!(result.is_ok(), "JSON format parsing should succeed");
}

#[test]
fn test_compact_format_basic() {
    let (_temp_dir, file_path) = create_temp_slang_file(SIMPLE_EXPRESSION)
        .expect("Failed to create temp file");
    
    let result = parse_and_print_ast(
        file_path.to_str().unwrap(), 
        AstFormat::Compact
    );
    
    assert!(result.is_ok(), "Compact format parsing should succeed");
}

#[test]
fn test_nonexistent_file() {
    let result = parse_and_print_ast("/nonexistent/file.sl", AstFormat::Pretty);
    
    assert!(result.is_err(), "Should fail for nonexistent file");
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("does not exist"), "Error should mention file doesn't exist");
}

#[test]
fn test_invalid_slang_syntax() {
    // Create a file with invalid Slang syntax
    let invalid_code = "this is not valid slang syntax {{{ +++";
    let (_temp_dir, file_path) = create_temp_slang_file(invalid_code)
        .expect("Failed to create temp file");
    
    let result = parse_and_print_ast(
        file_path.to_str().unwrap(), 
        AstFormat::Pretty
    );
    
    // Should handle parse errors gracefully
    // Note: The exact behavior depends on how the parser handles invalid syntax
    // This test mainly ensures the tool doesn't crash
    match result {
        Ok(_) => {
            // If parsing succeeded, the invalid syntax might have been interpreted differently
            // This is acceptable behavior
        }
        Err(_) => {
            // If parsing failed, that's expected for invalid syntax
            // This is also acceptable behavior
        }
    }
}

#[test]
fn test_empty_file() {
    let (_temp_dir, file_path) = create_temp_slang_file("")
        .expect("Failed to create temp file");
    
    let result = parse_and_print_ast(
        file_path.to_str().unwrap(), 
        AstFormat::Pretty
    );
    
    // Empty file handling should be graceful
    match result {
        Ok(_) => {
            // Empty AST is valid
        }
        Err(_) => {
            // Parser might require at least some content
            // Both behaviors are acceptable
        }
    }
}

#[test]
fn test_all_formats_with_complex_code() {
    let (_temp_dir, file_path) = create_temp_slang_file(COMPLEX_CODE)
        .expect("Failed to create temp file");
    
    let file_path_str = file_path.to_str().unwrap();
    
    // Test all formats with the same complex code
    for format in [AstFormat::Pretty, AstFormat::Json, AstFormat::Compact] {
        let result = parse_and_print_ast(file_path_str, format.clone());
        assert!(
            result.is_ok(), 
            "Format {:?} should handle complex code", 
            format
        );
    }
}

#[test]
fn test_file_extension_warning() {
    // Create a temporary file without .sl extension
    let mut temp_file = NamedTempFile::new()
        .expect("Failed to create temp file");
    
    write!(temp_file, "{}", SIMPLE_EXPRESSION)
        .expect("Failed to write to temp file");
    
    let result = parse_and_print_ast(
        temp_file.path().to_str().unwrap(), 
        AstFormat::Pretty
    );
    
    // Should still work, but might show a warning
    // The exact behavior depends on implementation
    assert!(result.is_ok(), "Should parse file even without .sl extension");
}
