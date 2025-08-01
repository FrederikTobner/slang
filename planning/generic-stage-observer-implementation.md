# Generic Stage Observer Implementation Plan

## Overview

This document outlines the step-by-step implementation plan for replacing the current `Any`-based `StageObserver` with a type-safe, generic observer system.

## Implementation Phases

### Phase 1: Foundation - New Observer Traits

**Duration**: 1-2 days
**Risk**: Low
**Dependencies**: None

#### 1.1 Create New Observer Trait Module

**File**: `crate/compilation_pipeline/src/pipeline/observers/traits.rs`

```rust
//! Stage-specific observer traits for type-safe pipeline monitoring

use slang_frontend::Token;
use slang_ir::ast::Statement;
use slang_backend::bytecode::Chunk;
use std::error::Error;
use crate::pipeline::result::CompilationResult;

/// Observer for tokenization stage
pub trait TokenizationObserver: Send + Sync {
    fn on_tokenization_start(&self, source: &str) {}
    fn on_tokenization_success(&self, tokens: &[Token]);
    fn on_tokenization_error(&self, error: &dyn Error) {}
}

/// Observer for parsing stage
pub trait ParsingObserver: Send + Sync {
    fn on_parsing_start(&self, tokens: &[Token]) {}
    fn on_parsing_success(&self, ast: &[Statement]);
    fn on_parsing_error(&self, error: &dyn Error) {}
}

/// Observer for semantic analysis stage
pub trait SemanticObserver: Send + Sync {
    fn on_semantic_start(&self, ast: &[Statement]) {}
    fn on_semantic_success(&self, validated_ast: &[Statement]);
    fn on_semantic_error(&self, error: &dyn Error) {}
}

/// Observer for code generation stage
pub trait CodegenObserver: Send + Sync {
    fn on_codegen_start(&self, ast: &[Statement]) {}
    fn on_codegen_success(&self, chunk: &Chunk);
    fn on_codegen_error(&self, error: &dyn Error) {}
}

/// Composite observer for entire pipeline
pub trait PipelineObserver: Send + Sync {
    fn tokenization_observer(&self) -> Option<&dyn TokenizationObserver> { None }
    fn parsing_observer(&self) -> Option<&dyn ParsingObserver> { None }
    fn semantic_observer(&self) -> Option<&dyn SemanticObserver> { None }
    fn codegen_observer(&self) -> Option<&dyn CodegenObserver> { None }
    fn on_pipeline_complete(&self, result: &CompilationResult) {}
}
```

#### 1.2 Create Observer Registry

**File**: `crate/compilation_pipeline/src/pipeline/observers/registry.rs`

```rust
//! Observer registry for managing pipeline stage observers

use super::traits::*;
use crate::pipeline::result::CompilationResult;

/// Registry for managing observers across all pipeline stages
pub struct ObserverRegistry {
    tokenization_observers: Vec<Box<dyn TokenizationObserver>>,
    parsing_observers: Vec<Box<dyn ParsingObserver>>,
    semantic_observers: Vec<Box<dyn SemanticObserver>>,
    codegen_observers: Vec<Box<dyn CodegenObserver>>,
    pipeline_observers: Vec<Box<dyn PipelineObserver>>,
}

impl ObserverRegistry {
    pub fn new() -> Self {
        Self {
            tokenization_observers: Vec::new(),
            parsing_observers: Vec::new(),
            semantic_observers: Vec::new(),
            codegen_observers: Vec::new(),
            pipeline_observers: Vec::new(),
        }
    }
    
    // Observer registration methods
    pub fn add_tokenization_observer<T: TokenizationObserver + 'static>(mut self, observer: T) -> Self {
        self.tokenization_observers.push(Box::new(observer));
        self
    }
    
    pub fn add_parsing_observer<T: ParsingObserver + 'static>(mut self, observer: T) -> Self {
        self.parsing_observers.push(Box::new(observer));
        self
    }
    
    pub fn add_semantic_observer<T: SemanticObserver + 'static>(mut self, observer: T) -> Self {
        self.semantic_observers.push(Box::new(observer));
        self
    }
    
    pub fn add_codegen_observer<T: CodegenObserver + 'static>(mut self, observer: T) -> Self {
        self.codegen_observers.push(Box::new(observer));
        self
    }
    
    pub fn add_pipeline_observer<T: PipelineObserver + 'static>(mut self, observer: T) -> Self {
        self.pipeline_observers.push(Box::new(observer));
        self
    }
    
    // Notification methods (called by pipeline stages)
    pub fn notify_tokenization_start(&self, source: &str) {
        for observer in &self.tokenization_observers {
            observer.on_tokenization_start(source);
        }
        for observer in &self.pipeline_observers {
            if let Some(tokenization_observer) = observer.tokenization_observer() {
                tokenization_observer.on_tokenization_start(source);
            }
        }
    }
    
    pub fn notify_tokenization_success(&self, tokens: &[Token]) {
        for observer in &self.tokenization_observers {
            observer.on_tokenization_success(tokens);
        }
        for observer in &self.pipeline_observers {
            if let Some(tokenization_observer) = observer.tokenization_observer() {
                tokenization_observer.on_tokenization_success(tokens);
            }
        }
    }
    
    // Similar methods for other stages...
    
    pub fn notify_pipeline_complete(&self, result: &CompilationResult) {
        for observer in &self.pipeline_observers {
            observer.on_pipeline_complete(result);
        }
    }
}
```

