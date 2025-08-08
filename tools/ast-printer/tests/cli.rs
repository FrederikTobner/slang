mod test_utils;

use ast_printer_lib::format::AstFormat;
use ast_printer_lib::cli::parse_and_print_ast;
use std::io::Write;
use tempfile::NamedTempFile;
use test_utils::*;

#[test]
fn pretty_format_basic() {
    let (_temp_dir, file_path) = create_temp_slang_file(SAMPLE_SLANG_CODE)
        .expect("Failed to create temp file");
    
    let result = parse_and_print_ast(
        file_path.to_str().unwrap(), 
        AstFormat::Pretty
    );
    
    assert!(result.is_ok(), "Pretty format parsing should succeed");
}

#[test]
fn json_format_basic() {
    let (_temp_dir, file_path) = create_temp_slang_file(SIMPLE_EXPRESSION)
        .expect("Failed to create temp file");
    
    let result = parse_and_print_ast(
        file_path.to_str().unwrap(), 
        AstFormat::Json
    );
    
    assert!(result.is_ok(), "JSON format parsing should succeed");
}

#[test]
fn compact_format_basic() {
    let (_temp_dir, file_path) = create_temp_slang_file(SIMPLE_EXPRESSION)
        .expect("Failed to create temp file");
    
    let result = parse_and_print_ast(
        file_path.to_str().unwrap(), 
        AstFormat::Compact
    );
    
    assert!(result.is_ok(), "Compact format parsing should succeed");
}

#[test]
fn nonexistent_file() {
    let result = parse_and_print_ast("/nonexistent/file.sl", AstFormat::Pretty);
    
    assert!(result.is_err(), "Should fail for nonexistent file");
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("does not exist"), "Error should mention file doesn't exist");
}

#[test]
fn invalid_slang_syntax() {
    let invalid_code = "this is not valid slang syntax {{{ +++";
    let (_temp_dir, file_path) = create_temp_slang_file(invalid_code)
        .expect("Failed to create temp file");
    
    let result = parse_and_print_ast(
        file_path.to_str().unwrap(), 
        AstFormat::Pretty
    );
    
    assert!(result.is_err(), "Invalid syntax should result in an error");
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Failed to parse source code") || error_msg.contains("syntax error"),
        "Error message should indicate parsing failure, got: {}", 
        error_msg
    );
}

#[test]
fn empty_file() {
    let (_temp_dir, file_path) = create_temp_slang_file("")
        .expect("Failed to create temp file");
    
    let result = parse_and_print_ast(
        file_path.to_str().unwrap(), 
        AstFormat::Pretty
    );
    
    assert!(result.is_ok(), "Empty file should parse successfully");
}

#[test]
fn all_formats_with_complex_code() {
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
fn file_extension_warning() {
    let mut temp_file = NamedTempFile::new()
        .expect("Failed to create temp file");
    
    write!(temp_file, "{}", SIMPLE_EXPRESSION)
        .expect("Failed to write to temp file");
    
    let result = parse_and_print_ast(
        temp_file.path().to_str().unwrap(), 
        AstFormat::Pretty
    );
    
    assert!(result.is_ok(), "Should parse file even without .sl extension");
}

#[test]
fn multiple_invalid_syntax_cases() {
    use test_utils::*;
    
    let invalid_cases = [
        ("unclosed_brace", INVALID_SYNTAX_UNCLOSED_BRACE),
        ("unexpected_token", INVALID_SYNTAX_UNEXPECTED_TOKEN), 
        ("missing_semicolon", INVALID_SYNTAX_MISSING_SEMICOLON),
        ("completely_invalid", COMPLETELY_INVALID),
    ];
    
    for (case_name, invalid_code) in invalid_cases {
        let (_temp_dir, file_path) = create_temp_slang_file(invalid_code)
            .expect("Failed to create temp file");
        
        let result = parse_and_print_ast(
            file_path.to_str().unwrap(), 
            AstFormat::Pretty
        );
        
        assert!(
            result.is_err(), 
            "Invalid syntax case '{}' should result in an error", 
            case_name
        );
    }
}

#[test]
fn error_message_contains_relevant_info() {
    let (_temp_dir, file_path) = create_temp_slang_file("invalid syntax")
        .expect("Failed to create temp file");
    
    let result = parse_and_print_ast(
        file_path.to_str().unwrap(), 
        AstFormat::Pretty
    );
    
    assert!(result.is_err(), "Should fail for invalid syntax");
    let error_msg = result.unwrap_err().to_string();
    
    assert!(
        error_msg.len() > 10,
        "Error message should be substantial, got: '{}'", 
        error_msg
    );
}
