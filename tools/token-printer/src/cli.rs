use crate::format::TokenFormat;
use crate::observer::TokenPrinter;
use clap::Parser as ClapParser;
use slang_compilation_pipeline::chain_pipeline::ChainPipeline;
use slang_compilation_pipeline::SlangSourceFile;
use std::fs;

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

/// Main CLI command handler for tokenizing files using the new chain-aware pipeline
pub fn tokenize_file(file_path: &str, format: TokenFormat) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;
    let formatter = format.create_formatter();
    
    let source_file = SlangSourceFile::for_tooling(file_path, source.clone());
    
    // Create a pipeline with only tokenization - no parsing or further stages
    let pipeline = ChainPipeline::tokenization_only()  
        .with_tokenization_observer(TokenPrinter::new(formatter, file_path.to_string()));

    match pipeline.execute(source_file) {
        slang_compilation_pipeline::result::CompilationResult::Success { output: _tokens, diagnostics } => {
            if diagnostics.has_errors() {
                eprintln!("Warnings occurred during tokenization:");
                diagnostics.report_all(&source);
            }
        }
        slang_compilation_pipeline::result::CompilationResult::Failed { diagnostics } => {
            eprintln!("Error tokenizing file:");
            diagnostics.report_all(&source);
            return Err("Tokenization failed".into());
        }
    }
    
    Ok(())
}