
use slang_error::{CompileResult, CompilerError};
use slang_ir::ast::Statement;
use slang_ir::Visitor;
use slang_shared::CompilationContext;

use crate::semantic_analysis::{
    analyzer::native_functions::register_builtins,
    traits::SemanticResult,
};



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
    let mut analyzer = SemanticAnalyzer::new(context);
    analyzer.analyze(statements)
}

/// Semantic analyzer that uses the visitor pattern for improved modularity
pub struct SemanticAnalyzer<'a> {
    /// Collected semantic errors
    errors: Vec<CompilerError>,
    /// Compilation context for type information and symbol table
    context: &'a mut CompilationContext,
}

impl<'a> SemanticAnalyzer<'a> {
    /// Creates a new semantic analyzer with built-in functions registered
    pub fn new(context: &'a mut CompilationContext) -> Self {
        // Register native functions first
        register_builtins(context);
        
        SemanticAnalyzer {
            errors: Vec::new(),
            context,
        }
    }

    /// Performs semantic analysis on a list of statements by recursively analyzing the AST.
    /// This is the main entry point for semantic analysis within the SemanticAnalyzer struct.
    ///
    /// ### Arguments
    /// * `statements` - The AST statements to analyze
    ///
    /// ### Returns
    /// * `CompileResult<()>` - Ok if no semantic errors were found, otherwise Err with the list of errors
    ///
    /// This function performs static semantic analysis on the entire program, collecting all
    /// semantic errors before returning them as a single result.
    pub fn analyze(&mut self, statements: &[Statement]) -> CompileResult<()> {
        for stmt in statements {
            if let Err(error) = self.analyze_statement(stmt) {
                let compiler_error = error.to_compiler_error(self.context);
                if !self.errors.iter().any(|e| {
                    e.message == compiler_error.message
                        && e.line == compiler_error.line
                        && e.column == compiler_error.column
                }) {
                    self.errors.push(compiler_error);
                }
            }
        }

        if !self.errors.is_empty() {
            Err(std::mem::take(&mut self.errors))
        } else {
            Ok(())
        }
    }

    /// Analyze a statement using the appropriate visitor
    pub fn analyze_statement(&mut self, stmt: &Statement) -> SemanticResult {
        let mut stmt_visitor = crate::semantic_analysis::visitors::statement_visitor::StatementVisitor::new(self.context);
        
        match stmt {
            Statement::FunctionDeclaration(fn_decl) => {
                stmt_visitor.visit_function_declaration(fn_decl)
            }
            Statement::Let(let_stmt) => {
                stmt_visitor.visit_let_statement(let_stmt)
            }
            Statement::Assignment(assign_stmt) => {
                stmt_visitor.visit_assignment_statement(assign_stmt)
            }
            Statement::Return(return_stmt) => {
                stmt_visitor.visit_return_statement(return_stmt)
            }
            Statement::TypeDefinition(type_def) => {
                stmt_visitor.visit_type_definition_statement(type_def)
            }
            Statement::Expression(expr) => {
                stmt_visitor.visit_expression_statement(expr)
            }
            Statement::If(if_stmt) => {
                stmt_visitor.visit_if_statement(if_stmt)
            }
        }
    }
}

impl<'a> Visitor<SemanticResult> for SemanticAnalyzer<'a> {
    fn visit_function_declaration_statement(
        &mut self,
        fn_decl: &slang_ir::ast::FunctionDeclarationStmt,
    ) -> SemanticResult {
        let mut stmt_visitor = crate::semantic_analysis::visitors::statement_visitor::StatementVisitor::new(self.context);
        stmt_visitor.visit_function_declaration(fn_decl)
    }

    fn visit_return_statement(&mut self, return_stmt: &slang_ir::ast::ReturnStatement) -> SemanticResult {
        let mut stmt_visitor = crate::semantic_analysis::visitors::statement_visitor::StatementVisitor::new(self.context);
        stmt_visitor.visit_return_statement(return_stmt)
    }

