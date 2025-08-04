//! Chain-Aware Pipeline Type for Compile-Time Observer Validation
//!
//! This module implements a type-safe pipeline that encodes the execution chain
//! at compile time, ensuring observers can only be registered for stages that
//! will actually execute.

use crate::pipeline::{
    execution_chain::{ExecuteChain, TokenizationChain, ParsingChain, ASTChain, FullCompilationChain},
    result::CompilationResult,
    stage::StageContext,
    observers::{ObserverRegistry, StageObserver},
    hlist::{HList, HCons, HNil},
};
use crate::ErrorStrategy;
use slang_shared::DiagnosticEngine;
use slang_ir::ast::Statement;
use slang_frontend::Token;
use slang_backend::bytecode::Chunk;

// Type-level stage markers
pub struct TokenizationStageMarker;
pub struct ParsingStageMarker;  
pub struct SemanticStageMarker;
pub struct CodegenStageMarker;

// Simple trait to check if a stage exists in a type-level list
// We'll implement this for specific combinations to avoid conflicts
pub trait HasStage<Stage> {}

// Direct match implementations for each stage at the head with any tail (including HNil) 
impl<Tail: HList> HasStage<TokenizationStageMarker> for HCons<TokenizationStageMarker, Tail> {}
impl<Tail: HList> HasStage<ParsingStageMarker> for HCons<ParsingStageMarker, Tail> {}
impl<Tail: HList> HasStage<SemanticStageMarker> for HCons<SemanticStageMarker, Tail> {}
impl<Tail: HList> HasStage<CodegenStageMarker> for HCons<CodegenStageMarker, Tail> {}

// Recursive implementations - only when the head is different
impl<Tail: HList> HasStage<TokenizationStageMarker> for HCons<ParsingStageMarker, Tail> where Tail: HasStage<TokenizationStageMarker> {}
impl<Tail: HList> HasStage<TokenizationStageMarker> for HCons<SemanticStageMarker, Tail> where Tail: HasStage<TokenizationStageMarker> {}  
impl<Tail: HList> HasStage<TokenizationStageMarker> for HCons<CodegenStageMarker, Tail> where Tail: HasStage<TokenizationStageMarker> {}

impl<Tail: HList> HasStage<ParsingStageMarker> for HCons<TokenizationStageMarker, Tail> where Tail: HasStage<ParsingStageMarker> {}
impl<Tail: HList> HasStage<ParsingStageMarker> for HCons<SemanticStageMarker, Tail> where Tail: HasStage<ParsingStageMarker> {}
impl<Tail: HList> HasStage<ParsingStageMarker> for HCons<CodegenStageMarker, Tail> where Tail: HasStage<ParsingStageMarker> {}

impl<Tail: HList> HasStage<SemanticStageMarker> for HCons<TokenizationStageMarker, Tail> where Tail: HasStage<SemanticStageMarker> {}
impl<Tail: HList> HasStage<SemanticStageMarker> for HCons<ParsingStageMarker, Tail> where Tail: HasStage<SemanticStageMarker> {}
impl<Tail: HList> HasStage<SemanticStageMarker> for HCons<CodegenStageMarker, Tail> where Tail: HasStage<SemanticStageMarker> {}

impl<Tail: HList> HasStage<CodegenStageMarker> for HCons<TokenizationStageMarker, Tail> where Tail: HasStage<CodegenStageMarker> {}
impl<Tail: HList> HasStage<CodegenStageMarker> for HCons<ParsingStageMarker, Tail> where Tail: HasStage<CodegenStageMarker> {}
impl<Tail: HList> HasStage<CodegenStageMarker> for HCons<SemanticStageMarker, Tail> where Tail: HasStage<CodegenStageMarker> {}

/// Type-safe pipeline that knows its execution chain at compile time
/// The Stages type parameter encodes which stages are available as a type-level list
pub struct ChainPipeline<'a, Chain, Stages: HList> {
    source: &'a str,
    file_name: Option<String>,
    error_strategy: ErrorStrategy,
    observer_registry: ObserverRegistry,
    chain: Chain,
    _stages: std::marker::PhantomData<Stages>,
}

