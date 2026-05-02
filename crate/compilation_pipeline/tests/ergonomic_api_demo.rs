use slang_compilation_pipeline::SlangSourceFile;
use slang_compilation_pipeline::{
    execution_chain::{ExecuteChain, ExecutionChain},
    stage::StageContext,
    stages::{ParsingStage, SemanticAnalysisStage, TokenizationStage},
};
use slang_shared::DiagnosticEngine;

#[test]
fn new_ergonomic_api_test() {
    let source_file = SlangSourceFile::new("test.sl", "let x = 42;".to_string()).unwrap();
    let mut context = StageContext::new(
        source_file.content().to_string(),
        Some(source_file.file_name().to_string()),
    );
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
    let mut context = StageContext::new(
        source_file.content().to_string(),
        Some(source_file.file_name().to_string()),
    );
    let mut diagnostics = DiagnosticEngine::new();

    // Single stage with new API
    let result = ExecutionChain::starting_with(TokenizationStage).execute_chain(
        source_file,
        &mut context,
        &mut diagnostics,
    );

    assert!(result.is_ok());
}
