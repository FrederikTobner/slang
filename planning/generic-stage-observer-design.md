# Generic Stage Observer Design Specification

## Problem Statement

The current `StageObserver` trait uses `std::any::Any` for input/output parameters, which:

1. **Type Safety Issues**: Requires runtime type checking with `downcast_ref`
2. **Maintenance Problems**: No compile-time guarantee that observers handle correct types
3. **Discoverability**: Unclear what types each observer expects
4. **Design Smell**: Indicates poor abstraction and loose coupling

## Current Architecture Analysis

### Data Flow Through Pipeline Stages

```rust
String -> TokenizationStage -> Vec<Token>
Vec<Token> -> ParsingStage -> Vec<Statement>  
Vec<Statement> -> SemanticAnalysisStage -> Vec<Statement>
Vec<Statement> -> CodeGenerationStage -> Chunk
```

### Current Observer Problems

```rust
// Current - Type unsafe, runtime errors possible
pub trait StageObserver: Send + Sync {
    fn on_stage_success(&self, stage_name: &str, output: &dyn Any);
}

// Usage requires unsafe downcasting
if let Some(tokens) = output.downcast_ref::<Vec<Token>>() {
    // Handle tokens - can fail at runtime!
}
```

## Proposed Solution: Generic Type Parameter Observers

### Design Decision: Generic vs Stage-Specific Traits

After analyzing both approaches, we recommend a **hybrid generic design** that combines the benefits of type parameters with user-friendly APIs.

#### Option 1: Stage-Specific Traits (Original Proposal)
```rust
pub trait TokenizationObserver: Send + Sync {
    fn on_tokenization_success(&self, tokens: &[Token]);
}
```

#### Option 2: Generic Type Parameters (Recommended)
```rust
pub trait StageObserver<Input, Output>: Send + Sync 
where Input: 'static, Output: 'static 
{
    fn on_stage_success(&self, output: &Output);
}

// User-friendly type aliases
pub type TokenizationObserver = dyn StageObserver<String, Vec<Token>>;
```

### Core Design Principles

1. **Type Safety First**: Compile-time type checking, no `Any` usage
2. **Stage-Specific**: Each observer is designed for specific pipeline stages  
3. **Composable**: Multiple observers can be combined for comprehensive monitoring
4. **Extensible**: Easy to add new observer types for new stages
5. **Zero-Cost Abstractions**: No runtime overhead from type erasure

### New Observer Architecture

#### Recommended: Generic Type Parameter Design

```rust
/// Core generic observer trait for maximum type safety
pub trait StageObserver<Input, Output>: Send + Sync 
where 
    Input: 'static, 
    Output: 'static 
{
    /// Called when a stage begins execution
    fn on_stage_start(&self, input: &Input) {}
    
    /// Called when a stage completes successfully
    fn on_stage_success(&self, output: &Output);
    
    /// Called when a stage fails
    fn on_stage_error(&self, error: &dyn Error) {}
}

/// User-friendly type aliases for better readability
pub type TokenizationObserver = dyn StageObserver<String, Vec<Token>>;
pub type ParsingObserver = dyn StageObserver<Vec<Token>, Vec<Statement>>;
pub type SemanticObserver = dyn StageObserver<Vec<Statement>, Vec<Statement>>;  
pub type CodegenObserver = dyn StageObserver<Vec<Statement>, Chunk>;
```

#### Multi-Stage Observer Support

```rust
/// Observer that can handle multiple pipeline stages
pub struct DebugObserver;

// Same observer can implement multiple generic variants
impl StageObserver<String, Vec<Token>> for DebugObserver {
    fn on_stage_success(&self, tokens: &Vec<Token>) {
        println!("Tokenization: {} tokens", tokens.len());
    }
}

impl StageObserver<Vec<Token>, Vec<Statement>> for DebugObserver {
    fn on_stage_success(&self, ast: &Vec<Statement>) {
        println!("Parsing: {} statements", ast.len());
    }
}
```

#### Observer Registration System

