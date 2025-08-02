use clap::{Parser, CommandFactory};
use bytecode_printer_lib::cli::Parser as CliParser;
use bytecode_printer_lib::format::BytecodeFormat;

const EXECUTABLE_NAME: &str = "bytecode-printer";

#[test] 
fn help() {
    // Test that help can be accessed without panicking
    let parser = CliParser::command();
    assert!(parser.get_name() == EXECUTABLE_NAME);
}

#[test]
fn with_file() {
    // Test parsing with a file argument
    let args = vec![EXECUTABLE_NAME, "test.sl"];
    let result = CliParser::try_parse_from(args);
    assert!(result.is_ok());
    
    let parser = result.unwrap();
    assert_eq!(parser.input, "test.sl");
    assert_eq!(parser.format, BytecodeFormat::Pretty); // default format
}

#[test]
fn with_format() {
    // Test parsing with format argument
    let args = vec![EXECUTABLE_NAME, "--format", "json", "test.sl"];
    let result = CliParser::try_parse_from(args);
    assert!(result.is_ok());
    
    let parser = result.unwrap();
    assert_eq!(parser.input, "test.sl");
    assert_eq!(parser.format, BytecodeFormat::Json);
}

#[test]
fn with_short_format() {
    // Test parsing with short format argument
    let args = vec![EXECUTABLE_NAME, "-f", "debug", "test.sl"];
    let result = CliParser::try_parse_from(args);
    assert!(result.is_ok());
    
    let parser = result.unwrap();
    assert_eq!(parser.input, "test.sl");
    assert_eq!(parser.format, BytecodeFormat::Debug);
}
