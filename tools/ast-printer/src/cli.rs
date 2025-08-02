use crate::format::AstFormat;
use clap::Parser as ClapParser;
use colored::Colorize;
use std::fs;
use slang_compilation_pipeline::pipeline::builder::PipelineBuilder;
use slang_compilation_pipeline::pipeline::stages::{TokenizationStage, ParsingStage};
use slang_compilation_pipeline::pipeline::error::ErrorStrategy;
use slang_ir::ast::Statement;

/// Command line interface for the Slang AST analyzer
#[derive(ClapParser)]
#[command(
    version,
    about = "Slang AST analyzer", 
    long_about = r#"Analyze and print Abstract Syntax Trees from Slang source code files.
This tool parses Slang source files and displays the resulting AST in various formats,
useful for debugging parser behavior, understanding code structure, and compiler development."#
)]
pub struct Parser {
    /// Input source file to parse
    #[arg(value_name = "FILE")]
    pub input: String,

    /// Output format for AST display
    #[arg(
        short, 
        long, 
        default_value = "pretty", 
        help = "Output format: pretty, json, compact"
    )]
    pub format: AstFormat,

    /// Include semantic analysis in the AST output
    #[arg(
        long,
        help = "Run semantic analysis and show the analyzed AST"
    )]
    pub semantic: bool,

    /// Enable verbose output showing compilation stages
    #[arg(
        short,
        long,
        help = "Show detailed compilation pipeline progress"
    )]
    pub verbose: bool,
}

/// Main CLI command handler for parsing files and printing AST
pub fn parse_and_print_ast(
    file_path: &str, 
    format: AstFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate input file
    validate_input_file(file_path)?;
    
    // Read source file
    let source = read_source_file(file_path)?;
    
    if format == AstFormat::Pretty {
        println!("{}: Parsing {}", "Info".blue().bold(), file_path);
    }
    
    // Create a pipeline that only runs tokenization and parsing stages
    let pipeline = PipelineBuilder::new(&source)
        .add_stage(TokenizationStage)
        .add_stage(ParsingStage)
        .with_error_strategy(ErrorStrategy::Recover { continue_on_non_critical: false })
        .build();
    
    // Execute the pipeline to get AST
    match pipeline.execute() {
        slang_compilation_pipeline::pipeline::result::CompilationResult::Success { output, diagnostics } => {
            // Try to downcast the output to Vec<Statement>
            if let Ok(statements) = output.downcast::<Vec<Statement>>() {
                let statements = *statements;
                
                if format == AstFormat::Pretty {
                    println!("{}: Successfully parsed {} statements", 
                        "Success".green().bold(), 
                        statements.len()
                    );
                }
                
                // Format and print the AST
                let formatter = format.create_formatter();
                let formatted_ast = formatter.format(&statements)?;
                println!("{}", formatted_ast);
                
                // Print any warnings or notes
                if diagnostics.has_errors() && format == AstFormat::Pretty {
                    eprintln!("\n{}: Compilation completed with diagnostics", "Info".blue().bold());
                }
            } else {
                return Err("Pipeline output was not in expected format".into());
            }
        }
        slang_compilation_pipeline::pipeline::result::CompilationResult::Failed { diagnostics: _ } => {
            return Err("Failed to parse source code. Check source file for syntax errors.".into());
        }
    }
    
    Ok(())
}

/// Validate that the input file exists and is readable
fn validate_input_file(file_path: &str) -> Result<(), String> {
    let path = std::path::Path::new(file_path);
    
    if !path.exists() {
        return Err(format!("File '{}' does not exist", file_path));
    }
    
    if !path.is_file() {
        return Err(format!("'{}' is not a regular file", file_path));
    }
    
    // Check if file has .sl extension (optional warning)
    if let Some(extension) = path.extension() {
        if extension != "sl" {
            eprintln!(
                "{}: File '{}' does not have .sl extension", 
                "Warning".yellow().bold(),
                file_path
            );
        }
    }
    
    Ok(())
}

/// Read source file with proper error handling
fn read_source_file(file_path: &str) -> Result<String, String> {
    fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))
}