    fn visit_call_expression(&mut self, call_expr: &slang_ir::ast::FunctionCallExpr) -> SemanticResult {
        let mut expr_visitor = crate::semantic_analysis::visitors::expression_visitor::ExpressionVisitor::new(self.context);
        expr_visitor.visit_call_expression(call_expr)
    }

    fn visit_type_definition_statement(&mut self, type_def: &slang_ir::ast::TypeDefinitionStmt) -> SemanticResult {
        let mut stmt_visitor = crate::semantic_analysis::visitors::statement_visitor::StatementVisitor::new(self.context);
        stmt_visitor.visit_type_definition_statement(type_def)
    }

    fn visit_expression_statement(&mut self, expr: &slang_ir::ast::Expression) -> SemanticResult {
        let mut expr_visitor = crate::semantic_analysis::visitors::expression_visitor::ExpressionVisitor::new(self.context);
        expr_visitor.visit_expression(expr)
    }

    fn visit_let_statement(&mut self, let_stmt: &slang_ir::ast::LetStatement) -> SemanticResult {
        let mut stmt_visitor = crate::semantic_analysis::visitors::statement_visitor::StatementVisitor::new(self.context);
        stmt_visitor.visit_let_statement(let_stmt)
    }

    fn visit_assignment_statement(
        &mut self,
        assign_stmt: &slang_ir::ast::AssignmentStatement,
    ) -> SemanticResult {
        let mut stmt_visitor = crate::semantic_analysis::visitors::statement_visitor::StatementVisitor::new(self.context);
        stmt_visitor.visit_assignment_statement(assign_stmt)
    }

    fn visit_variable_expression(&mut self, var_expr: &slang_ir::ast::VariableExpr) -> SemanticResult {
        let mut expr_visitor = crate::semantic_analysis::visitors::expression_visitor::ExpressionVisitor::new(self.context);
        expr_visitor.visit_variable_expression(var_expr)
    }

    fn visit_literal_expression(&mut self, literal_expr: &slang_ir::ast::LiteralExpr) -> SemanticResult {
        let mut expr_visitor = crate::semantic_analysis::visitors::expression_visitor::ExpressionVisitor::new(self.context);
        expr_visitor.visit_literal_expression(literal_expr)
    }

    fn visit_binary_expression(&mut self, bin_expr: &slang_ir::ast::BinaryExpr) -> SemanticResult {
        let mut expr_visitor = crate::semantic_analysis::visitors::expression_visitor::ExpressionVisitor::new(self.context);
        expr_visitor.visit_binary_expression(bin_expr)
    }

    fn visit_unary_expression(&mut self, unary_expr: &slang_ir::ast::UnaryExpr) -> SemanticResult {
        let mut expr_visitor = crate::semantic_analysis::visitors::expression_visitor::ExpressionVisitor::new(self.context);
        expr_visitor.visit_unary_expression(unary_expr)
    }

    fn visit_conditional_expression(&mut self, cond_expr: &slang_ir::ast::ConditionalExpr) -> SemanticResult {
        let mut expr_visitor = crate::semantic_analysis::visitors::expression_visitor::ExpressionVisitor::new(self.context);
        expr_visitor.visit_conditional_expression(cond_expr)
    }

    fn visit_block_expression(&mut self, block_expr: &slang_ir::ast::BlockExpr) -> SemanticResult {
        let mut expr_visitor = crate::semantic_analysis::visitors::expression_visitor::ExpressionVisitor::new(self.context);
        expr_visitor.visit_block_expression(block_expr)
    }

    fn visit_function_type_expression(&mut self, func_type_expr: &slang_ir::ast::FunctionTypeExpr) -> SemanticResult {
        let mut expr_visitor = crate::semantic_analysis::visitors::expression_visitor::ExpressionVisitor::new(self.context);
        expr_visitor.visit_function_type_expression(func_type_expr)
    }

    fn visit_if_statement(&mut self, if_stmt: &slang_ir::ast::IfStatement) -> SemanticResult {
        let mut stmt_visitor = crate::semantic_analysis::visitors::statement_visitor::StatementVisitor::new(self.context);
        stmt_visitor.visit_if_statement(if_stmt)
    }
}
