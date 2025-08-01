use clap::{Parser, CommandFactory};
use slang_tokens::cli::Parser as CliParser;
use slang_tokens::format::TokenFormat;

#[test]
fn test_token_format_from_str() {
    assert!(matches!(
        "pretty".parse::<TokenFormat>().unwrap(),
        TokenFormat::Pretty
    ));
    assert!(matches!(
        "debug".parse::<TokenFormat>().unwrap(),
        TokenFormat::Debug
    ));
    assert!("invalid".parse::<TokenFormat>().is_err());
}

#[test]
fn test_token_format_display() {
    assert_eq!(TokenFormat::Pretty.to_string(), "pretty");
    assert_eq!(TokenFormat::Debug.to_string(), "debug");
}

#[test] 
fn test_cli_parser_help() {
    // Test that help can be accessed without panicking
    let parser = CliParser::command();
    assert!(parser.get_name() == "slang-tokens");
}

#[test]
fn test_cli_parser_with_file() {
    // Test parsing with a file argument
    let args = vec!["slang-tokens", "test.sl"];
    let result = CliParser::try_parse_from(args);
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert_eq!(parsed.input, "test.sl");
    assert!(matches!(parsed.format, TokenFormat::Pretty)); // default format
}

#[test]
fn test_cli_parser_with_debug_format() {
    // Test parsing with debug format
    let args = vec!["slang-tokens", "--format", "debug", "test.sl"];
    let result = CliParser::try_parse_from(args);
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert_eq!(parsed.input, "test.sl");
    assert!(matches!(parsed.format, TokenFormat::Debug));
}
