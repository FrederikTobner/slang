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
            Self { context: _context, mut diagnostics, source: _source, file_name: _file_name } => {
                match slang_backend::codegen::generate_bytecode(&statements) {
                    Ok(chunk) => CompilationResult::Success {
                        chunk,
                        diagnostics,
                    },
                    Err(errors) => {
                        for error in errors {
                            let location = Location::new(error.position, error.line, error.column, error.token_length.unwrap_or(1));
                            diagnostics.emit_error(error.error_code, error.message, location);
                        }
                        CompilationResult::Failed {
                            diagnostics,
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
    },
    /// Compilation failed
    Failed {
        diagnostics: DiagnosticEngine<'a>,
    },
}

impl<'a> CompilationResult<'a> {
}

/// Create a compilation pipeline with the given configuration
///
/// ### Arguments
/// * `source` - The source code to compile
/// * `file_name` - Optional file name for better error reporting
/// * `recovery_mode` - Whether to enable error recovery mode
///
/// ### Returns
/// A configured compilation pipeline
pub fn create_pipeline(source: &str, file_name: Option<String>, recovery_mode: bool) -> CompilationPipeline {
    CompilationPipeline::new(source, file_name)
        .with_recovery_mode(recovery_mode)
}

/// Execute all compilation stages through the pipeline
///
/// ### Arguments
/// * `pipeline` - The compilation pipeline to execute
///
/// ### Returns
/// The final compilation result
pub fn execute_compilation_stages(pipeline: CompilationPipeline) -> CompilationResult {
    let tokenize_stage = pipeline.tokenize();
    
    let parse_stage = tokenize_stage.and_then(|pipeline, tokens| {
        pipeline.parse(tokens)
    });
    
    let semantic_stage = parse_stage.and_then(|pipeline, statements| {
        pipeline.semantic_analysis(statements)
    });
    
    match semantic_stage {
        PipelineStage::Success { pipeline, data } => {
            pipeline.codegen(data)
        }
        PipelineStage::Failed { pipeline } => {
            pipeline.finish()
        }
    }
}

/// Compile source code to bytecode using the diagnostic-aware pipeline
///
/// ### Arguments
/// * `source` - The source code to compile
/// * `file_name` - Optional file name for better error reporting
/// * `recovery_mode` - Whether to enable error recovery mode
///
/// ### Returns
/// The compilation result with diagnostics
pub fn compile_to_bytecode(
    source: &str, 
    file_name: Option<String>,
    recovery_mode: bool
) -> CompilationResult {
    let pipeline = create_pipeline(source, file_name, recovery_mode);
    execute_compilation_stages(pipeline)
}
