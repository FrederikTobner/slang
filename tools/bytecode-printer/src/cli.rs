use crate::format::BytecodeFormat;
use clap::Parser as ClapParser;
use colored::Colorize;
use slang_compilation_pipeline::pipeline::builder::PipelineBuilder;
use slang_compilation_pipeline::pipeline::stages::*;
use slang_backend::bytecode::Chunk;
use std::fs;
use zip::ZipArchive;

/// Command line interface for the Slang bytecode analyzer
#[derive(ClapParser)]
#[command(
    version,
    about = "Slang bytecode analyzer",
    long_about = r#"Analyze and print bytecode from Slang source code files or compiled bytecode files.
This tool can compile Slang source files to bytecode and display the resulting instructions,
or directly analyze pre-compiled .sip (Slang Intermediate Program) files.
Useful for debugging code generation, understanding the virtual machine, and compiler optimization analysis."#
)]
pub struct Parser {
    /// Input file - either source (.sl) or compiled bytecode (.sip)
    #[arg(value_name = "FILE")]
    pub input: String,

    /// Output format for bytecode display
    #[arg(
        short, 
        long, 
        default_value = "pretty", 
        help = "Output format: pretty, debug, json"
    )]
    pub format: BytecodeFormat,

    /// Enable verbose output showing compilation stages
    #[arg(
        short,
        long,
        help = "Show detailed compilation pipeline progress"
    )]
    pub verbose: bool,
}

/// Main CLI command handler for analyzing bytecode from source files or compiled files
pub fn analyze_bytecode(
    file_path: &str, 
    format: BytecodeFormat,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate input file exists
    validate_input_file(file_path)?;
    
    // Determine file type and handle accordingly
    let path = std::path::Path::new(file_path);
    let chunk = match path.extension().and_then(|ext| ext.to_str()) {
        Some("sip") => {
            if verbose {
                println!("{}: Loading compiled bytecode from {}", 
                    "Info".blue().bold(), 
                    file_path
                );
            }
            load_bytecode_from_sip(file_path)?
        }
        Some("sl") | None => {
            if verbose {
                println!("{}: Compiling source file {}", 
                    "Info".blue().bold(), 
                    file_path
                );
            }
            compile_source_to_bytecode(file_path, verbose)?
        }
        Some(ext) => {
            return Err(format!(
                "Unsupported file extension '.{}'. Expected '.sl' (source) or '.sip' (compiled bytecode)", 
                ext
            ).into());
        }
    };
    
    if verbose {
        println!("{}: Analyzing bytecode ({} bytes)", 
            "Success".green().bold(), 
            chunk.code.len()
        );
    }
    
    // Determine chunk name from file path
    let name = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("main");
    
    // Format and print the bytecode
    let formatter = format.create_formatter();
    let formatted_bytecode = formatter.format(&chunk, name)?;
    println!("{}", formatted_bytecode);
    
    Ok(())
}

/// Validates that the input file exists and is readable
fn validate_input_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new(file_path);
    
    if !path.exists() {
        return Err(format!("Input file '{}' does not exist", file_path).into());
    }
    
    if !path.is_file() {
        return Err(format!("Input path '{}' is not a file", file_path).into());
    }
    
    // Check if file is readable by attempting to open it
    match fs::File::open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Cannot read input file '{}': {}", file_path, e).into()),
    }
}

/// Reads the source file content
fn read_source_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read source file '{}': {}", file_path, e).into())
}

/// Load bytecode from a compiled .sip file (ZIP archive containing bytecode.bin)
fn load_bytecode_from_sip(file_path: &str) -> Result<Chunk, Box<dyn std::error::Error>> {
    use std::io::Read;
    
    // Open the .sip file as a ZIP archive
    let file = fs::File::open(file_path)
        .map_err(|e| format!("Failed to open .sip file '{}': {}", file_path, e))?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read .sip file as ZIP archive: {}", e))?;
    
    // Look for bytecode.bin in the archive
    let mut bytecode_file = archive.by_name("bytecode.bin")
        .map_err(|e| format!("Failed to find 'bytecode.bin' in .sip file: {}", e))?;
    
    // Read the bytecode data
    let mut bytecode_data = Vec::new();
    bytecode_file.read_to_end(&mut bytecode_data)
        .map_err(|e| format!("Failed to read bytecode data: {}", e))?;
    
    // Deserialize the bytecode chunk using the Chunk's custom deserialize method
    let mut cursor = std::io::Cursor::new(bytecode_data);
    Chunk::deserialize(&mut cursor)
        .map_err(|e| format!("Failed to deserialize bytecode: {}", e).into())
}

/// Compile source code to bytecode using the compilation pipeline
fn compile_source_to_bytecode(file_path: &str, verbose: bool) -> Result<Chunk, Box<dyn std::error::Error>> {
    // Read source file
    let source = read_source_file(file_path)?;
    
    // Create a full compilation pipeline
    let pipeline = PipelineBuilder::new(&source)
        .add_stage(TokenizationStage)
        .add_stage(ParsingStage)
        .add_stage(SemanticAnalysisStage)
        .add_stage(CodeGenerationStage)
        .with_file_name(file_path.to_string())
        .build();
    
    // Execute the pipeline to get bytecode
    match pipeline.execute() {
        slang_compilation_pipeline::pipeline::result::CompilationResult::Success { output, diagnostics } => {
            // Try to downcast the output to Chunk
            if let Ok(chunk) = output.downcast::<Chunk>() {
                let chunk = *chunk;
                
                if verbose && diagnostics.has_errors() {
                    println!("{}: Compilation completed with diagnostics", 
                        "Warning".yellow().bold()
                    );
                }
                
                Ok(chunk)
            } else {
                Err("Pipeline output was not in expected bytecode format".into())
            }
        }
        slang_compilation_pipeline::pipeline::result::CompilationResult::Failed { diagnostics: _ } => {
            if verbose {
                eprintln!("{}: Compilation failed", 
                    "Error".red().bold()
                );
            }
            
            eprintln!("{}: Failed to compile source code to bytecode", 
                "Error".red().bold()
            );
            
            Err("Failed to compile source code to bytecode. Check source file for errors.".into())
        }
    }
}
