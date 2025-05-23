use crate::ast::{Statement, BinaryExpr, Expression, FunctionCallExpr, FunctionDeclarationStmt, LetStatement, LiteralExpr, TypeDefinitionStmt, UnaryExpr};

/// Trait implementing the visitor pattern for traversing the AST
/// 
/// This trait allows implementing different behaviors when traversing
/// the AST, such as type checking, interpretation, or compilation.
/// 
/// The generic parameter T represents the return type of the visit methods.
pub trait Visitor<T> {
    /// Visit a general statement
    fn visit_statement(&mut self, stmt: &Statement) -> T;
    
    /// Visit an expression statement
    fn visit_expression_statement(&mut self, expr: &Expression) -> T;
    
    /// Visit a variable declaration statement
    fn visit_let_statement(&mut self, stmt: &LetStatement) -> T;
    
    /// Visit a type definition statement
    fn visit_type_definition_statement(&mut self, stmt: &TypeDefinitionStmt) -> T;
    
    /// Visit a function declaration statement
    fn visit_function_declaration_statement(&mut self, stmt: &FunctionDeclarationStmt) -> T;
    
    /// Visit a block statement (multiple statements in braces)
    fn visit_block_statement(&mut self, stmts: &[Statement]) -> T;
    
    /// Visit a return statement
    fn visit_return_statement(&mut self, expr: &Option<Expression>) -> T;
    
    /// Visit a general expression
    fn visit_expression(&mut self, expr: &Expression) -> T;
    
    /// Visit a binary expression (e.g., a + b)
    fn visit_binary_expression(&mut self, expr: &BinaryExpr) -> T;
    
    /// Visit a unary expression (e.g., -x)
    fn visit_unary_expression(&mut self, expr: &UnaryExpr) -> T;
    
    /// Visit a literal expression (e.g., 42, "hello")
    fn visit_literal_expression(&mut self, expr: &LiteralExpr) -> T;
    
    /// Visit a variable reference expression
    fn visit_variable_expression(&mut self, name: &str, location: &crate::source_location::SourceLocation) -> T;
    
    /// Visit a function call expression
    fn visit_call_expression(&mut self, expr: &FunctionCallExpr) -> T;
}
