use clap::{Parser, CommandFactory};
use token_printer_lib::cli::Parser as CliParser;
use token_printer_lib::format::TokenFormat;

const EXECUTABLE_NAME: &str = "token-printer";

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
    
    let parsed = result.unwrap();
    assert_eq!(parsed.input, "test.sl");
    assert!(matches!(parsed.format, TokenFormat::Pretty)); // default format
}

#[test]
fn with_debug_format() {
    // Test parsing with debug format
    let args = vec![EXECUTABLE_NAME, "--format", "debug", "test.sl"];
    let result = CliParser::try_parse_from(args);
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert_eq!(parsed.input, "test.sl");
    assert!(matches!(parsed.format, TokenFormat::Debug));
}
