//! Simple test to demonstrate ExecutionChain with tail-first execution works correctly

use slang_compilation_pipeline::pipeline::{
    execution_chain::{ExecutionChain, ExecuteChain, TokenizationChain},
    stage::StageContext,
    stages::*,
};
use slang_shared::DiagnosticEngine;

/// Test that demonstrates the ExecutionChain works with correct execution order

#[test]
fn execution_chain_basic() {
    // Create a simple chain
    let chain = ExecutionChain::new();
    
    let mut context = StageContext::new("test".to_string(), None);
    let mut diagnostics = DiagnosticEngine::new();
    
    // Empty chain should just return the input
    let result = chain.execute_chain("test input".to_string(), &mut context, &mut diagnostics);
    assert_eq!(result.unwrap(), "test input");
}

#[test] 
fn tokenization_chain() {
    // Test the predefined tokenization chain
    let chain = TokenizationChain::tokenization();
    
    let mut context = StageContext::new("let x = 42;".to_string(), None);
    let mut diagnostics = DiagnosticEngine::new();
    
    // This should work: String → TokenizationStage → Vec<Token>
    let result = chain.execute_chain("let x = 42;".to_string(), &mut context, &mut diagnostics);
    
    match result {
        Ok(tokens) => {
            println!("✅ Tokenization successful! Got {} tokens", tokens.len());
            assert!(!tokens.is_empty());
        }
        Err(_) => {
            println!("❌ Tokenization failed");
            assert!(false, "Tokenization should succeed");
        }
    }
}

#[test]
fn manual_chain_construction() {
    // Test manual chain construction with correct execution order
    let chain = ExecutionChain::new()
        .then(TokenizationStage);  // This should work: String → TokenizationStage → Vec<Token>
    
    let mut context = StageContext::new("let x = 42;".to_string(), None);
    let mut diagnostics = DiagnosticEngine::new();
    
    let result = chain.execute_chain("let x = 42;".to_string(), &mut context, &mut diagnostics);
    
    match result {
        Ok(tokens) => {
            println!("✅ Manual chain successful! Got {} tokens", tokens.len());
            assert!(!tokens.is_empty());
        }
        Err(_) => {
            println!("❌ Manual chain failed");
            assert!(false, "Manual chain should succeed");
        }
    }
}
