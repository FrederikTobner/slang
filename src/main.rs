mod ast;
mod ast_printer;
mod bytecode;
mod compiler;
mod lexer;
mod parser;
mod token;
mod type_checker;
mod value;
mod visitor;
mod vm;
mod types;

use ast::Statement;
use bytecode::Chunk;
use compiler::Compiler;
use clap::{Parser as ClapParser, Subcommand};
use lexer::{tokenize, LexerResult};
use parser::Parser;
use type_checker::TypeChecker;
use vm::VM;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use zip::{ZipArchive, ZipWriter, write::FileOptions};
use crate::types::TYPE_REGISTRY;

/// The extension for compiled Slang binaries
const SLANG_BYTECODE_EXTENSION: &str = "sip";

/// Command line interface for the Slang language
#[derive(ClapParser)]
#[command(version, about = "Slang programming language", long_about = r#"Slang is a simple programming language designed for educational purposes.
It features a REPL, compilation to bytecode, and execution of both source files and compiled bytecode."#)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Available commands for the Slang CLI
#[derive(Subcommand)]
enum Commands {
    /// Run the interactive REPL
    Repl {},
    
    /// Compile a Slang source file to bytecode
    Compile {
        /// Input source file
        input: String,
        
        /// Output bytecode file (default: same as input with .sip extension)
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Run a compiled Slang bytecode file
    Run {
        /// Input compiled bytecode file
        input: String,
    },
    
    /// Run a Slang source file directly
    Execute {
        /// Input source file
        input: String,
    },
}

/// Initialize the type system with built-in types
fn init_type_system() {
    let _ = crate::types::unknown_type();
    TYPE_REGISTRY.with(|_| ());
}

/// Parse source code into an AST
/// 
/// # Arguments
/// 
/// * `lexer_result` - The lexer result containing tokens and line info
/// 
/// # Returns
/// 
/// The parsed AST statements or an error message
fn parse(lexer_result: &LexerResult) -> Result<Vec<Statement>, String> {
    let mut parser = Parser::new(&lexer_result.tokens, &lexer_result.line_info);
    parser.parse()
}

/// Read a source file into a string
/// 
/// # Arguments
/// 
/// * `path` - The path to the source file
/// 
/// # Returns
/// 
/// The file contents or an error message
fn read_source_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Error reading file '{}': {}", path, e))
}

/// Compile source code to bytecode
/// 
/// # Arguments
/// 
/// * `source` - The source code to compile
/// 
/// # Returns
/// 
/// The compiled bytecode chunk or an error message
fn compile_source(source: &str) -> Result<Chunk, String> {
    let lexer_result = tokenize(source);
    let ast = parse(&lexer_result)?;
    
    let mut type_checker = TypeChecker::new();
    type_checker.check(&ast)?;
    
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&ast)?;
    
    Ok(chunk.clone())
}

/// Write a bytecode chunk to a file
/// 
/// # Arguments
/// 
/// * `chunk` - The bytecode chunk to write
/// * `output_path` - The path to write to
/// 
/// # Returns
/// 
/// Ok(()) if successful, or an error message
fn write_bytecode(chunk: &Chunk, output_path: &str) -> Result<(), String> {
    let path = Path::new(output_path);
    
    // Create a zip file
    let file = File::create(path)
        .map_err(|e| format!("Failed to create output file: {}", e))?;
    
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::<()>::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
    
    zip.start_file("bytecode.bin", options)
        .map_err(|e| format!("Failed to create zip entry: {}", e))?;
    
    {
        let mut cursor = std::io::Cursor::new(Vec::new());
        chunk.serialize(&mut cursor)
            .map_err(|e| format!("Failed to serialize bytecode: {}", e))?;
        
        zip.write_all(&cursor.into_inner())
            .map_err(|e| format!("Failed to write bytecode: {}", e))?;
    }
    
    zip.finish()
        .map_err(|e| format!("Failed to finalize zip file: {}", e))?;
    
    Ok(())
}

/// Read a bytecode chunk from a file
/// 
/// # Arguments
/// 
/// * `input_path` - The path to read from
/// 
/// # Returns
/// 
/// The bytecode chunk or an error message
fn read_bytecode(input_path: &str) -> Result<Chunk, String> {
    let file = File::open(input_path)
        .map_err(|e| format!("Failed to open bytecode file '{}': {}", input_path, e))?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read zip archive: {}", e))?;
    
    // Find and extract the bytecode file
    if let Ok(mut bytecode_file) = archive.by_name("bytecode.bin") {
        let mut buffer = Vec::new();
        std::io::copy(&mut bytecode_file, &mut buffer)
            .map_err(|e| format!("Failed to read bytecode data: {}", e))?;
        
        let mut cursor = std::io::Cursor::new(buffer);
        let chunk = Chunk::deserialize(&mut cursor)
            .map_err(|e| format!("Failed to deserialize bytecode: {}", e))?;
        
        Ok(chunk)
    } else {
        Err("Invalid bytecode file format: missing bytecode.bin".to_string())
    }
}

