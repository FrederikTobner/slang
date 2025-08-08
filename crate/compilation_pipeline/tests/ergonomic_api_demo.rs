use slang_compilation_pipeline::{
    execution_chain::{ExecutionChain, ExecuteChain},
    stages::{TokenizationStage, ParsingStage, SemanticAnalysisStage},
    stage::StageContext,
};
use slang_shared::DiagnosticEngine;
use slang_compilation_pipeline::SlangSourceFile;

#[test]
fn new_ergonomic_api_test() {
    let source_file = SlangSourceFile::new("test.sl", "let x = 42;".to_string()).unwrap();
    let mut context = StageContext::new(source_file.content().to_string(), Some(source_file.file_name().to_string()));
    let mut diagnostics = DiagnosticEngine::new();
    
    // New ergonomic API - start with the first stage directly
    let result = ExecutionChain::starting_with(TokenizationStage)
        .then(ParsingStage)
        .then(SemanticAnalysisStage)
        .execute_chain(source_file, &mut context, &mut diagnostics);
        
    assert!(result.is_ok());
}

#[test]
fn single_stage_ergonomic_api_test() {
    let source_file = SlangSourceFile::new("test.sl", "let x = 42;".to_string()).unwrap();
    let mut context = StageContext::new(source_file.content().to_string(), Some(source_file.file_name().to_string()));
    let mut diagnostics = DiagnosticEngine::new();
    
    // Single stage with new API
    let result = ExecutionChain::starting_with(TokenizationStage)
        .execute_chain(source_file, &mut context, &mut diagnostics);
        
    assert!(result.is_ok());
}
