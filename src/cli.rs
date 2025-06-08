use crate::error::{CliError, CliResult};
use crate::exit;
use clap::{Parser as ClapParser, Subcommand};
use colored::Colorize;
use slang_backend::bytecode::Chunk;
use slang_backend::codegen;
use slang_backend::vm::VM;
use slang_frontend::error::{CompileResult, report_errors};
use slang_frontend::{lexer, parser, semantic_analyzer};
use slang_shared::compilation_context::{CompilationContext};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use zip::{ZipArchive, ZipWriter, write::FileOptions};

/// Command line interface for the Slang language
#[derive(ClapParser)]
#[command(
    version,
    about = "Slang programming language",
    long_about = r#"Slang is a simple programming language designed for educational purposes.
It features a REPL, compilation to bytecode, and execution of both source files and compiled bytecode."#,
arg_required_else_help = true,
)]
pub struct Parser {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available commands for the Slang CLI
#[derive(Subcommand)]
pub enum Commands {
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

/// Compile a Slang source file to bytecode
///
/// ### Arguments
/// * `input` - The input source file
/// * `output` - The output file path (if provided)
pub fn compile_file(input: &str, output: Option<String>) {
    let output_path = resolve_output_path(input, output);
    println!("Compiling {} to {}", input, output_path);
    let mut compilation_context = CompilationContext::new();
    match read_source_file(input) {
        Ok(source) => match compile_source_to_bytecode(&source, &mut compilation_context) {
            Ok(chunk) => {
                if let Err(err) = write_bytecode(&chunk, &output_path) {
                    exit::with_code(err.exit_code(), &err.to_string())
                } else {
                    println!("Successfully compiled to {}", output_path);
                }
            }
            Err(errors) => {
                report_errors(&errors, &source); 
                exit::with_code(
                    exit::Code::Software,
                    &format!(
                        "{}: Compilation failed due to previous error(s)",
                        "error".red()
                    ),
                );
            }
        },
        Err(err) => exit::with_code(err.exit_code(), &err.to_string()),
    }
}

/// Execute a Slang source file directly
///
/// ### Arguments
/// * `input` - The input source file
pub fn execute_file(input: &str) {
    println!("Executing source file: {}", input);
    let mut compilation_context = CompilationContext::new();
    match read_source_file(input) {
        Ok(source) => match compile_source_to_bytecode(&source, &mut compilation_context) {
            Ok(chunk) => {
                if let Err(e) = execute_bytecode(&chunk) {
                    exit::with_code(
                        exit::Code::Software,
                        &format!("{}: {}", "Runtime Error".red(), e),
                    );
                }
            }
            Err(errors) => {
                report_errors(&errors, &source); 
                exit::with_code(
                    exit::Code::Software,
                    &format!(
                        "{}: Compilation failed due to previous error(s)",
                        "error".red()
                    ),
                );
            }
        },
        Err(err) => exit::with_code(err.exit_code(), &err.to_string()),
    }
}

/// Run a compiled Slang bytecode file
///
/// ### Arguments
/// * `input` - The input compiled bytecode file
pub fn run_file(input: &str) {
    println!("Running compiled file: {}", input);

    match read_bytecode_file(input) {
        Ok(chunk) => {
            if let Err(e) = execute_bytecode(&chunk) {
                exit::with_code(
                    exit::Code::Software,
                    &format!("{}: {}", "Runtime Error".red(), e),
                );
            }
        }
        Err(err) => exit::with_code(err.exit_code(), &err.to_string()),
    }
}

/// Compile source code to bytecode
///
/// ### Arguments
///
/// * `source` - The source code to compile
///
/// ### Returns
///
/// The compiled bytecode chunk or compilation errors
fn compile_source_to_bytecode(source: &str, compilation_context: &mut CompilationContext) -> CompileResult<Chunk> {
    let lexer_result = lexer::tokenize(source)?;
    #[cfg(feature = "print-tokens")]
    {
        let printer = slang_frontend::token_printer::TokenPrinter::new();
        printer.print(&lexer_result.tokens);
    }
    let statements = parser::parse(&lexer_result.tokens, &lexer_result.line_info, compilation_context)?;
    #[cfg(feature = "print-ast")]
    {
        let mut printer = slang_ir::ast_printer::ASTPrinter::new();
        printer.print(&statements);
    }
    semantic_analyzer::execute(&statements, compilation_context)?;
    codegen::generate_bytecode(&statements).map_err(|err_msg| {
        vec![slang_frontend::error::CompilerError::new(
            slang_frontend::error_codes::ErrorCode::GenericCompileError,
            err_msg,
            0,
            0,
            0,
            None,
        )]
    })
}

/// Determine the output path for a compiled file
///
/// ### Arguments
/// * `input` - The input source file
/// * `output` - The output file path (if provided)
///
/// ### Returns
/// The resolved output path
fn resolve_output_path(input: &str, output: Option<String>) -> String {
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
/// ### Arguments
///
/// * `path` - The path to the source file
///
/// ### Returns
///
/// The file contents or an error message
fn read_source_file(path: &str) -> CliResult<String> {
    fs::read_to_string(path).map_err(|e| CliError::from_io_error(e, path))
}

/// Write a bytecode chunk to a file
///
/// ### Arguments
///
/// * `chunk` - The bytecode chunk to write
/// * `output_path` - The path to write to
///
/// ### Returns
///
/// Ok(()) if successful, or an error message
fn write_bytecode(chunk: &Chunk, output_path: &str) -> CliResult<()> {
    let path = Path::new(output_path);

    let file = File::create(path).map_err(|e| CliError::Io {
        exit_code: if e.kind() == std::io::ErrorKind::PermissionDenied {
            exit::Code::NoPerm
        } else {
            exit::Code::CantCreat
        },
        source: e,
        path: output_path.to_string(),
    })?;

    let mut zip = ZipWriter::new(file);
    let options = FileOptions::<()>::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    zip.start_file("bytecode.bin", options)
        .map_err(|e| CliError::Zip {
            source: e,
            context: "Failed to create zip entry".to_string(),
            exit_code: exit::Code::IoErr,
        })?;

    {
        let mut cursor = std::io::Cursor::new(Vec::new());
        chunk
            .serialize(&mut cursor)
            .map_err(|e| CliError::Serialization {
                source: Box::new(e),
                context: "Failed to serialize bytecode".to_string(),
                exit_code: exit::Code::Software,
            })?;

        zip.write_all(&cursor.into_inner())
            .map_err(|e| CliError::Io {
                source: e,
                path: output_path.to_string(),
                exit_code: exit::Code::IoErr,
            })?;
    }

    zip.finish().map_err(|e| CliError::Zip {
        source: e,
        context: "Failed to finalize zip file".to_string(),
        exit_code: exit::Code::IoErr,
    })?;

    Ok(())
}

/// Read a bytecode chunk from a file
///
/// ### Arguments
///
/// * `input_path` - The path to read from
///
/// ### Returns
///
/// The bytecode chunk or an error message
fn read_bytecode_file(input_path: &str) -> CliResult<Chunk> {
    let file = File::open(input_path).map_err(|e| CliError::Io {
        source: e,
        path: input_path.to_string(),
        exit_code: exit::Code::NoInput,
    })?;

    let mut archive = ZipArchive::new(file).map_err(|e| CliError::Zip {
        source: e,
        context: "Failed to read zip archive".to_string(),
        exit_code: exit::Code::Dataerr,
    })?;

    if let Ok(mut bytecode_file) = archive.by_name("bytecode.bin") {
        let mut buffer = Vec::new();
        std::io::copy(&mut bytecode_file, &mut buffer).map_err(|e| CliError::Io {
            source: e,
            path: input_path.to_string(),
            exit_code: exit::Code::IoErr,
        })?;

        let mut cursor = std::io::Cursor::new(buffer);
        let chunk = Chunk::deserialize(&mut cursor).map_err(|e| CliError::Serialization {
            source: Box::new(e),
            context: "Failed to deserialize bytecode".to_string(),
            exit_code: exit::Code::Dataerr,
        })?;

        Ok(chunk)
    } else {
        Err(CliError::Generic {
            message: "Invalid bytecode file format: missing bytecode.bin".to_string(),
            exit_code: exit::Code::Dataerr,
        })
    }
}

/// Execute a bytecode chunk in the VM
///
/// ### Arguments
///
/// * `chunk` - The bytecode chunk to run
///
/// ### Returns
///
/// Ok(()) if successful, or an error message
fn execute_bytecode(chunk: &Chunk) -> Result<(), String> {
    let mut vm = VM::new();
    vm.interpret(chunk)
}