/// Execute a bytecode chunk in the VM
/// 
/// # Arguments
/// 
/// * `chunk` - The bytecode chunk to run
/// 
/// # Returns
/// 
/// Ok(()) if successful, or an error message
fn run_bytecode(chunk: &Chunk) -> Result<(), String> {
    let mut vm = VM::new();
    vm.interpret(chunk)
}

/// Run the interactive REPL
fn repl() {
    let mut type_checker = TypeChecker::new();
    let mut vm = VM::new();
    
    println!("Slang REPL - Type 'exit' to exit");
    
    loop {
        let mut input = String::new();
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        let trimmed;
        if std::io::stdin().read_line(&mut input).is_ok() {
            trimmed = input.trim();
            if input.trim() == "exit"{
                break;
            }
        } else {
            println!("Error reading input. Try again.");
            continue;
        }

        if trimmed.is_empty() {
            continue;
        }
        

        let lexer_result = tokenize(&input);
        if lexer_result.tokens.len() <= 1 {  // Just EOF token
            continue;
        }

        match parse(&lexer_result) {
            Ok(ast) => {
                #[cfg(feature = "print-ast")]
                {
                    println!("\n=== AST ===");
                    let mut printer = ast_printer::ASTPrinter::new();
                    printer.print(&ast);
                }
                match type_checker.check(&ast) {
                    Ok(_) => {
                        let mut compiler = Compiler::new();
                        match compiler.compile(&ast) {
                            Ok(chunk) => {

                                #[cfg(feature = "print-byte_code")]
                                {
                                    println!("\n=== Bytecode ===");
                                    chunk.disassemble("REPL");
                                }
                                // Execute the bytecode
                                if let Err(e) = vm.interpret(&chunk) {
                                    eprintln!("Runtime error: {}", e);
                                }
                            }
                            Err(e) => eprintln!("Compilation error: {}", e),
                        }
                    }
                    Err(e) => {
                        eprintln!("Type error: {}", e);
                    }
                }
            }
            Err(e) => eprintln!("Parse error: {}", e),
        }
    }
}

/// Application entry point
fn main() {
     let cli = Cli::parse();

    // Initialize the type registry
    init_type_system();

    match &cli.command {
        Some(Commands::Repl{}) => {
            repl();
        },
        
        Some(Commands::Compile { input, output }) => {
            let output_path = match output {
                Some(path) => path.clone(),
                None => {
                    let path = Path::new(input);
                    let stem = path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("output");
                    format!("{}.{}", stem, SLANG_BYTECODE_EXTENSION)
                }
            };
            
            println!("Compiling {} to {}", input, output_path);
            
            match read_source_file(input) {
                Ok(source) => {
                    match compile_source(&source) {
                        Ok(chunk) => {
                            if let Err(e) = write_bytecode(&chunk, &output_path) {
                                eprintln!("Failed to write bytecode: {}", e);
                            } else {
                                println!("Successfully compiled to {}", output_path);
                            }
                        },
                        Err(e) => eprintln!("Compilation failed: {}", e),
                    }
                },
                Err(e) => eprintln!("{}", e),
            }
        },
        
        Some(Commands::Run { input }) => {
            println!("Running compiled file: {}", input);
            match read_bytecode(input) {
                Ok(chunk) => {
                    if let Err(e) = run_bytecode(&chunk) {
                        eprintln!("Runtime error: {}", e);
                    }
                },
                Err(e) => eprintln!("Failed to load bytecode: {}", e),
            }
        },
        
        Some(Commands::Execute { input }) => {
            println!("Executing source file: {}", input);
            match read_source_file(input) {
                Ok(source) => {
                    match compile_source(&source) {
                        Ok(chunk) => {
                            if let Err(e) = run_bytecode(&chunk) {
                                eprintln!("Runtime error: {}", e);
                            }
                        },
                        Err(e) => eprintln!("Compilation failed: {}", e),
                    }
                },
                Err(e) => eprintln!("{}", e),
            }
        },
        
        None => {
            // Default to REPL if no command is provided
            repl();
        },
    }
}
