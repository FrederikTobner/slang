use slang_ir::ast::*;
use slang_shared::CompilationContext;

use super::super::{
    traits::SemanticResult,
    visitors::{expression_visitor::ExpressionVisitor, statement_visitor::StatementVisitor},
};

/// Core analyzer that coordinates between statement and expression visitors
///
/// This analyzer provides a unified interface for semantic analysis by delegating
/// to specialized visitors. It maintains efficient visitor reuse to avoid
/// unnecessary allocations and improve performance.
pub struct CoreAnalyzer<'a> {
    context: &'a mut CompilationContext,
}

impl<'a> CoreAnalyzer<'a> {
    /// Create a new core analyzer
    ///
    /// # Arguments
    /// * `context` - The compilation context for symbol management and types
    pub fn new(context: &'a mut CompilationContext) -> Self {
        Self { context }
    }

    /// Provides access to the compilation context.
    pub fn context(&mut self) -> &mut CompilationContext {
        self.context
    }

    /// Analyze a statement using the appropriate visitor
    ///
    /// This method reuses a single visitor instance for all statement operations
    /// to improve performance and reduce allocations.
    pub fn analyze_statement(&mut self, stmt: &Statement) -> SemanticResult {
        let mut stmt_visitor = StatementVisitor::new(self.context);

        match stmt {
            Statement::FunctionDeclaration(fn_decl) => {
                stmt_visitor.visit_function_declaration(fn_decl)
            }
            Statement::Let(let_stmt) => stmt_visitor.visit_let_statement(let_stmt),
            Statement::Assignment(assign_stmt) => {
                stmt_visitor.visit_assignment_statement(assign_stmt)
            }
            Statement::Return(return_stmt) => stmt_visitor.visit_return_statement(return_stmt),
            Statement::TypeDefinition(type_def) => {
                stmt_visitor.visit_type_definition_statement(type_def)
            }
            Statement::Expression(expr) => stmt_visitor.visit_expression_statement(expr),
            Statement::If(if_stmt) => stmt_visitor.visit_if_statement(if_stmt),
        }
    }

    /// Analyze an expression using the expression visitor
    ///
    /// This method reuses a single visitor instance for all expression operations
    /// to improve performance and reduce allocations.
    pub fn analyze_expression(&mut self, expr: &Expression) -> SemanticResult {
        let mut expr_visitor = ExpressionVisitor::new(self.context);
        expr_visitor.visit_expression(expr)
    }

    /// Analyze a block expression
    ///
    /// This method reuses a single visitor instance for all block operations
    /// to improve performance and reduce allocations.
    pub fn analyze_block(&mut self, block: &BlockExpr) -> SemanticResult {
        let mut expr_visitor = ExpressionVisitor::new(self.context);
        expr_visitor.visit_block_expression(block)
    }
}
