use crate::ast::{BinaryExpr, Expression, LetStatement, LiteralExpr, TypeDefinitionStmt, UnaryExpr};

pub trait Visitor<T> {
    #[allow(unused)]
    fn visit_statement(&mut self, stmt: &crate::ast::Statement) -> T;
    fn visit_expression_statement(&mut self, expr: &Expression) -> T;
    fn visit_let_statement(&mut self, stmt: &LetStatement) -> T;
    fn visit_type_definition_statement(&mut self, stmt: &TypeDefinitionStmt) -> T;
    fn visit_expression(&mut self, expr: &Expression) -> T;
    fn visit_binary_expression(&mut self, expr: &BinaryExpr) -> T;
    fn visit_unary_expression(&mut self, expr: &UnaryExpr) -> T;
    fn visit_literal_expression(&mut self, expr: &LiteralExpr) -> T;
    fn visit_variable_expression(&mut self, name: &str) -> T;
}
