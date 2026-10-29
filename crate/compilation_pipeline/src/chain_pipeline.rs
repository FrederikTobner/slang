//! Simplified Chain-Aware Pipeline for Slang Compilation
//!
//! This module implements a straightforward pipeline that supports different
//! execution chains with observer registration. Observers may be registered
//! for any stage, but will only be called if that stage executes.

use crate::error::ErrorStrategy;
use crate::execution_chain::{
    ASTChain, ExecuteChain, FullCompilationChain, ParsingChain, TokenizationChain,
};
use crate::observer::{ObserverRegistry, StageObserver};
use crate::result::CompilationResult;
use crate::source_file::SlangSourceFile;
use crate::stage::StageContext;
use slang_backend::bytecode::Chunk;
use slang_frontend::Token;
use slang_ir::ast::Statement;
use slang_shared::DiagnosticEngine;

/// Simple pipeline that knows its execution chain.
/// Observers can be registered for any stage, but will only be called
/// if that stage is included in the execution chain.
pub struct ChainPipeline<Chain> {
    error_strategy: ErrorStrategy,
    observer_registry: ObserverRegistry,
    chain: Chain,
}

/// Type aliases for common pipeline configurations
pub type TokenizationPipeline = ChainPipeline<TokenizationChain>;
pub type ParsingPipeline = ChainPipeline<ParsingChain>;
pub type ASTPipeline = ChainPipeline<ASTChain>;
pub type FullCompilationPipeline = ChainPipeline<FullCompilationChain>;

impl<Chain> ChainPipeline<Chain> {
    /// Create a new pipeline with the given execution chain.
    pub fn new(chain: Chain) -> Self {
        Self {
            error_strategy: ErrorStrategy::FailFast,
            observer_registry: ObserverRegistry::new(),
            chain,
        }
    }
}

/// Convenience constructors for common chain types
impl ChainPipeline<TokenizationChain> {
    /// Create a pipeline that only performs tokenization
    pub fn tokenization_only() -> Self {
        Self::new(TokenizationChain::tokenization())
    }
}

impl ChainPipeline<ParsingChain> {
    /// Create a pipeline that performs tokenization and parsing
    pub fn parsing_only() -> Self {
        Self::new(ParsingChain::parsing())
    }
}

impl ChainPipeline<ASTChain> {
    /// Create a pipeline that performs tokenization, parsing, and semantic analysis
    pub fn ast_compilation() -> Self {
        Self::new(ASTChain::ast_compilation())
    }
}

impl ChainPipeline<FullCompilationChain> {
    /// Create a pipeline for full compilation to bytecode
    pub fn full_compilation() -> Self {
        Self::new(FullCompilationChain::full_compilation())
    }
}

/// Common configuration methods available for all chains
impl<Chain> ChainPipeline<Chain> {
    /// Set the error handling strategy.
    pub fn with_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.error_strategy = strategy;
        self
    }

    /// Add a tokenization observer to monitor tokenization stage.
    /// Note: Observer will only be called if the chain includes tokenization.
    pub fn with_tokenization_observer<T>(mut self, observer: T) -> Self
    where
        T: StageObserver<SlangSourceFile, Vec<Token>> + 'static,
    {
        self.observer_registry.add_tokenization_observer(observer);
        self
    }

    /// Add a parsing observer to monitor parsing stage.
    /// Note: Observer will only be called if the chain includes parsing.
    pub fn with_parsing_observer<T>(mut self, observer: T) -> Self
    where
        T: StageObserver<Vec<Token>, Vec<Statement>> + 'static,
    {
        self.observer_registry.add_parsing_observer(observer);
        self
    }

    /// Add a semantic analysis observer to monitor semantic analysis stage.
    /// Note: Observer will only be called if the chain includes semantic analysis.
    pub fn with_semantic_observer<T>(mut self, observer: T) -> Self
    where
        T: StageObserver<Vec<Statement>, Vec<Statement>> + 'static,
    {
        self.observer_registry.add_semantic_observer(observer);
        self
    }

    /// Add a code generation observer to monitor code generation stage.
    /// Note: Observer will only be called if the chain includes code generation.
    pub fn with_codegen_observer<T>(mut self, observer: T) -> Self
    where
        T: StageObserver<Vec<Statement>, Chunk> + 'static,
    {
        self.observer_registry.add_codegen_observer(observer);
        self
    }
}

/// Execution methods for pipelines
impl<Chain> ChainPipeline<Chain>
where
    Chain: ExecuteChain<SlangSourceFile>,
{
    /// Execute the configured pipeline and return the result.
    pub fn execute(
        self,
        source_file: SlangSourceFile,
    ) -> CompilationResult<'static, Chain::Output> {
        let mut context = StageContext::with_observer_registry(
            source_file.content().to_string(),
            Some(source_file.file_name().to_string()),
            self.observer_registry,
        );
        let mut diagnostics = DiagnosticEngine::new();

        match self
            .chain
            .execute_chain(source_file, &mut context, &mut diagnostics)
        {
            Ok(output) => CompilationResult::Success {
                output,
                diagnostics,
            },
            Err(_) => CompilationResult::Failed { diagnostics },
        }
    }
}
