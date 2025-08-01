use slang_compilation_pipeline::{CompilationPipeline, CompilationResult, PipelineBuilder};
use slang_compilation_pipeline::pipeline::stages::*;
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

/// Helper function to parse only using PipelineBuilder
pub fn parse_only(program: &str) -> Result<Vec<Statement>, String> {
    let pipeline = PipelineBuilder::new(program)
        .add_stage(TokenizationStage)
        .add_stage(ParsingStage)
        .build();
    
    match pipeline.execute() {
        slang_compilation_pipeline::pipeline::result::CompilationResult::Success { output, .. } => {
            // Try to downcast to Vec<Statement>
            match output.downcast::<Vec<Statement>>() {
                Ok(statements) => Ok(*statements),
                Err(_) => Err("Failed to extract AST from pipeline output".to_string()),
            }
        }
        slang_compilation_pipeline::pipeline::result::CompilationResult::Failed { .. } => {
            Err("AST compilation failed".to_string())
        }
    }
}

/// Helper function to perform semantic analysis using PipelineBuilder
pub fn semantic_analysis_only(program: &str) -> Result<Vec<Statement>, String> {
    let pipeline = PipelineBuilder::new(program)
        .add_stage(TokenizationStage)
        .add_stage(ParsingStage)
        .add_stage(SemanticAnalysisStage)
        .build();
        
    match pipeline.execute() {
        slang_compilation_pipeline::pipeline::result::CompilationResult::Success { output, .. } => {
            // Try to downcast to Vec<Statement>
            match output.downcast::<Vec<Statement>>() {
                Ok(statements) => Ok(*statements),
                Err(_) => Err("Failed to extract statements from pipeline output".to_string()),
            }
        }
        slang_compilation_pipeline::pipeline::result::CompilationResult::Failed { .. } => {
            Err("Semantic analysis failed".to_string())
        }
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
