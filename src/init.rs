use crate::bytecode::Chunk;
use crate::compiler::compile;
use crate::parser::parse;
use crate::type_checker::TypeChecker;
use crate::vm::VM;
use colored::Colorize;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use zip::{ZipArchive, ZipWriter, write::FileOptions};

/// The extension for compiled Slang binaries
const SLANG_BYTECODE_EXTENSION: &str = "sip";

pub fn execute(input: &str) {
    println!("Executing source file: {}", input);
    match read_source_file(input) {
        Ok(source) => match compile_source(&source) {
            Ok(chunk) => {
                if let Err(e) = run_bytecode(&chunk) {
                    eprintln!("{}: {}", "Runtime Error".red(), e);
                }
            }
            Err(e) => {
                eprintln!(
                    "{}\n{}:could not compile due to previous error",
                    e,
                    "error".red(),
                );
            }
        },
        Err(e) => eprintln!("{}", e),
    }
}

pub fn run(input: &str) {
    println!("Running compiled file: {}", input);
    match read_bytecode(input) {
        Ok(chunk) => {
            if let Err(e) = run_bytecode(&chunk) {
                eprintln!("{}: {}", "Runtime Error".red(), e);
            }
        }
        Err(e) => eprintln!("Failed to load bytecode: {}", e),
    }
}

pub fn comp(input: &str, output: Option<String>) {
    let output_path = match output {
        Some(path) => path.clone(),
        None => {
            let path = Path::new(input);
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            format!("{}.{}", stem, SLANG_BYTECODE_EXTENSION)
        }
    };

    println!("Compiling {} to {}", input, output_path);

    match read_source_file(input) {
        Ok(source) => match compile_source(&source) {
            Ok(chunk) => {
                if let Err(e) = write_bytecode(&chunk, &output_path) {
                    eprintln!("Failed to write bytecode: {}", e);
                } else {
                    println!("Successfully compiled to {}", output_path);
                }
            }
            Err(e) => {
                eprintln!(
                    "{}\n{}:could not compile due to previous error",
                    e,
                    "error".red(),
                );
            }
        },
        Err(e) => eprintln!("{}", e),
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
    let lexer_result = crate::lexer::tokenize(source);
    let ast = parse(&lexer_result.tokens, &lexer_result.line_info)?;

    crate::type_checker::execute(&ast)?;

    let chunk = compile(&ast)?;

    Ok(chunk)
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
    let file = File::create(path).map_err(|e| format!("Failed to create output file: {}", e))?;

    let mut zip = ZipWriter::new(file);
    let options = FileOptions::<()>::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    zip.start_file("bytecode.bin", options)
        .map_err(|e| format!("Failed to create zip entry: {}", e))?;

    {
        let mut cursor = std::io::Cursor::new(Vec::new());
        chunk
            .serialize(&mut cursor)
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

    let mut archive =
        ZipArchive::new(file).map_err(|e| format!("Failed to read zip archive: {}", e))?;

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
pub fn repl() {
    let mut vm = VM::new();
    let mut type_checker = TypeChecker::new();
    println!("Slang REPL - Type 'exit' to exit");

    loop {
        let mut input = String::new();
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        let trimmed;
        if std::io::stdin().read_line(&mut input).is_ok() {
            trimmed = input.trim();
            if trimmed == "exit" {
                break;
            }
        } else {
            println!("Error reading input. Try again.");
            continue;
        }

        let lexer_result = crate::lexer::tokenize(&input);
        if lexer_result.tokens.len() <= 1 {
            // Just EOF token
            continue;
        }

        match parse(&lexer_result.tokens, &lexer_result.line_info) {
            Ok(ast) => {
                #[cfg(feature = "print-ast")]
                {
                    println!("\n=== AST ===");
                    let mut printer = crate::ast_printer::ASTPrinter::new();
                    printer.print(&ast);
                }
                match type_checker.check(&ast) {
                    Ok(_) => {
                        match compile(&ast) {
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
                            Err(e) => {
                                eprintln!(
                                    "{}\n{}:could not compile due to previous error",
                                    e,
                                    "error".red(),
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "{}\n{}: Unable to compile due to previous error",
                            "Type error".red(),
                            e
                        );
                    }
                }
            }
            Err(e) => eprintln!(
                "{}\n{}: Unable to parse input due to previous error",
                e,
                "Parse error".red()
            ),
        }
    }
}
