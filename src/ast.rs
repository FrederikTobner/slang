use crate::token::Tokentype;
use crate::visitor::Visitor;
use crate::types::TypeId;

/// Expression
#[derive(Debug)]
pub enum Expression {
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    Variable(String),
    Unary(UnaryExpr),
    Call(FunctionCallExpr),
}

/// Statement
#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Expression(Expression),
    TypeDefinition(TypeDefinitionStmt),
    FunctionDeclaration(FunctionDeclarationStmt), 
    Block(Vec<Statement>),
    Return(Option<Expression>), 
}

/// Function call expression
#[derive(Debug)]
pub struct FunctionCallExpr {
    pub name: String,
    pub arguments: Vec<Expression>,
    #[allow(dead_code)]
    pub expr_type: TypeId,
}

/// Function parameter
#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub param_type: TypeId,
}

/// Function declaration statement
#[derive(Debug)]
pub struct FunctionDeclarationStmt {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: TypeId,
    pub body: Vec<Statement>,
}

/// Type definition statement
#[derive(Debug)]
pub struct TypeDefinitionStmt {
    pub name: String,
    pub fields: Vec<(String, TypeId)>,
}

/// Literal expression
#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Value,
    #[allow(dead_code)]
    pub expr_type: TypeId,
}

/// Unary expression
#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Tokentype,
    pub right: Box<Expression>,
    #[allow(dead_code)]
    pub expr_type: TypeId,
}

/// Value of a literal expression
#[derive(Debug)]
pub enum Value {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    UnspecifiedInteger(i64),
    F64(f64),
    String(String),
}

/// Binary expression
#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expression>,
    pub operator: Tokentype,
    pub right: Box<Expression>,
    #[allow(dead_code)]
    pub expr_type: TypeId,
}

/// Let statement
#[derive(Debug)]
pub struct LetStatement {
    pub name: String,
    pub value: Expression,
    pub expr_type: TypeId,
}

impl Statement {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Statement::Let(let_stmt) => visitor.visit_let_statement(let_stmt),
            Statement::Expression(expr) => visitor.visit_expression_statement(expr),
            Statement::TypeDefinition(type_def) => visitor.visit_type_definition_statement(type_def),
            Statement::FunctionDeclaration(fn_decl) => visitor.visit_function_declaration_statement(fn_decl),
            Statement::Block(stmts) => visitor.visit_block_statement(stmts),
            Statement::Return(expr) => visitor.visit_return_statement(expr),
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
            Expression::Call(call) => visitor.visit_call_expression(call),
        }
    }
}

