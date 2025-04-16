use crate::token::Tokentype;
use crate::visitor::Visitor;

// AST Node types
#[derive(Debug)]
pub enum Expression {
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    Variable(String),
    Unary(UnaryExpr),
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Expression(Expression),
}


#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Value,
    #[allow(dead_code)]
    pub expr_type: Type, // Track the expression's type
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Tokentype,
    pub right: Box<Expression>,
    #[allow(dead_code)]
    pub expr_type: Type, // Track the expression's type
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    I64,
    U32,
    U64,
    String,
    Unknown, // Used during type inference
}

#[derive(Debug)]
pub enum Value {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    String(String),
}


#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expression>,
    pub operator: Tokentype,
    pub right: Box<Expression>,
    #[allow(dead_code)] 
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
            Expression::Unary(unary) => visitor.visit_unary_expression(unary),
        }
    }
}

