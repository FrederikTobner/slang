use slang_shared::{CompilationContext, DiagnosticEngine};
use slang_error::LineInfo;
use slang_ir::location::Location;
use slang_ir::ast::Statement;
use slang_frontend::{Token};
use slang_backend::bytecode::Chunk;

/// A composable compilation pipeline providing error recovery capabilities
pub struct CompilationPipeline<'a> {
    context: CompilationContext,
    diagnostics: DiagnosticEngine<'a>,
    source: &'a str,
    file_name: Option<String>,
}

impl<'a> CompilationPipeline<'a> {
    /// Create a new compilation pipeline with the given source code
    pub fn new(source: &'a str, file_name: Option<String>) -> Self {
        let context = CompilationContext::new();
        let mut diagnostics = DiagnosticEngine::new();
        
        if let Some(ref name) = file_name {
            diagnostics.set_file_name(name.clone());
        }
        diagnostics.set_source_text(source);
        
        Self {
            context,
            diagnostics,
            source,
            file_name,
        }
    }
    
    /// Enable or disable error recovery mode
    /// In recovery mode, the pipeline continues processing even after errors
    pub fn with_recovery_mode(mut self, enabled: bool) -> Self {
        self.diagnostics.set_recovery_mode(enabled);
        self
    }
    

    
    /// Tokenize the source code
    pub fn tokenize(mut self) -> PipelineStage<'a, Vec<Token>> {
        let tokenize_result = slang_frontend::lexer::tokenize(self.source);
        match tokenize_result {
            Ok(result) => PipelineStage::Success {
                pipeline: self,
                data: result.tokens,
            },
            Err(errors) => {
                let error_info: Vec<_> = errors.iter().map(|error| {
                    (error.error_code.clone(), error.message.clone(), 
                     Location::new(error.position, error.line, error.column, error.token_length.unwrap_or(1)))
                }).collect();
                
                for (error_code, message, location) in error_info {
                    self.diagnostics.emit_error(error_code, message, location);
                }
                
                PipelineStage::Failed { pipeline: self }
            }
        }
    }
    
    /// Parse tokens into an AST
    pub fn parse(self, tokens: Vec<Token>) -> PipelineStage<'a, Vec<Statement>> {
        match self {
            Self { mut context, mut diagnostics, source, file_name } => {
                let line_info = LineInfo::new(&source);
                match slang_frontend::parser::parse(&tokens, &line_info, &mut context) {
                    Ok(statements) => PipelineStage::Success {
                        pipeline: Self { context, diagnostics, source, file_name },
                        data: statements,
                    },
                    Err(errors) => {
                        for error in errors {
                            let location = Location::new(error.position, error.line, error.column, error.token_length.unwrap_or(1));
                            diagnostics.emit_error(error.error_code, error.message, location);
                        }
                        
                        if diagnostics.is_recovery_mode() {
                            PipelineStage::Success {
                                pipeline: Self { context, diagnostics, source, file_name },
                                data: Vec::new(),
                            }
                        } else {
                            PipelineStage::Failed { 
                                pipeline: Self { context, diagnostics, source, file_name }
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Perform semantic analysis on the AST
    pub fn semantic_analysis(self, statements: Vec<Statement>) -> PipelineStage<'a, Vec<Statement>> {
        match self {
            Self { mut context, mut diagnostics, source, file_name } => {
                match slang_frontend::semantic_analyzer::execute(&statements, &mut context) {
                    Ok(()) => PipelineStage::Success {
                        pipeline: Self { context, diagnostics, source, file_name },
                        data: statements,
                    },
                    Err(errors) => {
                        for error in errors {
                            let location = Location::new(error.position, error.line, error.column, error.token_length.unwrap_or(1));
                            diagnostics.emit_error(error.error_code, error.message, location);
                        }
                        
                        // In recovery mode, continue with the statements we have
                        if diagnostics.is_recovery_mode() {
                            PipelineStage::Success {
                                pipeline: Self { context, diagnostics, source, file_name },
                                data: statements,
                            }
                        } else {
                            PipelineStage::Failed { 
                                pipeline: Self { context, diagnostics, source, file_name }
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Generate bytecode from the analyzed AST
    pub fn codegen(self, statements: Vec<Statement>) -> CompilationResult<'a> {
        match self {
            Self { context, mut diagnostics, source: _source, file_name: _file_name } => {
                match slang_backend::codegen::generate_bytecode(&statements) {
                    Ok(chunk) => CompilationResult::Success {
                        chunk,
                        diagnostics,
                        context,
                    },
                    Err(errors) => {
                        for error in errors {
                            let location = Location::new(error.position, error.line, error.column, error.token_length.unwrap_or(1));
                            diagnostics.emit_error(error.error_code, error.message, location);
                        }
                        CompilationResult::Failed {
                            diagnostics,
                            context,
                        }
                    }
                }
            }
        }
    }
    
    /// Finish the pipeline and return the result
    pub fn finish(self) -> CompilationResult<'a> {
        CompilationResult::Failed {
            diagnostics: self.diagnostics,
            context: self.context,
        }
    }
 
}

/// Represents a stage in the compilation pipeline
pub enum PipelineStage<'a, T> {
    /// The stage completed successfully
    Success {
        pipeline: CompilationPipeline<'a>,
        data: T,
    },
    /// The stage failed with errors
    Failed {
        pipeline: CompilationPipeline<'a>,
    },
}

impl<'a, T> PipelineStage<'a, T> {
    /// Continue to the next stage if successful, otherwise propagate the failure
    pub fn and_then<U, F>(self, f: F) -> PipelineStage<'a, U>
    where
        F: FnOnce(CompilationPipeline<'a>, T) -> PipelineStage<'a, U>,
    {
        match self {
            PipelineStage::Success { pipeline, data } => f(pipeline, data),
            PipelineStage::Failed { pipeline } => PipelineStage::Failed { pipeline },
        }
    }
}

/// The final result of compilation
pub enum CompilationResult<'a> {
    /// Compilation succeeded
    Success {
        chunk: Chunk,
        diagnostics: DiagnosticEngine<'a>,
        #[allow(dead_code)]
        context: CompilationContext,
    },
    /// Compilation failed
    Failed {
        diagnostics: DiagnosticEngine<'a>,
        #[allow(dead_code)]
        context: CompilationContext,
    },
}

impl<'a> CompilationResult<'a> {
}
