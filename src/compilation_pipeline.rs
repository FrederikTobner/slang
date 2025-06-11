use slang_backend::bytecode::Chunk;
use slang_error::LineInfo;
use slang_frontend::Token;
use slang_ir::ast::Statement;
use slang_shared::{CompilationContext, DiagnosticEngine};

/// A composable compilation pipeline providing error recovery capabilities
///
/// The CompilationPipeline implements a multi-stage compilation process that can
/// gracefully handle errors and continue processing when possible. It integrates
/// with the DiagnosticEngine to provide rich error reporting and supports both
/// fail-fast and error recovery modes.
///
/// ### Features
/// - Stage-by-stage compilation with clear error boundaries
/// - Error recovery mode for collecting multiple errors in one pass
/// - Rich diagnostic reporting with source context
/// - Composable pipeline stages that can be chained or used independently
/// - Integration with all compiler phases: lexing, parsing, semantic analysis, codegen
///
/// ### Example Usage
/// ```rust
/// use slang::CompilationPipeline;
///
/// let source = "let x = 42;";
/// let result = CompilationPipeline::new(source, Some("example.sl".to_string()))
///     .with_recovery_mode(true)
///     .tokenize()
///     .and_then(|pipeline, tokens| pipeline.parse(tokens))
///     .and_then(|pipeline, ast| pipeline.semantic_analysis(ast))
///     .and_then(|pipeline, ast| pipeline.codegen(ast));
/// ```
pub struct CompilationPipeline<'a> {
    /// The compilation context containing symbol tables and type information
    context: CompilationContext,
    /// The diagnostic engine for collecting and reporting errors
    diagnostics: DiagnosticEngine<'a>,
    /// The source code being compiled
    source: &'a str,
    /// Optional file name for better error reporting
    file_name: Option<String>,
}

impl<'a> CompilationPipeline<'a> {
    /// Creates a new compilation pipeline with the given source code
    ///
    /// This initializes a fresh compilation pipeline with default settings.
    /// The diagnostic engine is automatically configured with the provided
    /// file name and source text for better error reporting.
    ///
    /// ### Arguments
    /// * `source` - The source code to compile
    /// * `file_name` - Optional file name for error reporting context
    ///
    /// ### Returns
    /// A new CompilationPipeline ready for processing
    ///
    /// ### Example
    /// ```rust
    /// let pipeline = CompilationPipeline::new(
    ///     "let x = 42;",
    ///     Some("example.sl".to_string())
    /// );
    /// ```
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

    /// Enables or disables error recovery mode
    ///
    /// In recovery mode, the pipeline continues processing even after encountering
    /// errors, allowing multiple issues to be discovered in a single compilation pass.
    /// This is useful for IDEs and development tools that want to show all errors
    /// rather than stopping at the first one.
    ///
    /// ### Arguments
    /// * `enabled` - Whether to enable error recovery mode
    ///
    /// ### Returns
    /// The pipeline with recovery mode configured
    ///
    /// ### Example
    /// ```rust
    /// let pipeline = CompilationPipeline::new(source, None)
    ///     .with_recovery_mode(true);
    /// ```
    pub fn with_recovery_mode(mut self, enabled: bool) -> Self {
        self.diagnostics.set_recovery_mode(enabled);
        self
    }

