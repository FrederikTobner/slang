use slang::compilation_pipeline::{CompilationPipeline, CompilationResult, PipelineStage};
use slang_backend::VM;
use slang_backend::bytecode::Chunk;
use slang_ir::ast::Statement;

/// Helper function to compile to bytecode using compilation_pipeline
pub fn compile_to_bytecode(program: &str) -> Result<Chunk, String> {
    let pipeline = CompilationPipeline::new(program, Some("benchmark.sl".to_string()), false);
    match pipeline.execute_all_stages() {
        CompilationResult::Success { chunk, .. } => Ok(chunk),
        CompilationResult::Failed { diagnostics } => {
            let error_msg = format!(
                "Compilation failed with {} errors",
                diagnostics.error_count()
            );
            Err(error_msg)
        }
    }
}

/// Helper function to parse only using CompilationPipeline
pub fn parse_only(program: &str) -> Result<Vec<Statement>, String> {
    let pipeline = CompilationPipeline::new(program, Some("benchmark.sl".to_string()), false);

    match pipeline
        .tokenize()
        .and_then(|pipeline, tokens| pipeline.parse(tokens))
    {
        PipelineStage::Success { data, .. } => Ok(data),
        PipelineStage::Failed { .. } => Err("AST compilation failed".to_string()),
    }
}

/// Helper function to perform semantic analysis using CompilationPipeline
pub fn semantic_analysis_only(program: &str) -> Result<Vec<Statement>, String> {
    let pipeline = CompilationPipeline::new(program, Some("benchmark.sl".to_string()), false);

    match pipeline
        .tokenize()
        .and_then(|pipeline, tokens| pipeline.parse(tokens))
        .and_then(|pipeline, statements| pipeline.semantic_analysis(statements))
    {
        PipelineStage::Success { data, .. } => Ok(data),
        PipelineStage::Failed { .. } => Err("Semantic analysis failed".to_string()),
    }
}

/// Helper function to execute a program using compilation_pipeline
pub fn execute_program(program: &str) -> Result<(), String> {
    let mut vm = VM::new();
    let pipeline = CompilationPipeline::new(program, Some("benchmark.sl".to_string()), false);
    match pipeline.execute_all_stages() {
        CompilationResult::Success { chunk, .. } => match vm.interpret(&chunk) {
            Ok(()) => Ok(()),
            Err(err) => Err(format!("VM execution failed: {err}")),
        },
        CompilationResult::Failed { diagnostics } => {
            let error_msg = format!(
                "Compilation failed with {} errors",
                diagnostics.error_count()
            );
            Err(error_msg)
        }
    }
}
