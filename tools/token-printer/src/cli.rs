use crate::format::TokenFormat;
use crate::observer::TokenPrinterObserver;
use clap::Parser as ClapParser;
use slang_compilation_pipeline::PipelineBuilder;
use slang_compilation_pipeline::pipeline::stages::TokenizationStage;
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