    /// Tokenizes the source code into a stream of tokens
    ///
    /// This is the first stage of compilation, converting raw source text into
    /// structured tokens that can be processed by the parser. Any lexical errors
    /// (invalid characters, malformed numbers, etc.) are collected and reported.
    ///
    /// ### Returns
    /// A PipelineStage containing either:
    /// - Success: The pipeline and tokenized output
    /// - Failed: The pipeline with diagnostic information about lexical errors
    ///
    /// ### Example
    /// ```rust
    /// let result = pipeline.tokenize();
    /// match result {
    ///     PipelineStage::Success { pipeline, data: tokens } => {
    ///         // Continue with parsing...
    ///     }
    ///     PipelineStage::Failed { pipeline } => {
    ///         // Handle tokenization errors
    ///     }
    /// }
    /// ```
    pub fn tokenize(mut self) -> PipelineStage<'a, Vec<Token>> {
        let tokenize_result = slang_frontend::lexer::tokenize(self.source);
        match tokenize_result {
            Ok(result) => PipelineStage::Success {
                pipeline: self,
                data: result.tokens,
            },
            Err(errors) => {
                for error in errors {
                    self.diagnostics.emit_compiler_error(error);
                }
                PipelineStage::Failed { pipeline: self }
            }
        }
    }

    /// Parses tokens into an Abstract Syntax Tree (AST)
    ///
    /// This stage takes the token stream from the lexer and builds a structured
    /// representation of the program. It handles syntax errors and can recover
    /// from many parse errors when recovery mode is enabled.
    ///
    /// ### Arguments
    /// * `tokens` - The vector of tokens to parse
    ///
    /// ### Returns
    /// A PipelineStage containing either:
    /// - Success: The pipeline and parsed AST statements
    /// - Failed: The pipeline with diagnostic information about syntax errors
    ///
    /// ### Error Recovery
    /// In recovery mode, parsing errors don't stop the pipeline. Instead, the
    /// parser attempts to recover and continue, allowing downstream stages to
    /// process what was successfully parsed.
    ///
    /// ### Example
    /// ```rust
    /// let result = pipeline.parse(tokens);
    /// ```
    pub fn parse(self, tokens: Vec<Token>) -> PipelineStage<'a, Vec<Statement>> {
        match self {
            Self {
                mut context,
                mut diagnostics,
                source,
                file_name,
            } => {
                let line_info = LineInfo::new(&source);
                match slang_frontend::parser::parse(&tokens, &line_info, &mut context) {
                    Ok(statements) => PipelineStage::Success {
                        pipeline: Self {
                            context,
                            diagnostics,
                            source,
                            file_name,
                        },
                        data: statements,
                    },
                    Err(errors) => {
                        for error in errors {
                            diagnostics.emit_compiler_error(error);
                        }

                        if diagnostics.is_recovery_mode() {
                            PipelineStage::Success {
                                pipeline: Self {
                                    context,
                                    diagnostics,
                                    source,
                                    file_name,
                                },
                                data: Vec::new(),
                            }
                        } else {
                            PipelineStage::Failed {
                                pipeline: Self {
                                    context,
                                    diagnostics,
                                    source,
                                    file_name,
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    /// Performs semantic analysis on the parsed AST
    ///
    /// This stage validates the semantic correctness of the program, including:
    /// - Type checking and inference
    /// - Variable declaration and usage validation
    /// - Scope resolution and symbol table management
    /// - Function call verification
    ///
    /// ### Arguments
    /// * `statements` - The parsed AST statements to analyze
    ///
    /// ### Returns
    /// A PipelineStage containing either:
    /// - Success: The pipeline and semantically validated statements
    /// - Failed: The pipeline with diagnostic information about semantic errors
    ///
    /// ### Error Recovery
    /// In recovery mode, semantic errors don't prevent code generation from
    /// proceeding, though the generated code may not be valid for execution.
    ///
    /// ### Example
    /// ```rust
    /// let result = pipeline.semantic_analysis(statements);
    /// ```
    pub fn semantic_analysis(
        self,
        statements: Vec<Statement>,
    ) -> PipelineStage<'a, Vec<Statement>> {
        match self {
            Self {
                mut context,
                mut diagnostics,
                source,
                file_name,
            } => {
                match slang_frontend::semantic_analysis::execute(&statements, &mut context) {
                    Ok(()) => PipelineStage::Success {
                        pipeline: Self {
                            context,
                            diagnostics,
                            source,
                            file_name,
                        },
                        data: statements,
                    },
                    Err(errors) => {
                        for error in errors {
                            diagnostics.emit_compiler_error(error);
                        }

                        // In recovery mode, continue with the statements we have
                        if diagnostics.is_recovery_mode() {
                            PipelineStage::Success {
                                pipeline: Self {
                                    context,
                                    diagnostics,
                                    source,
                                    file_name,
                                },
                                data: statements,
                            }
                        } else {
                            PipelineStage::Failed {
                                pipeline: Self {
                                    context,
                                    diagnostics,
                                    source,
                                    file_name,
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    /// Generates bytecode from the semantically analyzed AST
    ///
    /// This is the final compilation stage that produces executable bytecode
    /// from the validated AST. The generated bytecode can be executed by the
    /// Slang virtual machine.
    ///
    /// ### Arguments
    /// * `statements` - The semantically validated AST statements
    ///
    /// ### Returns
    /// A CompilationResult containing either:
    /// - Success: The generated bytecode chunk and diagnostic engine
    /// - Failed: The diagnostic engine with code generation errors
    ///
    /// ### Example
    /// ```rust
    /// let result = pipeline.codegen(statements);
    /// match result {
    ///     CompilationResult::Success { chunk, diagnostics } => {
    ///         // Execute the bytecode...
    ///     }
    ///     CompilationResult::Failed { diagnostics } => {
    ///         // Handle codegen errors
    ///     }
    /// }
    /// ```
    pub fn codegen(self, statements: Vec<Statement>) -> CompilationResult<'a> {
        match self {
            Self {
                context: _context,
                mut diagnostics,
                source: _source,
                file_name: _file_name,
            } => match slang_backend::codegen::generate_bytecode(&statements) {
                Ok(chunk) => CompilationResult::Success { chunk, diagnostics },
                Err(errors) => {
                    for error in errors {
                        diagnostics.emit_compiler_error(error);
                    }
                    CompilationResult::Failed { diagnostics }
                }
            },
        }
    }

    /// Finalizes the pipeline and returns a failed compilation result
    ///
    /// This method is typically called when the pipeline needs to terminate
    /// early due to unrecoverable errors. It preserves all collected diagnostics
    /// for error reporting.
    ///
    /// ### Returns
    /// A CompilationResult::Failed containing the diagnostic information
    ///
    /// ### Example
    /// ```rust
    /// let result = pipeline.finish();
    /// // Always returns CompilationResult::Failed
    /// ```
    pub fn finish(self) -> CompilationResult<'a> {
        CompilationResult::Failed {
            diagnostics: self.diagnostics,
        }
    }
}

/// Represents a stage in the compilation pipeline
///
/// Each compilation stage can either succeed and produce data for the next stage,
/// or fail with diagnostic information. This enum enables composition of pipeline
/// stages while preserving error information throughout the process.
///
/// ### Variants
/// - `Success`: The stage completed successfully with data for the next stage
/// - `Failed`: The stage failed and compilation cannot continue normally
///
/// ### Example
/// ```rust
/// match pipeline_stage {
///     PipelineStage::Success { pipeline, data } => {
///         // Continue to next stage
///         pipeline.next_stage(data)
///     }
///     PipelineStage::Failed { pipeline } => {
///         // Handle failure or terminate
///         pipeline.finish()
///     }
/// }
/// ```
pub enum PipelineStage<'a, T> {
    /// The stage completed successfully with data for the next stage
    Success {
        /// The pipeline ready for the next stage
        pipeline: CompilationPipeline<'a>,
        /// The data produced by this stage
        data: T,
    },
    /// The stage failed and compilation cannot continue normally
    Failed {
        /// The pipeline containing diagnostic information about the failure
        pipeline: CompilationPipeline<'a>,
    },
}

impl<'a, T> PipelineStage<'a, T> {
    /// Chains pipeline stages together, continuing only on success
    ///
    /// This method enables functional composition of pipeline stages. If the current
    /// stage succeeded, the provided function is called with the pipeline and data.
    /// If the current stage failed, the failure is propagated without calling the function.
    ///
    /// ### Arguments
    /// * `f` - A function that takes the pipeline and data to produce the next stage
    ///
    /// ### Returns
    /// The result of calling `f` if successful, or a propagated failure
    ///
    /// ### Example
    /// ```rust
    /// let result = pipeline
    ///     .tokenize()
    ///     .and_then(|pipeline, tokens| pipeline.parse(tokens))
    ///     .and_then(|pipeline, ast| pipeline.semantic_analysis(ast));
    /// ```
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
///
/// This enum represents the ultimate outcome of the entire compilation process,
/// containing either successful bytecode generation or failure information with
/// comprehensive diagnostics.
///
/// ### Variants
/// - `Success`: Compilation completed successfully with executable bytecode
/// - `Failed`: Compilation failed with diagnostic information for error reporting
///
/// ### Example
/// ```rust
/// match compilation_result {
///     CompilationResult::Success { chunk, diagnostics } => {
///         // Execute the bytecode or handle warnings
///         if diagnostics.warning_count() > 0 {
///             diagnostics.report_all(&source);
///         }
///         vm.execute(chunk);
///     }
///     CompilationResult::Failed { diagnostics } => {
///         // Report all errors and warnings
///         diagnostics.report_all(&source);
///         std::process::exit(1);
///     }
/// }
/// ```
pub enum CompilationResult<'a> {
    /// Compilation succeeded with executable bytecode
    Success {
        /// The generated bytecode chunk ready for execution
        chunk: Chunk,
        /// The diagnostic engine containing any warnings or notes
        diagnostics: DiagnosticEngine<'a>,
    },
    /// Compilation failed with error information
    Failed {
        /// The diagnostic engine containing all errors, warnings, and notes
        diagnostics: DiagnosticEngine<'a>,
    },
}

/// Creates a compilation pipeline with the given configuration
///
/// This is a convenience function for creating and configuring a compilation pipeline
/// in a single call. It's equivalent to calling `CompilationPipeline::new()` followed
/// by `with_recovery_mode()`.
///
/// ### Arguments
/// * `source` - The source code to compile
/// * `file_name` - Optional file name for better error reporting
/// * `recovery_mode` - Whether to enable error recovery mode
///
/// ### Returns
/// A configured compilation pipeline ready for processing
///
/// ### Example
/// ```rust
/// let pipeline = create_pipeline(
///     "let x = 42;",
///     Some("example.sl".to_string()),
///     true  // Enable recovery mode
/// );
/// ```
pub fn create_pipeline(
    source: &str,
    file_name: Option<String>,
    recovery_mode: bool,
) -> CompilationPipeline {
    CompilationPipeline::new(source, file_name).with_recovery_mode(recovery_mode)
}

/// Executes all compilation stages through the pipeline
///
/// This function runs the complete compilation pipeline from tokenization through
/// code generation. It uses the `and_then` combinator to chain stages together,
/// ensuring that each stage only runs if the previous one succeeded (unless in
/// recovery mode).
///
/// ### Arguments
/// * `pipeline` - The compilation pipeline to execute
///
/// ### Returns
/// The final compilation result with either bytecode or error information
///
/// ### Example
/// ```rust
/// let pipeline = create_pipeline(source, file_name, recovery_mode);
/// let result = execute_compilation_stages(pipeline);
/// ```
pub fn execute_compilation_stages(pipeline: CompilationPipeline) -> CompilationResult {
    match pipeline
        .tokenize()
        .and_then(|pipeline, tokens| pipeline.parse(tokens))
        .and_then(|pipeline, statements| pipeline.semantic_analysis(statements))
    {
        PipelineStage::Success { pipeline, data } => pipeline.codegen(data),
        PipelineStage::Failed { pipeline } => pipeline.finish(),
    }
}

/// Compiles source code to bytecode using the diagnostic-aware pipeline
///
/// This is the highest-level compilation function that combines pipeline creation
/// and execution into a single call. It's the main entry point for most compilation
/// tasks and provides the complete end-to-end compilation experience.
///
/// ### Arguments
/// * `source` - The source code to compile
/// * `file_name` - Optional file name for better error reporting and debugging
/// * `recovery_mode` - Whether to enable error recovery mode for collecting multiple errors
///
/// ### Returns
/// The compilation result with either executable bytecode or comprehensive error information
///
/// ### Example
/// ```rust
/// let result = compile_to_bytecode(
///     "let x = 42; print(x);",
///     Some("example.sl".to_string()),
///     true  // Enable recovery mode
/// );
///
/// match result {
///     CompilationResult::Success { chunk, diagnostics } => {
///         // Handle any warnings
///         if diagnostics.warning_count() > 0 {
///             diagnostics.report_all(source);
///         }
///         // Execute the bytecode
///         vm.execute(chunk);
///     }
///     CompilationResult::Failed { diagnostics } => {
///         // Report all errors
///         diagnostics.report_all(source);
///         std::process::exit(1);
///     }
/// }
/// ```
pub fn compile_to_bytecode(
    source: &str,
    file_name: Option<String>,
    recovery_mode: bool,
) -> CompilationResult {
    let pipeline = create_pipeline(source, file_name, recovery_mode);
    execute_compilation_stages(pipeline)
}
