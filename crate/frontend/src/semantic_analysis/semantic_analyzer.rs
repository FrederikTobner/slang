use crate::semantic_analysis::ErrorCollector;
use slang_error::CompileResult;
use slang_ir::ast::Statement;
use slang_shared::CompilationContext;

use super::analyzer_modules::core::CoreAnalyzer;
use super::analyzer_modules::native_functions;

/// Performs semantic analysis including type checking on a list of statements.
/// This is the main entry point for the semantic analysis system.
///
/// ### Arguments
/// * `statements` - The AST statements to analyze
/// * `context` - The compilation context
///
/// ### Returns
/// * `CompileResult<()>` - Ok if no semantic errors were found, otherwise Err with the list of errors
pub fn execute(statements: &[Statement], context: &mut CompilationContext) -> CompileResult<()> {
    let mut analyzer = CoreAnalyzer::new(context);
    native_functions::register_native_functions(analyzer.context()); // Register native functions using the accessor

    let mut error_collector = ErrorCollector::new();

    for stmt in statements {
        if let Err(error) = analyzer.analyze_statement(stmt) {
            error_collector.add_semantic_error(error, analyzer.context());
        }
    }

    if error_collector.has_errors() {
        Err(error_collector.into_errors())
    } else {
        Ok(())
    }
}