#### 1.3 Update Module Exports

**File**: `crate/compilation_pipeline/src/pipeline/observers/mod.rs`

```rust
//! Observer system for monitoring compilation pipeline stages

pub mod traits;
pub mod registry;
pub mod debug;

// Re-export the new observer system
pub use traits::*;
pub use registry::ObserverRegistry;

// Keep existing observers for backward compatibility
pub use debug::{ASTPrintObserver, BytecodePrintObserver};
```

#### 1.4 Update Pipeline Builder

Add observer registry support to `PipelineBuilder`:

```rust
impl<'a> PipelineBuilder<'a> {
    pub fn add_tokenization_observer<T: TokenizationObserver + 'static>(mut self, observer: T) -> Self {
        self.observer_registry = self.observer_registry.add_tokenization_observer(observer);
        self
    }
    
    // Similar methods for other observer types...
}
```

### Phase 2: Pipeline Integration

**Duration**: 2-3 days
**Risk**: Medium
**Dependencies**: Phase 1 complete

#### 2.1 Update Pipeline Stages

Modify each stage to call the appropriate observer methods:

**Tokenization Stage**:
```rust
impl CompilationStage for TokenizationStage {
    fn execute(&self, input: Self::Input, context: &mut StageContext, diagnostics: &mut DiagnosticEngine) -> Result<Self::Output, ()> {
        // Notify observers about start
        context.observer_registry.notify_tokenization_start(&input);
        
        let lexer = Lexer::new(&input);
        match lexer.tokenize() {
            Ok(result) => {
                // Notify observers about success
                context.observer_registry.notify_tokenization_success(&result.tokens);
                Ok(result.tokens)
            },
            Err(errors) => {
                for error in errors {
                    // Notify observers about error
                    context.observer_registry.notify_tokenization_error(&error);
                    diagnostics.emit_compiler_error(error);
                }
                Err(())
            }
        }
    }
}
```

#### 2.2 Update StageContext

Add observer registry to the stage context:

```rust
pub struct StageContext {
    pub source: String,
    pub file_name: Option<String>,
    pub compilation_context: CompilationContext,
    pub observer_registry: ObserverRegistry,  // New field
}
```

#### 2.3 Create Backward Compatibility Adapter

Create an adapter that allows old `StageObserver` implementations to work with the new system:

```rust
/// Adapter to make old StageObserver work with new system
pub struct LegacyObserverAdapter {
    inner: Box<dyn StageObserver>,
}

impl PipelineObserver for LegacyObserverAdapter {
    fn tokenization_observer(&self) -> Option<&dyn TokenizationObserver> {
        Some(self)
    }
    // Implement for other stages...
}

impl TokenizationObserver for LegacyObserverAdapter {
    fn on_tokenization_success(&self, tokens: &[Token]) {
        self.inner.on_stage_success("Tokenization", tokens as &dyn Any);
    }
}
```

### Phase 3: Tool Updates

**Duration**: 1-2 days
**Risk**: Low
**Dependencies**: Phase 2 complete

#### 3.1 Update Token-Printer Observer

**File**: `tools/token-printer/src/observer.rs`