```rust
/// Type-safe observer registry using generics
pub struct ObserverRegistry {
    tokenization_observers: Vec<Box<dyn StageObserver<String, Vec<Token>>>>,
    parsing_observers: Vec<Box<dyn StageObserver<Vec<Token>, Vec<Statement>>>>,
    semantic_observers: Vec<Box<dyn StageObserver<Vec<Statement>, Vec<Statement>>>>,
    codegen_observers: Vec<Box<dyn StageObserver<Vec<Statement>, Chunk>>>,
}

impl ObserverRegistry {
    pub fn add_tokenization_observer<T>(&mut self, observer: T) 
    where T: StageObserver<String, Vec<Token>> + 'static 
    {
        self.tokenization_observers.push(Box::new(observer));
    }
    
    pub fn notify_tokenization_success(&self, tokens: &Vec<Token>) {
        for observer in &self.tokenization_observers {
            observer.on_stage_success(tokens);  // No downcasting needed!
        }
    }
    
    // Similar methods for other stages...
}
```

## Why Generic Type Parameters Are Superior


### 1. Maximum Type Safety

- **Zero Runtime Failures**: No `downcast_ref` calls that can fail
- **Compile-Time Guarantees**: Wrong types cause compilation errors  
- **Self-Documenting**: `StageObserver<String, Vec<Token>>` clearly shows expected types

### 2. DRY (Don't Repeat Yourself)

- **Single Trait Definition**: One generic trait instead of multiple specific ones
- **Consistent Interface**: Same method names across all stages
- **Less Maintenance**: Changes to observer interface only need one place

### 3. Multi-Stage Observer Support  

- **Code Reuse**: Same observer can handle multiple stages
- **Shared State**: Easy to track data across pipeline stages
- **Flexible Registration**: Register same observer for different stages

### 4. Better Performance

- **Zero-Cost Abstractions**: No runtime type checking overhead
- **Monomorphization**: Compiler generates optimized code for each type
- **No Virtual Dispatch**: Direct method calls where possible

### 5. Enhanced Developer Experience

- **Clear Error Messages**: Generic constraints provide better error messages
- **IDE Support**: Full auto-completion and type checking
- **Easy to Extend**: Adding new stages is straightforward

## Migration Strategy

### Phase 1: Introduce New Traits
- Add new observer traits alongside existing ones
- Mark old `StageObserver` as deprecated
- Update documentation

### Phase 2: Implement New Observers
- Convert existing observers to new system
- Create adapter for backward compatibility  
- Update pipeline builder

### Phase 3: Update Clients
- Update token-printer tool to use new observers
- Update any other tools using observers
- Remove deprecated traits

### Phase 4: Cleanup
- Remove `Any` usage completely
- Remove old observer implementation
- Update examples and documentation

## Example Usage

### Token Printer Implementation
```rust
pub struct TokenPrinter {
    formatter: Box<dyn TokenFormatter>,
    file_name: String,
}

impl TokenizationObserver for TokenPrinter {
    fn on_tokenization_success(&self, tokens: &[Token]) {
        self.formatter.format_tokens(tokens, &self.file_name);
    }
    
    fn on_tokenization_start(&self, _source: &str) {}
    fn on_tokenization_error(&self, _error: &dyn Error) {}
}

// Register with pipeline
let pipeline = PipelineBuilder::new(source)
    .add_tokenization_observer(TokenPrinter::new(formatter, file_name))
    .build();
```

### Multi-Stage Debug Observer
```rust  
pub struct DebugObserver;

impl PipelineObserver for DebugObserver {
    fn tokenization_observer(&self) -> Option<&dyn TokenizationObserver> {
        Some(self)
    }
    
    fn parsing_observer(&self) -> Option<&dyn ParsingObserver> {
        Some(self)  
    }
}

impl TokenizationObserver for DebugObserver {
    fn on_tokenization_success(&self, tokens: &[Token]) {
        println!("Tokenization produced {} tokens", tokens.len());
    }
}

impl ParsingObserver for DebugObserver {
    fn on_parsing_success(&self, ast: &[Statement]) {
        println!("Parsing produced {} statements", ast.len());
    }
}
```

## Conclusion

This design eliminates the problematic `Any` usage while providing:

- **Type Safety**: Compile-time guarantees about observer interfaces
- **Flexibility**: Observers can target specific stages or handle multiple stages  
- **Performance**: Zero-cost abstractions with no runtime type checking
- **Maintainability**: Clear, documented interfaces that are easy to extend

The migration can be done incrementally, ensuring backward compatibility during the transition period.
