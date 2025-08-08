//! Execution Chain Pattern for Type-Safe Pipeline Construction
//!
//! This module provides a clean API for constructing compilation pipelines
//! using a chain pattern that maintains logical execution order and type safety.

use crate::{
    error::StageError,
    hlist::{HCons, HList, HNil},
    stage::{CompilationStage, StageContext},
};
use slang_shared::DiagnosticEngine;
use crate::source_file::SlangSourceFile;
use slang_ir::ast::Statement; 
use slang_frontend::Token;
use slang_backend::bytecode::Chunk;
use crate::stages::{
    TokenizationStage, ParsingStage, SemanticAnalysisStage, CodeGenerationStage
};

// Import the macros from the chain_macros module
use crate::{define_chain_types, define_chain_constructors};


/// A type-safe execution chain that maintains stages in their logical execution order.
/// 
/// The ExecutionChain pattern allows users to construct pipelines by chaining stages
/// in the order they should execute, which is more intuitive than the HList approach.
/// Internally, it uses HList for type safety and zero-cost execution.
pub struct ExecutionChain<Input, Output, Stages: HList> {
    stages: Stages,
    _phantom: std::marker::PhantomData<(Input, Output)>,
}


impl ExecutionChain<(), (), HNil> {
    /// Create a new execution chain with the first stage.
    pub fn starting_with<S>(stage: S) -> ExecutionChain<S::Input, S::Output, HCons<S, HNil>>
    where
        S: CompilationStage + 'static,
    {
        ExecutionChain {
            stages: HCons::new(stage, HNil),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Input, Output, Stages> ExecutionChain<Input, Output, Stages>
where
    Stages: HList,
{
    /// Add a stage to the end of the execution chain.
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
    ) -> Result<Self::Output, StageError>;
}

impl<Input, Output> ExecuteChain<Input> for ExecutionChain<Input, Output, HNil> {
    type Output = Input;
    
    fn execute_chain(
        self,
        input: Input,
        _context: &mut StageContext,
        _diagnostics: &mut DiagnosticEngine,
    ) -> Result<Self::Output, StageError> {
        Ok(input)
    }
}

pub trait ExecuteTailFirst<Input> {
    type Output;
    
    fn execute_tail_first(
        &self,
        input: Input,
        context: &mut StageContext,
        diagnostics: &mut DiagnosticEngine,
    ) -> Result<Self::Output, StageError>;
}

impl<Input> ExecuteTailFirst<Input> for HNil {
    type Output = Input;
    
    fn execute_tail_first(
        &self,
        input: Input,
        _context: &mut StageContext,
        _diagnostics: &mut DiagnosticEngine,
    ) -> Result<Self::Output, StageError> {
        Ok(input)
    }
}

impl<Input, H, T> ExecuteTailFirst<Input> for HCons<H, T>
where
    H: CompilationStage + 'static,
    T: HList + ExecuteTailFirst<Input>,
    T::Output: 'static,
    H::Input: From<T::Output> + 'static,
    H::Output: 'static,
{
    type Output = H::Output;
    
    fn execute_tail_first(
        &self,
        input: Input,
        context: &mut StageContext,
        diagnostics: &mut DiagnosticEngine,
    ) -> Result<Self::Output, StageError> {
        let intermediate = self.tail.execute_tail_first(input, context, diagnostics)?;
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
    ) -> Result<Self::Output, StageError> {
        self.stages.execute_tail_first(input, context, diagnostics)
    }
}

// Define all chain types using the macro to eliminate repetitive boilerplate
define_chain_types! {
    /// Tokenization-only chain: SlangSourceFile → Vec<Token>
    TokenizationChain(SlangSourceFile, Vec<Token>): [TokenizationStage];

    /// Parsing chain: SlangSourceFile → Vec<Statement>
    ParsingChain(SlangSourceFile, Vec<Statement>): [ParsingStage, TokenizationStage];

    /// AST compilation chain: SlangSourceFile → Vec<Statement> (with semantic analysis)
    ASTChain(SlangSourceFile, Vec<Statement>): [SemanticAnalysisStage, ParsingStage, TokenizationStage];

    /// Full compilation chain: SlangSourceFile → Chunk
    FullCompilationChain(SlangSourceFile, Chunk): [CodeGenerationStage, SemanticAnalysisStage, ParsingStage, TokenizationStage];

    /// Parsing-only chain starting from tokens: Vec<Token> → Vec<Statement>
    TokenParsingChain(Vec<Token>, Vec<Statement>): [ParsingStage];

    /// Semantic analysis-only chain: Vec<Statement> → Vec<Statement>
    SemanticChain(Vec<Statement>, Vec<Statement>): [SemanticAnalysisStage];

    /// Code generation-only chain: Vec<Statement> → Chunk
    CodegenChain(Vec<Statement>, Chunk): [CodeGenerationStage];

    /// Semantic + Codegen chain: Vec<Statement> → Chunk
    SemanticCodegenChain(Vec<Statement>, Chunk): [CodeGenerationStage, SemanticAnalysisStage];

    /// Token to AST chain: Vec<Token> → Vec<Statement> (parsing + semantic)
    TokenASTChain(Vec<Token>, Vec<Statement>): [SemanticAnalysisStage, ParsingStage];

    /// Token to Bytecode chain: Vec<Token> → Chunk (parsing + semantic + codegen)
    TokenBytecodeChain(Vec<Token>, Chunk): [CodeGenerationStage, SemanticAnalysisStage, ParsingStage];
}

// Define convenience constructors for all chain types  
define_chain_constructors! {
    TokenizationChain => tokenization: [TokenizationStage];
    ParsingChain => parsing: [TokenizationStage, ParsingStage];
    ASTChain => ast_compilation: [TokenizationStage, ParsingStage, SemanticAnalysisStage];
    FullCompilationChain => full_compilation: [TokenizationStage, ParsingStage, SemanticAnalysisStage, CodeGenerationStage];
    TokenParsingChain => token_parsing: [ParsingStage];
    SemanticChain => semantic_only: [SemanticAnalysisStage];
    CodegenChain => codegen_only: [CodeGenerationStage];
    SemanticCodegenChain => semantic_codegen: [SemanticAnalysisStage, CodeGenerationStage];
    TokenASTChain => token_ast: [ParsingStage, SemanticAnalysisStage];
    TokenBytecodeChain => token_bytecode: [ParsingStage, SemanticAnalysisStage, CodeGenerationStage];
}