/// Type aliases for common pipeline configurations with explicit stage lists
pub type TokenizationPipeline<'a> = ChainPipeline<'a, TokenizationChain, HCons<TokenizationStageMarker, HNil>>;
pub type ParsingPipeline<'a> = ChainPipeline<'a, ParsingChain, HCons<ParsingStageMarker, HCons<TokenizationStageMarker, HNil>>>;
pub type ASTPipeline<'a> = ChainPipeline<'a, ASTChain, HCons<SemanticStageMarker, HCons<ParsingStageMarker, HCons<TokenizationStageMarker, HNil>>>>;
pub type FullCompilationPipeline<'a> = ChainPipeline<'a, FullCompilationChain, HCons<CodegenStageMarker, HCons<SemanticStageMarker, HCons<ParsingStageMarker, HCons<TokenizationStageMarker, HNil>>>>>;

impl<'a, Chain, Stages: HList> ChainPipeline<'a, Chain, Stages> {
    /// Create a new typed pipeline with the given source code and execution chain.
    /// The chain type determines which observers can be registered at compile time.
    pub fn new(source: &'a str, chain: Chain, stages: std::marker::PhantomData<Stages>) -> Self {
        Self {
            source,
            file_name: None,
            error_strategy: ErrorStrategy::FailFast,
            observer_registry: ObserverRegistry::new(),
            chain,
            _stages: stages,
        }
    }
}

/// Convenience constructors for common chain types
impl<'a> ChainPipeline<'a, TokenizationChain, HCons<TokenizationStageMarker, HNil>> {
    /// Create a pipeline that only performs tokenization
    pub fn tokenization_only(source: &'a str) -> Self {
        Self::new(source, TokenizationChain::tokenization(), std::marker::PhantomData)
    }
}

impl<'a> ChainPipeline<'a, ParsingChain, HCons<ParsingStageMarker, HCons<TokenizationStageMarker, HNil>>> {
    /// Create a pipeline that performs tokenization and parsing
    pub fn parsing_only(source: &'a str) -> Self {
        Self::new(source, ParsingChain::parsing(), std::marker::PhantomData)
    }
}

impl<'a> ChainPipeline<'a, ASTChain, HCons<SemanticStageMarker, HCons<ParsingStageMarker, HCons<TokenizationStageMarker, HNil>>>> {
    /// Create a pipeline that performs tokenization, parsing, and semantic analysis
    pub fn ast_compilation(source: &'a str) -> Self {
        Self::new(source, ASTChain::ast_compilation(), std::marker::PhantomData)
    }
}

impl<'a> ChainPipeline<'a, FullCompilationChain, HCons<CodegenStageMarker, HCons<SemanticStageMarker, HCons<ParsingStageMarker, HCons<TokenizationStageMarker, HNil>>>>> {
    /// Create a pipeline for full compilation to bytecode
    pub fn full_compilation(source: &'a str) -> Self {
        Self::new(source, FullCompilationChain::full_compilation(), std::marker::PhantomData)
    }
}

/// Common configuration methods available for all chains
impl<'a, Chain, Stages: HList> ChainPipeline<'a, Chain, Stages> {
    /// Set the file name for error reporting and diagnostics.
    pub fn with_file_name(mut self, file_name: String) -> Self {
        self.file_name = Some(file_name);
        self
    }

    /// Set the error handling strategy.
    pub fn with_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.error_strategy = strategy;
        self
    }
}

/// Observer methods only available for chains that include tokenization stage in their type-level list
impl<'a, Chain, Stages: HList> ChainPipeline<'a, Chain, Stages>
where
    Stages: HasStage<TokenizationStageMarker>,
{
    /// Add a tokenization observer to monitor tokenization stage.
    /// Only available for chains that include tokenization.
    pub fn with_tokenization_observer<T>(mut self, observer: T) -> Self 
    where 
        T: StageObserver<String, Vec<Token>> + 'static 
    {
        self.observer_registry.add_tokenization_observer(observer);
        self
    }
}

