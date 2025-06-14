use crate::ast::{
    AssignmentStatement, BinaryExpr, BlockExpr, ConditionalExpr, Expression, FunctionCallExpr,
    FunctionDeclarationStmt, FunctionTypeExpr, IfStatement, LetStatement, LiteralExpr, ReturnStatement, Statement, TypeDefinitionStmt,
    UnaryExpr, VariableExpr,
};

/// Trait implementing the visitor pattern for traversing the AST
///
/// This trait allows implementing different behaviors when traversing
/// the AST, such as type checking, interpretation, or compilation.
///
/// The generic parameter T represents the return type of the visit methods.
pub trait Visitor<T> {
    /// Visit a general statement
    fn visit_statement(&mut self, stmt: &Statement) -> T {
        match stmt {
            Statement::Let(let_stmt) => self.visit_let_statement(let_stmt),
            Statement::Assignment(assign_stmt) => self.visit_assignment_statement(assign_stmt),
            Statement::Expression(expr) => self.visit_expression_statement(expr),
            Statement::TypeDefinition(type_def) => self.visit_type_definition_statement(type_def),
            Statement::FunctionDeclaration(fn_decl) => {
                self.visit_function_declaration_statement(fn_decl)
            }
            Statement::Return(return_stmt) => self.visit_return_statement(return_stmt),
            Statement::If(if_stmt) => self.visit_if_statement(if_stmt),
        }
    }

    /// Visit an expression statement
    fn visit_expression_statement(&mut self, expr: &Expression) -> T;

    /// Visit a variable declaration statement
    fn visit_let_statement(&mut self, stmt: &LetStatement) -> T;

    /// Visit a type definition statement
    fn visit_type_definition_statement(&mut self, stmt: &TypeDefinitionStmt) -> T;

    /// Visit a function declaration statement
    fn visit_function_declaration_statement(&mut self, stmt: &FunctionDeclarationStmt) -> T;

    /// Visit a return statement
    fn visit_return_statement(&mut self, stmt: &ReturnStatement) -> T;

    /// Visit a variable assignment statement
    fn visit_assignment_statement(&mut self, stmt: &AssignmentStatement) -> T;

    /// Visit a general expression
    fn visit_expression(&mut self, expr: &Expression) -> T {
        match expr {
            Expression::Literal(lit) => self.visit_literal_expression(lit),
            Expression::Binary(bin) => self.visit_binary_expression(bin),
            Expression::Variable(var) => self.visit_variable_expression(var),
            Expression::Unary(unary) => self.visit_unary_expression(unary),
            Expression::Call(call) => self.visit_call_expression(call),
            Expression::Conditional(cond) => self.visit_conditional_expression(cond),
            Expression::Block(block) => self.visit_block_expression(block),
            Expression::FunctionType(func_type) => self.visit_function_type_expression(func_type),
        }
    }

    /// Visit a binary expression (e.g., a + b)
    fn visit_binary_expression(&mut self, expr: &BinaryExpr) -> T;

    /// Visit a unary expression (e.g., -x)
    fn visit_unary_expression(&mut self, expr: &UnaryExpr) -> T;

    /// Visit a literal expression (e.g., 42, "hello")
    fn visit_literal_expression(&mut self, expr: &LiteralExpr) -> T;

    /// Visit a variable reference expression
    fn visit_variable_expression(&mut self, var_expr: &VariableExpr) -> T;

    /// Visit a function call expression
    fn visit_call_expression(&mut self, expr: &FunctionCallExpr) -> T;

    /// Visit a conditional expression (if/else)
    fn visit_conditional_expression(&mut self, expr: &ConditionalExpr) -> T;

    /// Visit a block expression
    fn visit_block_expression(&mut self, expr: &BlockExpr) -> T;

    /// Visit a function type expression (e.g., fn(i32, string) -> string)
    fn visit_function_type_expression(&mut self, expr: &FunctionTypeExpr) -> T;

    /// Visit a conditional statement (if/else)
    fn visit_if_statement(&mut self, stmt: &IfStatement) -> T;
}
