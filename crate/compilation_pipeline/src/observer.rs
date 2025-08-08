//! Generic type-safe observer system for pipeline stages
//!
//! This module provides a generic observer trait that eliminates the need for `std::any::Any`
//! while maintaining type safety and flexibility. The hybrid approach combines the benefits
//! of generic type parameters with user-friendly type aliases.

use std::error::Error;
use slang_frontend::Token;
use slang_ir::ast::Statement;
use slang_backend::bytecode::Chunk;
use crate::source_file::SlangSourceFile;

/// Generic observer trait for pipeline stages with compile-time type safety
///
/// This trait uses generic type parameters to ensure observers can only be used
/// with the correct input and output types, eliminating runtime type checking
/// and providing better performance and safety.
///
/// # Type Parameters
/// * `Input` - The input type for the stage (e.g., `String` for tokenization)
/// * `Output` - The output type for the stage (e.g., `Vec<Token>` for tokenization)
///
/// # Examples
/// ```rust
/// use slang_compilation_pipeline::observer::StageObserver;
/// use slang_frontend::Token;
/// use slang_compilation_pipeline::SlangSourceFile;
/// 
/// struct TokenCounter;
/// 
/// impl StageObserver<SlangSourceFile, Vec<Token>> for TokenCounter {
///     fn on_stage_success(&self, output: &Vec<Token>) {
///         println!("Tokenized {} tokens", output.len());
///     }
/// }
/// ```
pub trait StageObserver<Input, Output>: Send + Sync 
where 
    Input: 'static,
    Output: 'static,
{
    /// Called when a stage begins execution
    /// 
    /// Default implementation does nothing, allowing observers to focus only
    /// on the events they care about.
    fn on_stage_start(&self, _input: &Input) {}
    
    /// Called when a stage completes successfully
    /// 
    /// This is the primary method observers should implement to handle
    /// successful stage completion.
    fn on_stage_success(&self, output: &Output);
    
    /// Called when a stage fails with an error
    /// 
    /// Default implementation does nothing, but observers can override
    /// to handle error cases if needed.
    fn on_stage_error(&self, _error: &dyn Error) {}
}

/// User-friendly type aliases for better readability and discoverability
/// 
/// These aliases hide the generic complexity while providing clear, descriptive names
/// that make the API easier to understand and use.
pub type TokenizationObserver = dyn StageObserver<SlangSourceFile, Vec<Token>>;
pub type ParsingObserver = dyn StageObserver<Vec<Token>, Vec<Statement>>;
pub type SemanticObserver = dyn StageObserver<Vec<Statement>, Vec<Statement>>;
pub type CodegenObserver = dyn StageObserver<Vec<Statement>, Chunk>;

/// Registry for managing type-safe observers across all pipeline stages
///
/// The registry maintains separate collections for each stage type, ensuring
/// compile-time type safety while providing a clean API for registration
/// and notification.
pub struct ObserverRegistry {
    tokenization_observers: Vec<Box<TokenizationObserver>>,
    parsing_observers: Vec<Box<ParsingObserver>>,
    semantic_observers: Vec<Box<SemanticObserver>>,
    codegen_observers: Vec<Box<CodegenObserver>>,
}

impl ObserverRegistry {
    /// Create a new empty observer registry
    pub fn new() -> Self {
        Self {
            tokenization_observers: Vec::new(),
            parsing_observers: Vec::new(),
            semantic_observers: Vec::new(),
            codegen_observers: Vec::new(),
        }
    }
    
    /// Add a tokenization observer
    /// 
    /// The generic constraint ensures only observers that handle the correct
    /// types can be registered, providing compile-time safety.
    pub fn add_tokenization_observer<T>(&mut self, observer: T) 
    where 
        T: StageObserver<SlangSourceFile, Vec<Token>> + 'static 
    {
        self.tokenization_observers.push(Box::new(observer));
    }
    
    /// Add a parsing observer
    pub fn add_parsing_observer<T>(&mut self, observer: T) 
    where 
        T: StageObserver<Vec<Token>, Vec<Statement>> + 'static 
    {
        self.parsing_observers.push(Box::new(observer));
    }
    
    /// Add a semantic analysis observer
    pub fn add_semantic_observer<T>(&mut self, observer: T) 
    where 
        T: StageObserver<Vec<Statement>, Vec<Statement>> + 'static 
    {
        self.semantic_observers.push(Box::new(observer));
    }
    
    /// Add a code generation observer
    pub fn add_codegen_observer<T>(&mut self, observer: T) 
    where 
        T: StageObserver<Vec<Statement>, Chunk> + 'static 
    {
        self.codegen_observers.push(Box::new(observer));
    }
    
    /// Notify tokenization observers of stage start
    pub fn notify_tokenization_start(&self, input: &SlangSourceFile) {
        for observer in &self.tokenization_observers {
            observer.on_stage_start(input);
        }
    }
    
    /// Notify tokenization observers of successful completion
    pub fn notify_tokenization_success(&self, tokens: &Vec<Token>) {
        for observer in &self.tokenization_observers {
            observer.on_stage_success(tokens);
        }
    }
    
    /// Notify tokenization observers of errors
    pub fn notify_tokenization_error(&self, error: &dyn Error) {
        for observer in &self.tokenization_observers {
            observer.on_stage_error(error);
        }
    }
    
    /// Notify parsing observers of stage start
    pub fn notify_parsing_start(&self, input: &Vec<Token>) {
        for observer in &self.parsing_observers {
            observer.on_stage_start(input);
        }
    }
    
    /// Notify parsing observers of successful completion
    pub fn notify_parsing_success(&self, ast: &Vec<Statement>) {
        for observer in &self.parsing_observers {
            observer.on_stage_success(ast);
        }
    }
    
    /// Notify parsing observers of errors
    pub fn notify_parsing_error(&self, error: &dyn Error) {
        for observer in &self.parsing_observers {
            observer.on_stage_error(error);
        }
    }
    
    /// Notify semantic analysis observers of stage start
    pub fn notify_semantic_start(&self, input: &Vec<Statement>) {
        for observer in &self.semantic_observers {
            observer.on_stage_start(input);
        }
    }
    
    /// Notify semantic analysis observers of successful completion
    pub fn notify_semantic_success(&self, ast: &Vec<Statement>) {
        for observer in &self.semantic_observers {
            observer.on_stage_success(ast);
        }
    }
    
    /// Notify semantic analysis observers of errors
    pub fn notify_semantic_error(&self, error: &dyn Error) {
        for observer in &self.semantic_observers {
            observer.on_stage_error(error);
        }
    }
    
    /// Notify code generation observers of stage start
    pub fn notify_codegen_start(&self, input: &Vec<Statement>) {
        for observer in &self.codegen_observers {
            observer.on_stage_start(input);
        }
    }
    
    /// Notify code generation observers of successful completion
    pub fn notify_codegen_success(&self, chunk: &Chunk) {
        for observer in &self.codegen_observers {
            observer.on_stage_success(chunk);
        }
    }
    
    /// Notify code generation observers of errors
    pub fn notify_codegen_error(&self, error: &dyn Error) {
        for observer in &self.codegen_observers {
            observer.on_stage_error(error);
        }
    }
}

impl Default for ObserverRegistry {
    fn default() -> Self {
        Self::new()
    }
}
