mod ast;
mod ast_printer;
mod bytecode;
mod compiler;
mod lexer;
mod parser;
mod token;
mod type_checker;
mod visitor;
mod vm;
mod types;

use ast::Statement;
use bytecode::Chunk;
use compiler::Compiler;
use clap::{Parser as ClapParser, Subcommand};
use lexer::tokenize;
use parser::Parser;
use token::Token;
use type_checker::TypeChecker;
use vm::VM;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use zip::{ZipArchive, ZipWriter, write::FileOptions};
use crate::types::TYPE_REGISTRY;

/// The extension for compiled Slang binaries
const SLANG_BYTECODE_EXTENSION: &str = "sip";

#[derive(ClapParser)]
#[command(version, about = "Slang programming language", long_about = r#"Slang is a simple programming language designed for educational purposes.
It features a REPL, compilation to bytecode, and execution of both source files and compiled bytecode."#)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

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

fn init_type_system() {
    let _ = crate::types::unknown_type();
    TYPE_REGISTRY.with(|_| ());
}

fn parse(tokens: &[Token]) -> Result<Vec<Statement>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

fn read_source_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Error reading file '{}': {}", path, e))
}

fn compile_source(source: &str) -> Result<Chunk, String> {
    let tokens = tokenize(source);
    let ast = parse(&tokens)?;
    
    let mut type_checker = TypeChecker::new();
    type_checker.check(&ast)?;
    
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&ast)?;
    
    Ok(chunk.clone())
}

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

fn run_bytecode(chunk: &Chunk) -> Result<(), String> {
    let mut vm = VM::new();
    vm.interpret(chunk)
}

fn repl() {
    let mut type_checker = TypeChecker::new();
    let mut vm = VM::new();
    
    println!("Slang REPL - Type 'exit' to exit");
    println!("Type 'vars' to display currently defined variables");
    
    loop {
        let mut input = String::new();
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        
        if std::io::stdin().read_line(&mut input).is_ok() {
            if input.trim() == "exit" {
                break;
            }
        } else {
            println!("Error reading input. Try again.");
            continue;
        }

        let trimmed = input.trim();
        
        
        if trimmed.is_empty() {
            continue;
        }
        

        let tokens = tokenize(&input);
        if tokens.len() <= 1 {  // Just EOF token
            continue;
        }

        match parse(&tokens) {
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
                                    println!("Runtime error: {}", e);
                                }
                            }
                            Err(e) => println!("Compilation error: {}", e),
                        }
                    }
                    Err(e) => {
                        println!("Type error: {}", e);
                    }
                }
            }
            Err(e) => println!("Parse error: {}", e),
        }
    }
}

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
                                println!("Failed to write bytecode: {}", e);
                            } else {
                                println!("Successfully compiled to {}", output_path);
                            }
                        },
                        Err(e) => println!("Compilation failed: {}", e),
                    }
                },
                Err(e) => println!("{}", e),
            }
        },
        
        Some(Commands::Run { input }) => {
            println!("Running compiled file: {}", input);
            match read_bytecode(input) {
                Ok(chunk) => {
                    if let Err(e) = run_bytecode(&chunk) {
                        println!("Runtime error: {}", e);
                    }
                },
                Err(e) => println!("Failed to load bytecode: {}", e),
            }
        },
        
        Some(Commands::Execute { input }) => {
            println!("Executing source file: {}", input);
            match read_source_file(input) {
                Ok(source) => {
                    match compile_source(&source) {
                        Ok(chunk) => {
                            if let Err(e) = run_bytecode(&chunk) {
                                println!("Runtime error: {}", e);
                            }
                        },
                        Err(e) => println!("Compilation failed: {}", e),
                    }
                },
                Err(e) => println!("{}", e),
            }
        },
        
        None => {
            // Default to REPL if no command is provided
            repl();
        },
    }
}
