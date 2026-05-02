use slang_compilation_pipeline::SlangSourceFile;
use slang_compilation_pipeline::{
    execution_chain::{ExecuteChain, ExecutionChain},
    stage::StageContext,
    stages::{ParsingStage, SemanticAnalysisStage, TokenizationStage},
};
use slang_shared::DiagnosticEngine;

#[test]
fn single_stage_execution() {
    let source_file = SlangSourceFile::new("test.sl", "let x = 42;".to_string()).unwrap();
    let mut context = StageContext::new(
        source_file.content().to_string(),
        Some(source_file.file_name().to_string()),
    );
    let mut diagnostics = DiagnosticEngine::new();

    let chain = ExecutionChain::starting_with(TokenizationStage);
    let result = chain.execute_chain(source_file, &mut context, &mut diagnostics);

    assert!(result.is_ok());
}

#[test]
fn multi_stage_execution() {
    let source_file = SlangSourceFile::new("test.sl", "let x = 42;".to_string()).unwrap();
    let mut context = StageContext::new(
        source_file.content().to_string(),
        Some(source_file.file_name().to_string()),
    );
    let mut diagnostics = DiagnosticEngine::new();

    let chain = ExecutionChain::starting_with(TokenizationStage).then(ParsingStage);

    let result = chain.execute_chain(source_file, &mut context, &mut diagnostics);

    assert!(result.is_ok());
}

#[test]
fn execution_order() {
    let source_file =
        SlangSourceFile::new("test.sl", "fn main() { let x = 42; }".to_string()).unwrap();
    let mut context = StageContext::new(
        source_file.content().to_string(),
        Some(source_file.file_name().to_string()),
    );
    let mut diagnostics = DiagnosticEngine::new();

    // Should execute: Tokenization -> Parsing -> Semantic Analysis
    let chain = ExecutionChain::starting_with(TokenizationStage)
        .then(ParsingStage)
        .then(SemanticAnalysisStage);

    let result = chain.execute_chain(source_file, &mut context, &mut diagnostics);

    assert!(result.is_ok());
}

#[test]
fn chain_composition() {
    let chain1 = ExecutionChain::starting_with(TokenizationStage);
    let chain2 = ExecutionChain::starting_with(TokenizationStage);

    // Compose chains (this would be implementation-specific)
    // For now, just test that we can create and use both chains
    let source_file1 = SlangSourceFile::new("test1.sl", "let x = 42;".to_string()).unwrap();
    let source_file2 = SlangSourceFile::new("test2.sl", "let x = 42;".to_string()).unwrap();
    let mut context1 = StageContext::new(
        source_file1.content().to_string(),
        Some(source_file1.file_name().to_string()),
    );
    let mut context2 = StageContext::new(
        source_file2.content().to_string(),
        Some(source_file2.file_name().to_string()),
    );
    let mut diagnostics1 = DiagnosticEngine::new();
    let mut diagnostics2 = DiagnosticEngine::new();

    let result1 = chain1.execute_chain(source_file1, &mut context1, &mut diagnostics1);
    let result2 = chain2.execute_chain(source_file2, &mut context2, &mut diagnostics2);

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[test]
fn execution_chain_builder_pattern() {
    let source_file = SlangSourceFile::new("test.sl", "let x = 42;".to_string()).unwrap();
    let mut context = StageContext::new(
        source_file.content().to_string(),
        Some(source_file.file_name().to_string()),
    );
    let mut diagnostics = DiagnosticEngine::new();

    let result = ExecutionChain::starting_with(TokenizationStage)
        .then(ParsingStage)
        .execute_chain(source_file, &mut context, &mut diagnostics);

    assert!(result.is_ok());
}
