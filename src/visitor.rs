use crate::ast::{Expression, LiteralExpr, BinaryExpr, Statement, LetStatement};

pub trait Visitor<T> {
    fn visit_let_statement(&mut self, let_stmt: &LetStatement) -> T;
    fn visit_expression_statement(&mut self, expr: &Expression) -> T;
    #[allow(dead_code)]
    fn visit_statement(&mut self, stmt: &Statement) -> T;
    fn visit_literal_expression(&mut self, literal: &LiteralExpr) -> T;
    fn visit_binary_expression(&mut self, binary: &BinaryExpr) -> T;
    fn visit_variable_expression(&mut self, variable: &str) -> T;
    fn visit_expression(&mut self, expr: &Expression) -> T;
}
