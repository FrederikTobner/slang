//! Execution Chain Pattern for Type-Safe Pipeline Construction
//!
//! This module provides a clean API for constructing compilation pipelines
//! using a chain pattern that maintains logical execution order and type safety.

use crate::pipeline::{
    hlist::{HCons, HList, HNil},
    stage::{CompilationStage, StageContext},
};
use slang_shared::DiagnosticEngine;
use slang_ir::ast::Statement; 
use slang_frontend::Token;
use slang_backend::bytecode::Chunk;

/// A type-safe execution chain that maintains stages in their logical execution order.
/// 
/// The ExecutionChain pattern allows users to construct pipelines by chaining stages
/// in the order they should execute, which is more intuitive than the HList approach.
/// Internally, it uses HList for type safety and zero-cost execution.
pub struct ExecutionChain<Input, Output, Stages: HList> {
    stages: Stages,
    _phantom: std::marker::PhantomData<(Input, Output)>,
}

impl ExecutionChain<String, String, HNil> {
    /// Create a new empty execution chain that starts with String input.
    pub fn new() -> Self {
        Self {
            stages: HNil,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Input, Output, Stages> ExecutionChain<Input, Output, Stages>
where
    Stages: HList,
{
    /// Add a stage to the end of the execution chain.
    /// 
    /// This maintains the logical execution order - stages execute in the order
    /// they are added to the chain.
    pub fn then<S>(self, stage: S) -> ExecutionChain<Input, S::Output, HCons<S, Stages>>
    where
        S: CompilationStage<Input = Output> + 'static,
    {
        ExecutionChain {
            stages: HCons::new(stage, self.stages),
            _phantom: std::marker::PhantomData,
        }
    }
}

/// Trait for executing an execution chain.
/// 
/// This trait is implemented for ExecutionChain types that can be executed
/// with a given input type.
pub trait ExecuteChain<Input> {
    type Output;
    
    fn execute_chain(
        self,
        input: Input,
        context: &mut StageContext,
        diagnostics: &mut DiagnosticEngine,
    ) -> Result<Self::Output, ()>;
}

impl<Input, Output> ExecuteChain<Input> for ExecutionChain<Input, Output, HNil> {
    type Output = Input;
    
    fn execute_chain(
        self,
        input: Input,
        _context: &mut StageContext,
        _diagnostics: &mut DiagnosticEngine,
    ) -> Result<Self::Output, ()> {
        // Empty chain just returns the input unchanged
        Ok(input)
    }
}

// We need a trait to handle tail-first execution recursively
pub trait ExecuteTailFirst<Input> {
    type Output;
    
    fn execute_tail_first(
        &self,
        input: Input,
        context: &mut StageContext,
        diagnostics: &mut DiagnosticEngine,
    ) -> Result<Self::Output, ()>;
}

impl<Input> ExecuteTailFirst<Input> for HNil {
    type Output = Input;
    
    fn execute_tail_first(
        &self,
        input: Input,
        _context: &mut StageContext,
        _diagnostics: &mut DiagnosticEngine,
    ) -> Result<Self::Output, ()> {
        Ok(input)
    }
}

impl<Input, H, T> ExecuteTailFirst<Input> for HCons<H, T>
where
    H: CompilationStage + 'static,
    T: HList + ExecuteTailFirst<Input>,
    T::Output: 'static,
    H::Input: From<T::Output> + 'static, // H must accept the tail's output
    H::Output: 'static,
{
    type Output = H::Output;
    
    fn execute_tail_first(
        &self,
        input: Input,
        context: &mut StageContext,
        diagnostics: &mut DiagnosticEngine,
    ) -> Result<Self::Output, ()> {
        // Execute tail first (recursive)
        let intermediate = self.tail.execute_tail_first(input, context, diagnostics)?;
        
        // Then execute head with tail's output
        self.head.execute(H::Input::from(intermediate), context, diagnostics)
    }
}

impl<Input, Output, H, T> ExecuteChain<Input> for ExecutionChain<Input, Output, HCons<H, T>>
where
    H: CompilationStage + 'static,
    T: HList + 'static,
    HCons<H, T>: ExecuteTailFirst<Input, Output = Output> + 'static,
{
    type Output = Output;
    
    fn execute_chain(
        self,
        input: Input,
        context: &mut StageContext,
        diagnostics: &mut DiagnosticEngine,
    ) -> Result<Self::Output, ()> {
        // Use tail-first execution
        self.stages.execute_tail_first(input, context, diagnostics)
    }
}

// Convenience type aliases for common execution chains
use crate::pipeline::stages::{
    TokenizationStage, ParsingStage, SemanticAnalysisStage, CodeGenerationStage
};

/// Tokenization-only chain: String → Vec<Token>
pub type TokenizationChain = ExecutionChain<
    String,
    Vec<Token>,
    HCons<TokenizationStage, HNil>
>;

/// Parsing chain: String → Vec<Statement>
pub type ParsingChain = ExecutionChain<
    String,
    Vec<Statement>,
    HCons<ParsingStage, HCons<TokenizationStage, HNil>>
>;

/// AST compilation chain: String → Vec<Statement> (with semantic analysis)
pub type ASTChain = ExecutionChain<
    String,
    Vec<Statement>,
    HCons<SemanticAnalysisStage, HCons<ParsingStage, HCons<TokenizationStage, HNil>>>
>;

/// Full compilation chain: String → Chunk
pub type FullCompilationChain = ExecutionChain<
    String,
    Chunk,
    HCons<CodeGenerationStage, HCons<SemanticAnalysisStage, HCons<ParsingStage, HCons<TokenizationStage, HNil>>>>
>;

/// Parsing-only chain starting from tokens: Vec<Token> → Vec<Statement>
pub type TokenParsingChain = ExecutionChain<
    Vec<Token>,
    Vec<Statement>,
    HCons<ParsingStage, HNil>
>;

/// Semantic analysis-only chain: Vec<Statement> → Vec<Statement>
pub type SemanticChain = ExecutionChain<
    Vec<Statement>,
    Vec<Statement>,
    HCons<SemanticAnalysisStage, HNil>
>;

/// Code generation-only chain: Vec<Statement> → Chunk
pub type CodegenChain = ExecutionChain<
    Vec<Statement>,
    Chunk,
    HCons<CodeGenerationStage, HNil>
>;

/// Semantic + Codegen chain: Vec<Statement> → Chunk
pub type SemanticCodegenChain = ExecutionChain<
    Vec<Statement>,
    Chunk,
    HCons<CodeGenerationStage, HCons<SemanticAnalysisStage, HNil>>
>;

/// Token to AST chain: Vec<Token> → Vec<Statement> (parsing + semantic)
pub type TokenASTChain = ExecutionChain<
    Vec<Token>,
    Vec<Statement>,
    HCons<SemanticAnalysisStage, HCons<ParsingStage, HNil>>
>;

/// Token to Bytecode chain: Vec<Token> → Chunk (parsing + semantic + codegen)
pub type TokenBytecodeChain = ExecutionChain<
    Vec<Token>,
    Chunk,
    HCons<CodeGenerationStage, HCons<SemanticAnalysisStage, HCons<ParsingStage, HNil>>>
>;

impl TokenizationChain {
    /// Create a tokenization-only chain.
    pub fn tokenization() -> Self {
        ExecutionChain::new().then(TokenizationStage)
    }
}

impl ParsingChain {
    /// Create a parsing chain (tokenization + parsing).
    /// With tail-first execution, we can add stages in logical order!
    pub fn parsing() -> Self {
        ExecutionChain::new()
            .then(TokenizationStage)   // Execute first (tail)
            .then(ParsingStage)        // Execute second (head)
    }
}

impl ASTChain {
    /// Create an AST compilation chain (tokenization + parsing + semantic analysis).
    /// With tail-first execution, we can add stages in logical order!
    pub fn ast_compilation() -> Self {
        ExecutionChain::new()
            .then(TokenizationStage)      // Execute first (tail)
            .then(ParsingStage)           // Execute second (middle)
            .then(SemanticAnalysisStage)  // Execute third (head)
    }
}

impl FullCompilationChain {
    /// Create a full compilation chain (all stages).
    /// With tail-first execution, we can add stages in logical order!
    pub fn full_compilation() -> Self {
        ExecutionChain::new()
            .then(TokenizationStage)      // Execute first (tail)
            .then(ParsingStage)           // Execute second (middle¹)
            .then(SemanticAnalysisStage)  // Execute third (middle²)
            .then(CodeGenerationStage)    // Execute fourth (head)
    }
}

impl TokenParsingChain {
    /// Create a parsing-only chain that starts from tokens.
    pub fn token_parsing() -> Self {
        ExecutionChain {
            stages: HCons::new(ParsingStage, HNil),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl SemanticChain {
    /// Create a semantic analysis-only chain.
    pub fn semantic_only() -> Self {
        ExecutionChain {
            stages: HCons::new(SemanticAnalysisStage, HNil),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl CodegenChain {
    /// Create a code generation-only chain.
    pub fn codegen_only() -> Self {
        ExecutionChain {
            stages: HCons::new(CodeGenerationStage, HNil),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl SemanticCodegenChain {
    /// Create a semantic analysis + code generation chain.
    pub fn semantic_codegen() -> Self {
        ExecutionChain {
            stages: HCons::new(CodeGenerationStage, 
                       HCons::new(SemanticAnalysisStage, HNil)),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl TokenASTChain {
    /// Create a token to AST chain (parsing + semantic analysis).
    pub fn token_ast() -> Self {
        ExecutionChain {
            stages: HCons::new(SemanticAnalysisStage,
                       HCons::new(ParsingStage, HNil)),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl TokenBytecodeChain {
    /// Create a token to bytecode chain (parsing + semantic + codegen).
    pub fn token_bytecode() -> Self {
        ExecutionChain {
            stages: HCons::new(CodeGenerationStage,
                       HCons::new(SemanticAnalysisStage,
                          HCons::new(ParsingStage, HNil))),
            _phantom: std::marker::PhantomData,
        }
    }
}


