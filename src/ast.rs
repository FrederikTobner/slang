use crate::token::Tokentype;
use crate::visitor::Visitor;

// AST Node types
#[derive(Debug)]
pub enum Expression {
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    Variable(String),
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Expression(Expression),
}


#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Value,
    pub expr_type: Type, // Track the expression's type
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    String,
    Unknown, // Used during type inference
}

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    String(String),
}


#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expression>,
    pub operator: Tokentype,
    pub right: Box<Expression>,
    pub expr_type: Type, // Track the expression's type
}

#[derive(Debug)]
pub struct LetStatement {
    pub name: String,
    pub value: Expression,
    pub expr_type: Type, // Track the expression's type
}

impl Statement {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Statement::Let(let_stmt) => visitor.visit_let_statement(let_stmt),
            Statement::Expression(expr) => visitor.visit_expression_statement(expr),
        }
    }
}
impl Expression {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expression::Literal(lit) => visitor.visit_literal_expression(lit),
            Expression::Binary(bin) => visitor.visit_binary_expression(bin),
            Expression::Variable(name) => visitor.visit_variable_expression(name),
        }
    }
}

