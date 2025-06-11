use crate::compilation_pipeline::CompilationResult;
use crate::compiler::{CompileOptions, Compiler};
use crate::error::{CliError, CliResult};
use crate::exit;
use clap::{Parser as ClapParser, Subcommand};
use colored::Colorize;
use slang_backend::bytecode::Chunk;
use slang_backend::vm;
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
It features compilation to bytecode and execution of both source files and compiled bytecode."#,
    arg_required_else_help = true
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

/// Represents different execution modes for source file processing
enum ExecutionMode {
    /// Compile source to bytecode
    Compile { output_path: String },
    /// Execute source directly
    Execute,
}

/// Run a compiled Slang bytecode file
///
/// ### Arguments
/// * `input` - The input compiled bytecode file
pub fn run_file(input: &str) -> CliResult<()> {
    println!("Running compiled file: {}", input);

    // Validate file extension for better user experience
    validate_file_extension(input, SLANG_BYTECODE_EXTENSION, "bytecode execution")?;

    let chunk = read_bytecode_from_file(input)?;
    vm::execute_bytecode(&chunk).map_err(|e| CliError::Generic {
        message: format!("{}: {} (in file '{}')", "Runtime Error".red(), e, input),
        exit_code: exit::Code::Software,
    })?;

    Ok(())
}

/// Process a source file for either compilation or execution
///
/// ### Arguments
/// * `input` - The input source file path
/// * `mode` - The execution mode (compile or execute)
///
/// ### Returns
/// Result indicating success or failure
fn process_source_file(input: &str, mode: ExecutionMode) -> CliResult<()> {
    let source = read_source_file(input)?;
    let compiler = Compiler::new();
    let recovery_mode = matches!(mode, ExecutionMode::Execute);

    let compile_options = CompileOptions {
        recovery_mode,
        file_name: Some(input.to_string()),
    };

    let result = compiler.compile_source(&source, compile_options);

    match result {
        CompilationResult::Success {
            chunk, diagnostics, ..
        } => {
            let has_diagnostics = diagnostics.error_count() > 0 || diagnostics.warning_count() > 0;
            if has_diagnostics {
                diagnostics.report_all(&source);
            }

            match mode {
                ExecutionMode::Compile { output_path } => {
                    write_bytecode(&chunk, &output_path)?;
                    println!("Successfully compiled to {}", output_path);
                }
                ExecutionMode::Execute => {
                    vm::execute_bytecode(&chunk).map_err(|e| CliError::Generic {
                        message: format!("{}: {} (in file '{}')", "Runtime Error".red(), e, input),
                        exit_code: exit::Code::Software,
                    })?;
                }
            }
            Ok(())
        }
        CompilationResult::Failed { diagnostics, .. } => {
            diagnostics.report_all(&source);
            Err(CliError::Generic {
                message: format!("Compilation failed for file '{}'", input),
                exit_code: exit::Code::Software,
            })
        }
    }
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

/// Read source code from a file
///
/// ### Arguments
/// * `path` - The path to the source file
///
/// ### Returns
/// The file contents as a string, or a CliError on failure with enhanced context
fn read_source_file(path: &str) -> CliResult<String> {
    fs::read_to_string(path).map_err(|e| {
        let error = CliError::from_io_error(e, path);
        if let CliError::Io {
            source,
            path,
            exit_code,
        } = error
        {
            CliError::Io {
                source,
                path: format!("{} (attempted to read source file)", path),
                exit_code,
            }
        } else {
            error
        }
    })
}

/// Write a bytecode chunk to a compressed archive file
///
/// ### Arguments
/// * `chunk` - The bytecode chunk to write
/// * `output_path` - The path to write the archive to
///
/// ### Returns
/// Ok(()) if successful, or a CliError on failure
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
            context: "Failed to create zip entry",
            exit_code: exit::Code::IoErr,
        })?;

    {
        let mut cursor = std::io::Cursor::new(Vec::new());
        chunk
            .serialize(&mut cursor)
            .map_err(|e| CliError::Serialization {
                source: Box::new(e),
                context: "Failed to serialize bytecode",
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
        context: "Failed to finalize zip file",
        exit_code: exit::Code::IoErr,
    })?;

    Ok(())
}

/// Read a bytecode chunk from a compressed archive file
///
/// ### Arguments
/// * `input_path` - The path to read the archive from
///
/// ### Returns
/// The bytecode chunk, or a CliError on failure
fn read_bytecode_from_file(input_path: &str) -> CliResult<Chunk> {
    let file = File::open(input_path).map_err(|e| CliError::Io {
        source: e,
        path: input_path.to_string(),
        exit_code: exit::Code::NoInput,
    })?;

    let mut archive = ZipArchive::new(file).map_err(|e| CliError::Zip {
        source: e,
        context: "Failed to read zip archive",
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
            context: "Failed to deserialize bytecode",
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

/// Validate that a file has the expected extension for its operation
///
/// ### Arguments
/// * `path` - The file path to validate
/// * `expected_ext` - The expected file extension (without the dot)
/// * `operation` - Description of the operation for error messages
///
/// ### Returns
/// Ok(()) if valid, or a CliError if the extension is incorrect
fn validate_file_extension(path: &str, expected_ext: &str, operation: &str) -> CliResult<()> {
    let path_obj = Path::new(path);
    if let Some(ext) = path_obj.extension() {
        if ext.to_str().unwrap_or("") == expected_ext {
            Ok(())
        } else {
            Err(CliError::Generic {
                message: format!(
                    "Invalid file extension for {}: expected '.{}', got '.{}' (file: '{}')",
                    operation,
                    expected_ext,
                    ext.to_str().unwrap_or("?"),
                    path
                ),
                exit_code: exit::Code::Usage,
            })
        }
    } else {
        Err(CliError::Generic {
            message: format!(
                "Missing file extension for {}: expected '.{}' (file: '{}')",
                operation, expected_ext, path
            ),
            exit_code: exit::Code::Usage,
        })
    }
}

/// Compile a Slang source file to bytecode with enhanced error handling
///
/// ### Arguments
/// * `input` - The input source file
/// * `output` - The output file path (if provided)
pub fn compile_file(input: &str, output: Option<String>) -> CliResult<()> {
    let output_path = resolve_output_path(input, output);
    println!("Compiling {} to {}", input, output_path);
    process_source_file(input, ExecutionMode::Compile { output_path })
}

/// Execute a Slang source file with enhanced error handling and diagnostics
///
/// ### Arguments
/// * `input` - The input source file
pub fn execute_file(input: &str) -> CliResult<()> {
    println!("Executing source file: {}", input);
    process_source_file(input, ExecutionMode::Execute)
}

