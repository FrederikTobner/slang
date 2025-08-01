use crate::format::TokenFormat;
use crate::observer::TokenPrinterObserver;
use clap::Parser as ClapParser;
use colored::Colorize;
use slang_compilation_pipeline::PipelineBuilder;
use slang_compilation_pipeline::pipeline::stages::TokenizationStage;
use std::fs;
use std::path::Path;

/// Command line interface for the Slang token analyzer
#[derive(ClapParser)]
#[command(
    version,
    about = "Slang token analyzer",
    long_about = r#"Analyze and print tokens from Slang source code files.
This tool tokenizes Slang source files and displays the resulting tokens in various formats,
useful for debugging lexical analysis and understanding how the compiler processes source code."#
)]
pub struct Parser {
    /// Input source file to tokenize
    #[arg(value_name = "FILE")]
    pub input: String,

    /// Output format for token display
    #[arg(short, long, default_value = "pretty", help = "Output format: pretty, debug")]
    pub format: TokenFormat,
}

/// Main CLI command handler for tokenizing files
pub fn tokenize_file(file_path: &str, format: TokenFormat) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;
    let formatter = format.create_formatter();
    let observer = TokenPrinterObserver::new(formatter, file_path.to_string());
    
    let pipeline = PipelineBuilder::new(&source)
        .add_stage(TokenizationStage)
        .add_tokenization_observer(observer)
        .build();
    
    let _result = pipeline.execute();
    Ok(())
}

/// Validate that the input file exists and has the correct extension
fn validate_input_file(input_path: &str) -> Result<(), String> {
    let path = Path::new(input_path);
    
    // Check if file exists
    if !path.exists() {
        return Err(format!("File '{}' does not exist", input_path));
    }
    
    // Check if it's a file (not a directory)
    if !path.is_file() {
        return Err(format!("'{}' is not a file", input_path));
    }
    
    // Check file extension (optional but helpful)
    if let Some(extension) = path.extension() {
        if extension != "sl" {
            eprintln!(
                "{}: File '{}' does not have .sl extension",
                "Warning".yellow().bold(),
                input_path
            );
        }
    } else {
        eprintln!(
            "{}: File '{}' has no extension, expected .sl",
            "Warning".yellow().bold(),
            input_path
        );
    }
    
    Ok(())
}

/// Read the source file content
fn read_source_file(input_path: &str) -> Result<String, String> {
    fs::read_to_string(input_path)
        .map_err(|e| format!("Failed to read file '{}': {}", input_path, e))
}