/// Observer methods only available for chains that include parsing stage in their type-level list
impl<'a, Chain, Stages: HList> ChainPipeline<'a, Chain, Stages>
where
    Stages: HasStage<ParsingStageMarker>,
{
    /// Add a parsing observer to monitor parsing stage.
    /// Only available for chains that include parsing.
    pub fn with_parsing_observer<T>(mut self, observer: T) -> Self 
    where 
        T: StageObserver<Vec<Token>, Vec<Statement>> + 'static 
    {
        self.observer_registry.add_parsing_observer(observer);
        self
    }
}

/// Observer methods only available for chains that include semantic analysis stage in their type-level list
impl<'a, Chain, Stages: HList> ChainPipeline<'a, Chain, Stages>
where
    Stages: HasStage<SemanticStageMarker>,
{
    /// Add a semantic analysis observer to monitor semantic analysis stage.
    /// Only available for chains that include semantic analysis.
    pub fn with_semantic_observer<T>(mut self, observer: T) -> Self 
    where 
        T: StageObserver<Vec<Statement>, Vec<Statement>> + 'static 
    {
        self.observer_registry.add_semantic_observer(observer);
        self
    }
}

/// Observer methods only available for chains that include code generation stage in their type-level list
impl<'a, Chain, Stages: HList> ChainPipeline<'a, Chain, Stages>
where
    Stages: HasStage<CodegenStageMarker>,
{
    /// Add a code generation observer to monitor code generation stage.
    /// Only available for chains that include code generation.
    pub fn with_codegen_observer<T>(mut self, observer: T) -> Self 
    where 
        T: StageObserver<Vec<Statement>, Chunk> + 'static 
    {
        self.observer_registry.add_codegen_observer(observer);
        self
    }
}

/// Execution methods for typed pipelines
impl<'a, Chain, Stages: HList> ChainPipeline<'a, Chain, Stages>
where
    Chain: ExecuteChain<String>,
{
    /// Execute the configured pipeline and return the result.
    pub fn execute(self) -> CompilationResult<'a, Chain::Output> {
        let mut context = StageContext::with_observer_registry(
            self.source.to_string(),
            self.file_name,
            self.observer_registry,
        );
        let mut diagnostics = DiagnosticEngine::new();

        match self.chain.execute_chain(self.source.to_string(), &mut context, &mut diagnostics) {
            Ok(output) => CompilationResult::Success {
                output,
                diagnostics,
            },
            Err(_) => CompilationResult::Failed {
                diagnostics,
            },
        }
    }
}

/// Convenience methods with specific return types for better API ergonomics
impl<'a> ChainPipeline<'a, TokenizationChain, HCons<TokenizationStageMarker, HNil>> {
    /// Execute tokenization and return tokens
    pub fn tokenize(self) -> CompilationResult<'a, Vec<Token>> {
        self.execute()
    }
}

impl<'a> ChainPipeline<'a, ParsingChain, HCons<ParsingStageMarker, HCons<TokenizationStageMarker, HNil>>> {
    /// Execute parsing and return AST statements
    pub fn parse(self) -> CompilationResult<'a, Vec<Statement>> {
        self.execute()
    }
}

impl<'a> ChainPipeline<'a, ASTChain, HCons<SemanticStageMarker, HCons<ParsingStageMarker, HCons<TokenizationStageMarker, HNil>>>> {
    /// Execute AST compilation and return analyzed AST
    pub fn compile_to_ast(self) -> CompilationResult<'a, Vec<Statement>> {
        self.execute()
    }
}

impl<'a> ChainPipeline<'a, FullCompilationChain, HCons<CodegenStageMarker, HCons<SemanticStageMarker, HCons<ParsingStageMarker, HCons<TokenizationStageMarker, HNil>>>>> {
    /// Execute full compilation and return bytecode
    pub fn compile_to_bytecode(self) -> CompilationResult<'a, Chunk> {
        self.execute()
    }

    /// Legacy-compatible method that mimics CompilationPipeline::execute_all_stages()
    #[deprecated(since = "0.2.0", note = "Use compile_to_bytecode() instead")]
    pub fn execute_all_stages(self) -> CompilationResult<'a, Chunk> {
        self.compile_to_bytecode()
    }
}
