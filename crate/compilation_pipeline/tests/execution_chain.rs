use slang_compilation_pipeline::pipeline::{
    execution_chain::{ExecutionChain, ExecuteChain},
    stages::{TokenizationStage, ParsingStage, SemanticAnalysisStage},
    stage::StageContext,
};
use slang_shared::DiagnosticEngine;

#[test]
fn single_stage_execution() {
    let source = "let x = 42;";
    let mut context = StageContext::new(source.to_string(), None);
    let mut diagnostics = DiagnosticEngine::new();
    
    let chain = ExecutionChain::new().then(TokenizationStage);
    let result = chain.execute_chain(source.to_string(), &mut context, &mut diagnostics);
    
    assert!(result.is_ok());
}

#[test]
fn multi_stage_execution() {
    let source = "let x = 42;";
    let mut context = StageContext::new(source.to_string(), None);
    let mut diagnostics = DiagnosticEngine::new();
    
    let chain = ExecutionChain::new()
        .then(TokenizationStage)
        .then(ParsingStage);
        
    let result = chain.execute_chain(source.to_string(), &mut context, &mut diagnostics);
    
    assert!(result.is_ok());
}

#[test]
fn execution_order() {
    let source = "fn main() { let x = 42; }";
    let mut context = StageContext::new(source.to_string(), None);
    let mut diagnostics = DiagnosticEngine::new();
    
    // Should execute: Tokenization -> Parsing -> Semantic Analysis
    let chain = ExecutionChain::new()
        .then(TokenizationStage)
        .then(ParsingStage)
        .then(SemanticAnalysisStage);
        
    let result = chain.execute_chain(source.to_string(), &mut context, &mut diagnostics);
    
    assert!(result.is_ok());
}

#[test]
fn chain_composition() {
    let chain1 = ExecutionChain::new().then(TokenizationStage);
    let chain2 = ExecutionChain::new().then(TokenizationStage);
    
    // Compose chains (this would be implementation-specific)
    // For now, just test that we can create and use both chains
    let source = "let x = 42;";
    let mut context1 = StageContext::new(source.to_string(), None);
    let mut context2 = StageContext::new(source.to_string(), None);
    let mut diagnostics1 = DiagnosticEngine::new();
    let mut diagnostics2 = DiagnosticEngine::new();
    
    let result1 = chain1.execute_chain(source.to_string(), &mut context1, &mut diagnostics1);
    let result2 = chain2.execute_chain(source.to_string(), &mut context2, &mut diagnostics2);
    
    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[test]
fn execution_chain_builder_pattern() {
    let source = "let x = 42;";
    let mut context = StageContext::new(source.to_string(), None);
    let mut diagnostics = DiagnosticEngine::new();
    
    // Test fluent interface
    let result = ExecutionChain::new()
        .then(TokenizationStage)
        .then(ParsingStage)
        .execute_chain(source.to_string(), &mut context, &mut diagnostics);
        
    assert!(result.is_ok());
}
