use crate::token::Tokentype;
use crate::visitor::Visitor;
use crate::types::TypeId;

/// Expression nodes in the AST
#[derive(Debug)]
pub enum Expression {
    /// A literal value (constant)
    Literal(LiteralExpr),
    /// A binary operation (e.g., a + b)
    Binary(BinaryExpr),
    /// A variable reference
    Variable(String),
    /// A unary operation (e.g., -x)
    Unary(UnaryExpr),
    /// A function call
    Call(FunctionCallExpr),
}

/// Statement nodes in the AST
#[derive(Debug)]
pub enum Statement {
    /// Variable declaration
    Let(LetStatement),
    /// Expression statement
    Expression(Expression),
    /// Type definition (e.g., struct)
    TypeDefinition(TypeDefinitionStmt),
    /// Function declaration
    FunctionDeclaration(FunctionDeclarationStmt), 
    /// Block of statements
    Block(Vec<Statement>),
    /// Return statement
    Return(Option<Expression>), 
}

/// A function call expression
#[derive(Debug)]
pub struct FunctionCallExpr {
    /// Name of the function being called
    pub name: String,
    /// Arguments passed to the function
    pub arguments: Vec<Expression>,
    /// Type of the function call expression
    #[allow(dead_code)]
    pub expr_type: TypeId,
}

/// A function parameter
#[derive(Debug)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: TypeId,
}

/// A function declaration statement
#[derive(Debug)]
pub struct FunctionDeclarationStmt {
    /// Function name
    pub name: String,
    /// Function parameters
    pub parameters: Vec<Parameter>,
    /// Function return type
    pub return_type: TypeId,
    /// Function body (list of statements)
    pub body: Vec<Statement>,
}

/// A type definition statement (like struct)
#[derive(Debug)]
pub struct TypeDefinitionStmt {
    /// Name of the defined type
    pub name: String,
    /// Fields of the type with their names and types
    pub fields: Vec<(String, TypeId)>,
}

/// A literal expression
#[derive(Debug)]
pub struct LiteralExpr {
    /// Value of the literal
    pub value: Value,
    /// Type of the literal expression
    #[allow(dead_code)]
    pub expr_type: TypeId,
}

/// A unary expression (e.g., -x)
#[derive(Debug)]
pub struct UnaryExpr {
    /// The operator (e.g., -)
    pub operator: Tokentype,
    /// The operand
    pub right: Box<Expression>,
    /// Type of the unary expression
    #[allow(dead_code)]
    pub expr_type: TypeId,
}

/// Possible values for literal expressions
#[derive(Debug)]
pub enum Value {
    /// 32-bit signed integer
    I32(i32),
    /// 64-bit signed integer
    I64(i64),
    /// 32-bit unsigned integer
    U32(u32),
    /// 64-bit unsigned integer
    U64(u64),
    /// Integer without specified type (needs inference)
    UnspecifiedInteger(i64),
    /// 64-bit floating point
    F64(f64),
    /// String value
    String(String),
}

/// A binary expression (e.g., a + b)
#[derive(Debug)]
pub struct BinaryExpr {
    /// Left operand
    pub left: Box<Expression>,
    /// Operator
    pub operator: Tokentype,
    /// Right operand
    pub right: Box<Expression>,
    /// Type of the binary expression
    #[allow(dead_code)]
    pub expr_type: TypeId,
}

/// A variable declaration statement
#[derive(Debug)]
pub struct LetStatement {
    /// Name of the variable
    pub name: String,
    /// Initial value for the variable
    pub value: Expression,
    /// Type of the variable
    pub expr_type: TypeId,
}

impl Statement {
    /// Accepts a visitor for this statement
    /// 
    /// This is part of the visitor pattern implementation.
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
    /// Accepts a visitor for this expression
    /// 
    /// This is part of the visitor pattern implementation.
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