```rust
use slang_compilation_pipeline::pipeline::observers::traits::TokenizationObserver;
use slang_frontend::Token;
use crate::formatter::TokenFormatter;
use std::error::Error;

/// Type-safe token printer observer using the new observer system
pub struct TokenPrinterObserver {
    formatter: Box<dyn TokenFormatter>,
    file_name: String,
}

impl TokenPrinterObserver {
    pub fn new(formatter: Box<dyn TokenFormatter>, file_name: String) -> Self {
        Self { formatter, file_name }
    }
}

impl TokenizationObserver for TokenPrinterObserver {
    fn on_tokenization_success(&self, tokens: &[Token]) {
        self.formatter.format_tokens(tokens, &self.file_name);
    }
}
```

#### 3.2 Update CLI Integration

**File**: `tools/token-printer/src/cli.rs`

```rust
pub fn tokenize_file(file_path: &str, format: TokenFormat) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;
    let formatter = format.create_formatter();
    let observer = TokenPrinterObserver::new(formatter, file_path.to_string());
    
    let pipeline = PipelineBuilder::new(&source)
        .add_stage(TokenizationStage)
        .add_tokenization_observer(observer)  // New type-safe method
        .build();
    
    pipeline.execute();
    Ok(())
}
```

### Phase 4: Testing & Validation

**Duration**: 1-2 days
**Risk**: Low
**Dependencies**: Phase 3 complete

#### 4.1 Create Integration Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestTokenizationObserver {
        tokens_received: Arc<Mutex<Vec<Token>>>,
    }
    
    impl TokenizationObserver for TestTokenizationObserver {
        fn on_tokenization_success(&self, tokens: &[Token]) {
            *self.tokens_received.lock().unwrap() = tokens.to_vec();
        }
    }
    
    #[test]
    fn test_type_safe_observer() {
        let observer = TestTokenizationObserver::new();
        let tokens_received = observer.tokens_received.clone();
        
        let pipeline = PipelineBuilder::new("fn test() {}")
            .add_stage(TokenizationStage)
            .add_tokenization_observer(observer)
            .build();
            
        pipeline.execute();
        
        let tokens = tokens_received.lock().unwrap();
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].token_type, TokenType::Fn);
    }
}
```

#### 4.2 Performance Benchmarks

Create benchmarks to ensure the new system doesn't introduce performance regressions:

```rust
#[divan::bench]
fn bench_new_observer_system(bencher: Bencher) {
    let source = "fn fibonacci(n: int) -> int { if n <= 1 { return n; } return fibonacci(n - 1) + fibonacci(n - 2); }";
    
    bencher.bench(|| {
        let pipeline = PipelineBuilder::new(source)
            .add_tokenization_observer(NoOpTokenizationObserver)
            .add_parsing_observer(NoOpParsingObserver)
            .build();
        pipeline.execute();
    });
}
```

### Phase 5: Cleanup & Documentation

**Duration**: 1 day
**Risk**: Low
**Dependencies**: Phase 4 complete

#### 5.1 Remove Deprecated Code

- Remove old `StageObserver` trait
- Remove `Any` usage from pipeline
- Remove backward compatibility adapter
- Update all documentation

#### 5.2 Update Documentation

- Update README with new observer examples
- Add migration guide for existing observers
- Update inline documentation

#### 5.3 Final Validation

- Run full test suite
- Check all compiler warnings are resolved
- Validate no `Any` usage remains in pipeline code

## Risk Mitigation

### Breaking Changes

**Risk**: Changes to observer interface break existing code
**Mitigation**: Provide backward compatibility adapter during transition

### Performance Regression

**Risk**: New observer system is slower than current implementation
**Mitigation**: Benchmark early and optimize hot paths

### Complex Migration

**Risk**: Migration is too complex for users
**Mitigation**: Provide clear migration guide and automated tools where possible

## Success Criteria

- [ ] No `std::any::Any` usage in observer system
- [ ] All observer interactions are compile-time type-safe
- [ ] Token-printer tool works with new observer system
- [ ] No performance regression (< 5% slowdown acceptable)
- [ ] All existing tests pass
- [ ] Documentation is complete and accurate

## Timeline

**Total Duration**: 7-10 days

- Phase 1: Days 1-2
- Phase 2: Days 3-5
- Phase 3: Days 6-7
- Phase 4: Days 8-9
- Phase 5: Day 10

## Deliverables

1. New type-safe observer trait system
2. Updated pipeline builder with observer registry
3. Migrated token-printer tool
4. Comprehensive test suite
5. Complete documentation
6. Performance benchmarks
7. Migration guide for existing tools
