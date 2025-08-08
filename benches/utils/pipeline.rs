use slang_compilation_pipeline::{
    ChainPipeline,
    result::CompilationResult as PipelineResult,
};
use slang_backend::VM;
use slang_backend::bytecode::Chunk;
use slang_ir::ast::Statement;
use slang_compilation_pipeline::SlangSourceFile;

/// Helper function to compile to bytecode using the new type-safe ChainPipeline
pub fn compile_to_bytecode(program: &str) -> Result<Chunk, String> {
    let source_file = SlangSourceFile::new("benchmark.sl", program.to_string());
    let result = ChainPipeline::full_compilation()
        .execute(source_file.unwrap());
    
    match result {
            PipelineResult::Success { output: chunk, .. } => Ok(chunk),
        PipelineResult::Failed { diagnostics } => {
            let error_msg = format!(
                "Compilation failed with {} errors",
                diagnostics.error_count()
            );
            Err(error_msg)
        }
    }
}

/// Helper function to parse only using the new ChainPipeline
pub fn parse_only(program: &str) -> Result<Vec<Statement>, String> {
    let source_file = SlangSourceFile::new("benchmark.sl", program.to_string());
    let result = ChainPipeline::parsing_only()
        .execute(source_file.unwrap());
    
    match result {
        PipelineResult::Success { output, .. } => Ok(output),
        PipelineResult::Failed { .. } => {
            Err("AST compilation failed".to_string())
        }
    }
}

/// Helper function to perform semantic analysis using the new ChainPipeline
pub fn semantic_analysis_only(program: &str) -> Result<Vec<Statement>, String> {
    let source_file = SlangSourceFile::new("benchmark.sl", program.to_string());
    let result = ChainPipeline::ast_compilation()
        .execute(source_file.unwrap());
        
    match result {
        PipelineResult::Success { output, .. } => Ok(output),
        PipelineResult::Failed { .. } => {
            Err("Semantic analysis failed".to_string())
        }
    }
}

/// Helper function to execute a program using the new ChainPipeline
pub fn execute_program(program: &str) -> Result<(), String> {
    let mut vm = VM::new();
    let source_file = SlangSourceFile::new("benchmark.sl", program.to_string());
    let result = ChainPipeline::full_compilation()
        .execute(source_file.unwrap());

    match result {
        PipelineResult::Success { output: chunk, .. } => match vm.interpret(&chunk) {
            Ok(()) => Ok(()),
            Err(err) => Err(format!("VM execution failed: {err}")),
        },
        PipelineResult::Failed { diagnostics } => {
            let error_msg = format!(
                "Compilation failed with {} errors",
                diagnostics.error_count()
            );
            Err(error_msg)
        }
    }
}
