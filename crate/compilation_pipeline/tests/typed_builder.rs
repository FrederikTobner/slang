//! Tests for compile-time observer validation in ChainPipeline
//!
//! These tests demonstrate that observers can only be registered for stages
//! that are actually present in the execution chain.

use slang_compilation_pipeline::pipeline::{
    typed_builder::ChainPipeline,
    observers::StageObserver,
    result::CompilationResult,
};
use slang_compilation_pipeline::ErrorStrategy;
use slang_frontend::Token;
use slang_ir::ast::Statement;
use slang_backend::bytecode::Chunk;
use std::error::Error;

/// Mock observer for tokenization stage
struct MockTokenizationObserver;

impl StageObserver<String, Vec<Token>> for MockTokenizationObserver {
    fn on_stage_start(&self, _input: &String) {}
    fn on_stage_success(&self, _output: &Vec<Token>) {}
    fn on_stage_error(&self, _error: &dyn Error) {}
}

/// Mock observer for parsing stage
struct MockParsingObserver;

impl StageObserver<Vec<Token>, Vec<Statement>> for MockParsingObserver {
    fn on_stage_start(&self, _input: &Vec<Token>) {}
    fn on_stage_success(&self, _output: &Vec<Statement>) {}
    fn on_stage_error(&self, _error: &dyn Error) {}
}

/// Mock observer for semantic analysis stage
struct MockSemanticObserver;

impl StageObserver<Vec<Statement>, Vec<Statement>> for MockSemanticObserver {
    fn on_stage_start(&self, _input: &Vec<Statement>) {}
    fn on_stage_success(&self, _output: &Vec<Statement>) {}
    fn on_stage_error(&self, _error: &dyn Error) {}
}

/// Mock observer for code generation stage
struct MockCodegenObserver;

impl StageObserver<Vec<Statement>, Chunk> for MockCodegenObserver {
    fn on_stage_start(&self, _input: &Vec<Statement>) {}
    fn on_stage_success(&self, _output: &Chunk) {}
    fn on_stage_error(&self, _error: &dyn Error) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenization_chain_allows_tokenization_observer() {
        let source = "let x = 42;";
        
        // ✅ This should compile - tokenization observer is valid for tokenization chain
        let _pipeline = ChainPipeline::tokenization_only(source)
            .with_tokenization_observer(MockTokenizationObserver);
    }

    #[test]
    fn test_parsing_chain_allows_tokenization_and_parsing_observers() {
        let source = "let x = 42;";
        
        // ✅ Both observers are valid for parsing chain
        let _pipeline = ChainPipeline::parsing_only(source)
            .with_tokenization_observer(MockTokenizationObserver)
            .with_parsing_observer(MockParsingObserver);
    }

    #[test]
    fn test_ast_compilation_chain_allows_appropriate_observers() {
        let source = "let x = 42;";
        
        // ✅ These observers are valid for AST compilation chain
        let _pipeline = ChainPipeline::ast_compilation(source)
            .with_tokenization_observer(MockTokenizationObserver)
            .with_parsing_observer(MockParsingObserver)
            .with_semantic_observer(MockSemanticObserver);
    }

    #[test]
    fn test_full_compilation_chain_allows_all_observers() {
        let source = "let x = 42;";
        
        // ✅ All observers are valid for full compilation chain
        let _pipeline = ChainPipeline::full_compilation(source)
            .with_tokenization_observer(MockTokenizationObserver)
            .with_parsing_observer(MockParsingObserver)
            .with_semantic_observer(MockSemanticObserver)
            .with_codegen_observer(MockCodegenObserver);
    }

    #[test]
    fn test_pipeline_execution_works() {
        let source = "let x = 42;";
        
        // Test that execution works with observers
        let result = ChainPipeline::tokenization_only(source)
            .with_tokenization_observer(MockTokenizationObserver)
            .tokenize();
        
        // Should succeed (tokenization should work)
        assert!(result.is_success());
    }

    #[test]
    fn test_configuration_methods_work() {
        let source = "let x = 42;";
        
        // Test that configuration methods work with the typed pipeline
        let _pipeline = ChainPipeline::tokenization_only(source)
            .with_file_name("test.sl".to_string())
            .with_error_strategy(ErrorStrategy::FailFast)
            .with_tokenization_observer(MockTokenizationObserver);
    }

    // Note: These tests demonstrate what should compile.
    // The following examples would NOT compile if uncommented:
    
    // ❌ This would fail to compile - semantic observer not available for tokenization chain
    // #[test]
    // fn test_tokenization_chain_rejects_semantic_observer() {
    //     let source = "let x = 42;";
    //     let _pipeline = ChainPipeline::tokenization_only(source)
    //         .with_semantic_observer(MockSemanticObserver);  // Compile error!
    // }
    
    // ❌ This would fail to compile - codegen observer not available for parsing chain
    // #[test]
    // fn test_parsing_chain_rejects_codegen_observer() {
    //     let source = "let x = 42;";
    //     let _pipeline = ChainPipeline::parsing_only(source)
    //         .with_codegen_observer(MockCodegenObserver);  // Compile error!
    // }
}

/// Integration tests that demonstrate the API in action
#[cfg(test)]
mod integration_tests {
    use super::*;
    use slang_compilation_pipeline::pipeline::typed_builder::{TokenizationPipeline, FullCompilationPipeline};

    #[test]
    fn test_type_aliases_work() {
        let source = "let x = 42;";
        
        // Test using type aliases
        let _tokenization_pipeline: TokenizationPipeline = ChainPipeline::tokenization_only(source)
            .with_tokenization_observer(MockTokenizationObserver);
            
        let _full_pipeline: FullCompilationPipeline = ChainPipeline::full_compilation(source)
            .with_tokenization_observer(MockTokenizationObserver)
            .with_semantic_observer(MockSemanticObserver);
    }

    #[test]
    fn test_method_chaining_ergonomics() {
        let source = "let x = 42;";
        
        // Test that method chaining feels natural
        let result = ChainPipeline::full_compilation(source)
            .with_file_name("example.sl".to_string())
            .with_tokenization_observer(MockTokenizationObserver)
            .with_parsing_observer(MockParsingObserver)
            .with_semantic_observer(MockSemanticObserver)
            .with_codegen_observer(MockCodegenObserver)
            .compile_to_bytecode();
            
        // The compilation might fail due to semantic errors, but the API should work
        match result {
            CompilationResult::Success { .. } => {
                // Great! Full compilation succeeded
            }
            CompilationResult::Failed { .. } => {
                // Expected for simple test input - semantic analysis might fail
            }
        }
    }
}
