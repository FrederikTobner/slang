use crate::exit;
use clap::{Parser as ClapParser, Subcommand};
use colored::Colorize;
use slang_backend::vm::VM;
use slang_backend::compiler;
use slang_frontend::error::{CompileResult, report_errors};
use slang_frontend::lexer;
use slang_frontend::parser;
use slang_frontend::type_guard;
use slang_backend::bytecode::Chunk;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use zip::{ZipArchive, ZipWriter, write::FileOptions};

//------------------------------------------------------------------------------
// Public CLI Interface
//------------------------------------------------------------------------------

/// Command line interface for the Slang language
#[derive(ClapParser)]
#[command(
    version,
    about = "Slang programming language",
    long_about = r#"Slang is a simple programming language designed for educational purposes.
It features a REPL, compilation to bytecode, and execution of both source files and compiled bytecode."#
)]
pub struct CliParser {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available commands for the Slang CLI
#[derive(Subcommand)]
pub enum Commands {
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

/// The extension for compiled Slang bytecode files
const SLANG_BYTECODE_EXTENSION: &str = "sip";

/// Parse command line arguments
pub fn parse_args() -> CliParser {
    CliParser::parse()
}

//------------------------------------------------------------------------------
// Main Command Implementations
//------------------------------------------------------------------------------

/// Run the interactive REPL
pub fn repl() {
    let mut vm = VM::new();
    println!("Slang REPL - Type 'exit' to exit");

    loop {
        let mut input = String::new();
        print!(">>> ");
        std::io::stdout().flush().unwrap();

        if std::io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim();
            if trimmed == "exit" {
                break;
            } else if trimmed.is_empty() {
                continue;
            }
        } else {
            println!("Error reading input. Try again.");
            continue;
        }

        // Compile the input
        match compile_source(&input) {
            Ok(chunk) => {
                #[cfg(feature = "print-byte_code")]
                {
                    println!("\n=== Bytecode ===");
                    chunk.disassemble("REPL");
                }

                // Execute the bytecode
                if let Err(e) = vm.interpret(&chunk) {
                    eprintln!("{}: {}", "Runtime error".red(), e);
                }
            }
            Err(errors) => {
                // In REPL mode, we just report errors and continue
                report_errors(&errors);
            }
        }
    }
}

/// Compile a Slang source file to bytecode
pub fn compile_file(input: &str, output: Option<String>) {
    let output_path = determine_output_path(input, output);
    println!("Compiling {} to {}", input, output_path);

    match read_source_file(input) {
        Ok(source) => match compile_source(&source) {
            Ok(chunk) => {
                if let Err((code, message)) = write_bytecode(&chunk, &output_path) {
                    exit::with_code(code, &message)
                } else {
                    println!("Successfully compiled to {}", output_path);
                }
            }
            Err(errors) => {
                report_errors(&errors);
                exit::with_code(
                    exit::Code::Software,
                    &format!("{}: Compilation failed due to previous error(s)", "error".red()),
                );
            }
        },
        Err((code, message)) => exit::with_code(code, &message),
    }
}

/// Execute a Slang source file directly
pub fn execute_file(input: &str) {
    println!("Executing source file: {}", input);
    
    match read_source_file(input) {
        Ok(source) => match compile_source(&source) {
            Ok(chunk) => {
                if let Err(e) = run_bytecode(&chunk) {
                    exit::with_code(
                        exit::Code::Software,
                        &format!("{}: {}", "Runtime Error".red(), e),
                    );
                }
            }
            Err(errors) => {
                report_errors(&errors);
                exit::with_code(
                    exit::Code::Software,
                    &format!("{}: Compilation failed due to previous error(s)", "error".red()),
                );
            }
        },
        Err((code, message)) => exit::with_code(code, &message),
    }
}

/// Run a compiled Slang bytecode file
pub fn run_file(input: &str) {
    println!("Running compiled file: {}", input);
    
    match read_bytecode_file(input) {
        Ok(chunk) => {
            if let Err(e) = run_bytecode(&chunk) {
                exit::with_code(
                    exit::Code::Software,
                    &format!("{}: {}", "Runtime Error".red(), e),
                );
            }
        }
        Err((code, message)) => exit::with_code(code, &message),
    }
}

//------------------------------------------------------------------------------
// Compilation
//------------------------------------------------------------------------------

/// Compile source code to bytecode
///
/// # Arguments
///
/// * `source` - The source code to compile
///
/// # Returns
///
/// The compiled bytecode chunk or compilation errors
fn compile_source(source: &str) -> CompileResult<Chunk> {
    let lexer_result = lexer::tokenize(source);
    let statements = parser::parse(&lexer_result.tokens, &lexer_result.line_info)?;
    #[cfg(feature = "print-ast")]
    {
        let mut printer = slang_ir::ast_printer::ASTPrinter::new();
        printer.print(&statements);
    }
    type_guard::execute(&statements)?;
    compiler::compile(&statements)
        .map_err(|err| vec![slang_frontend::error::CompilerError::new(err, 0, 0)])
}

//------------------------------------------------------------------------------
// File Operations
//------------------------------------------------------------------------------

/// Determine the output path for a compiled file
fn determine_output_path(input: &str, output: Option<String>) -> String {
    match output {
        Some(path) => path,
        None => {
            let path = Path::new(input);
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            format!("{}.{}", stem, SLANG_BYTECODE_EXTENSION)
        }
    }
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
fn read_source_file(path: &str) -> Result<String, (exit::Code, String)> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => {
            let exit_code = match e.kind() {
                std::io::ErrorKind::NotFound => exit::Code::NoInput,
                std::io::ErrorKind::PermissionDenied => exit::Code::NoPerm,
                _ => exit::Code::IoErr,
            };
            Err((exit_code, format!("Error reading file '{}': {}", path, e)))
        }
    }
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
fn write_bytecode(chunk: &Chunk, output_path: &str) -> Result<(), (exit::Code, String)> {
    let path = Path::new(output_path);

    // Create a zip file
    let file = File::create(path).map_err(|e| match e.kind() {
        std::io::ErrorKind::PermissionDenied => (
            exit::Code::NoPerm,
            format!("Permission denied when creating output file: {}", e),
        ),
        _ => (
            exit::Code::CantCreat,
            format!("Failed to create output file: {}", e),
        ),
    })?;

    let mut zip = ZipWriter::new(file);
    let options = FileOptions::<()>::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    zip.start_file("bytecode.bin", options).map_err(|e| {
        (
            exit::Code::IoErr,
            format!("Failed to create zip entry: {}", e),
        )
    })?;

    {
        let mut cursor = std::io::Cursor::new(Vec::new());
        chunk.serialize(&mut cursor).map_err(|e| {
            (
                exit::Code::Software,
                format!("Failed to serialize bytecode: {}", e),
            )
        })?;

        zip.write_all(&cursor.into_inner()).map_err(|e| {
            (
                exit::Code::IoErr,
                format!("Failed to write bytecode: {}", e),
            )
        })?;
    }

    zip.finish().map_err(|e| {
        (
            exit::Code::IoErr,
            format!("Failed to finalize zip file: {}", e),
        )
    })?;

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
fn read_bytecode_file(input_path: &str) -> Result<Chunk, (exit::Code, String)> {
    let file = File::open(input_path).map_err(|e| {
        (
            exit::Code::NoInput,
            format!("Failed to open bytecode file '{}': {}", input_path, e),
        )
    })?;

    let mut archive = ZipArchive::new(file).map_err(|e| {
        (
            exit::Code::Dataerr,
            format!("Failed to read zip archive: {}", e),
        )
    })?;

    // Find and extract the bytecode file
    if let Ok(mut bytecode_file) = archive.by_name("bytecode.bin") {
        let mut buffer = Vec::new();
        std::io::copy(&mut bytecode_file, &mut buffer).map_err(|e| {
            (
                exit::Code::IoErr,
                format!("Failed to read bytecode data: {}", e),
            )
        })?;

        let mut cursor = std::io::Cursor::new(buffer);
        let chunk = Chunk::deserialize(&mut cursor).map_err(|e| {
            (
                exit::Code::Dataerr,
                format!("Failed to deserialize bytecode: {}", e),
            )
        })?;

        Ok(chunk)
    } else {
        Err((
            exit::Code::Dataerr,
            "Invalid bytecode file format: missing bytecode.bin".to_string(),
        ))
    }
}

//------------------------------------------------------------------------------
// VM Operations
//------------------------------------------------------------------------------

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
