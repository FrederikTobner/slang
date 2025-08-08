use crate::format::AstFormat;
use crate::observer::ASTPrinter;
use clap::Parser as ClapParser;
use colored::Colorize;
use std::fs;
use slang_compilation_pipeline::ChainPipeline;
use slang_compilation_pipeline::SlangSourceFile;

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
    validate_input_file(file_path)?;
    
    let source = read_source_file(file_path)?;
    
    if format == AstFormat::Pretty {
        println!("{}: Parsing {}", "Info".blue().bold(), file_path);
    }

    let source_file = SlangSourceFile::for_tooling(file_path, source.clone());
    
    let pipeline = ChainPipeline::parsing_only()
        .with_parsing_observer(ASTPrinter::new());

    match pipeline.execute(source_file) {
        slang_compilation_pipeline::result::CompilationResult::Success { output, diagnostics } => {
            let statements = output;
            
            if format == AstFormat::Pretty {
                println!("{}: Successfully parsed {} statements", 
                    "Success".green().bold(), 
                    statements.len()
                );
            }
            
            let formatter = format.create_formatter();
            let formatted_ast = formatter.format(&statements)?;
            println!("{formatted_ast}");
            
            if diagnostics.has_errors() && format == AstFormat::Pretty {
                eprintln!("\n{}: Compilation completed with diagnostics", "Info".blue().bold());
            }
        }
        slang_compilation_pipeline::result::CompilationResult::Failed { diagnostics: _ } => {
            return Err("Failed to parse source code. Check source file for syntax errors.".into());
        }
    }
    
    Ok(())
}

/// Validate that the input file exists and is readable
fn validate_input_file(file_path: &str) -> Result<(), String> {
    let path = std::path::Path::new(file_path);
    
    if !path.exists() {
        return Err(format!("File '{file_path}' does not exist"));
    }
    
    if !path.is_file() {
        return Err(format!("'{file_path}' is not a regular file"));
    }
    
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
        .map_err(|e| format!("Failed to read file '{file_path}': {e}"))
}
