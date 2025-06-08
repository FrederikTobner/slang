use crate::SourceLocation;
use crate::Visitor;
use slang_types::types::TypeId;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    /// Addition operator
    Add,
    /// Subtraction operator
    Subtract,
    /// Multiplication operator
    Multiply,
    /// Division operator
    Divide,
    /// Greater than operator
    GreaterThan,
    /// Less than operator
    LessThan,
    /// Greater than or equal to operator
    GreaterThanOrEqual,
    /// Less than or equal to operator
    LessThanOrEqual,
    /// Equality operator
    Equal,
    /// Not equal operator
    NotEqual,
    /// Logical AND operator
    And,
    /// Logical OR operator
    Or,
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::LessThan => "<",
            BinaryOperator::GreaterThanOrEqual => ">=",
            BinaryOperator::LessThanOrEqual => "<=",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
        };
        write!(f, "{}", op_str)
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    /// Negation operator
    Negate,
    /// Logical NOT operator
    Not,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            UnaryOperator::Negate => "-",
            UnaryOperator::Not => "!",
        };
        write!(f, "{}", op_str)
    }
}

/// Expression nodes in the AST
#[derive(Debug)]
pub enum Expression {
    /// A literal value (constant)
    Literal(LiteralExpr),
    /// A binary operation (e.g., a + b)
    Binary(BinaryExpr),
    /// A variable reference
    Variable(VariableExpr),
    /// A unary operation (e.g., -x)
    Unary(UnaryExpr),
    /// A function call
    Call(FunctionCallExpr),
    /// A conditional expression (if/else)
    Conditional(ConditionalExpr),
    /// A block expression with statements and optional return value
    Block(BlockExpr),
    /// A function type expression (e.g., fn(i32, string) -> string)
    FunctionType(FunctionTypeExpr),
}

impl Expression {
    pub fn location(&self) -> SourceLocation {
        match self {
            Expression::Literal(e) => e.location,
            Expression::Binary(e) => e.location,
            Expression::Variable(e) => e.location,
            Expression::Unary(e) => e.location,
            Expression::Call(e) => e.location,
            Expression::Conditional(e) => e.location,
            Expression::Block(e) => e.location,
            Expression::FunctionType(e) => e.location,
        }
    }
}

/// Statement nodes in the AST
#[derive(Debug)]
pub enum Statement {
    /// Variable declaration
    Let(LetStatement),
    /// Variable assignment
    Assignment(AssignmentStatement),
    /// Expression statement
    Expression(Expression),
    /// Type definition (e.g., struct)
    TypeDefinition(TypeDefinitionStmt),
    /// Function declaration
    FunctionDeclaration(FunctionDeclarationStmt),
    /// Return statement
    Return(ReturnStatement),
    /// Conditional statement (if/else)
    If(IfStatement),
}

/// A function call expression
#[derive(Debug)]
pub struct FunctionCallExpr {
    /// Name of the function being called
    pub name: String,
    /// Arguments passed to the function
    pub arguments: Vec<Expression>,
    /// Type of the function call expression
    pub expr_type: TypeId,
    /// Source code location information
    pub location: SourceLocation,
}

/// A conditional expression (if/else)
#[derive(Debug)]
pub struct ConditionalExpr {
    /// Condition to evaluate
    pub condition: Box<Expression>,
    /// Expression to evaluate if condition is true
    pub then_branch: Box<Expression>,
    /// Expression to evaluate if condition is false (always present for expressions)
    pub else_branch: Box<Expression>,
    /// Type of the conditional expression
    pub expr_type: TypeId,
    /// Source code location information
    pub location: SourceLocation,
}

/// A block expression containing statements and an optional return value
#[derive(Debug)]
pub struct BlockExpr {
    /// Statements in the block
    pub statements: Vec<Statement>,
    /// Optional final expression that becomes the return value (without semicolon)
    pub return_expr: Option<Box<Expression>>,
    /// Type of the block expression
    pub expr_type: TypeId,
    /// Source code location information
    pub location: SourceLocation,
}

/// A function parameter
#[derive(Debug)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: TypeId,
    /// Source code location information
    pub location: SourceLocation,
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
    /// Function body (block expression)
    pub body: BlockExpr,
    /// Source code location information
    pub location: SourceLocation,
}

/// A function type expression (e.g., fn(i32, string) -> string)
#[derive(Debug)]
pub struct FunctionTypeExpr {
    /// Parameter types of the function
    pub param_types: Vec<TypeId>,
    /// Return type of the function
    pub return_type: TypeId,
    /// Type of the function type expression (will be a function type)
    pub expr_type: TypeId,
    /// Source code location information
    pub location: SourceLocation,
}

/// A type definition statement (like struct)
#[derive(Debug)]
pub struct TypeDefinitionStmt {
    /// Name of the defined type
    pub name: String,
    /// Fields of the type with their names and types
    pub fields: Vec<(String, TypeId)>,
    /// Source code location information
    pub location: SourceLocation,
}

/// A literal expression
#[derive(Debug)]
pub struct LiteralExpr {
    /// Value of the literal
    pub value: LiteralValue,
    /// Type of the literal expression
    pub expr_type: TypeId,
    /// Source code location information
    pub location: SourceLocation,
}

/// A variable reference expression
#[derive(Debug)]
pub struct VariableExpr {
    /// Name of the variable being referenced
    pub name: String,
    /// Source code location information
    pub location: SourceLocation,
}

/// A unary expression (e.g., -x)
#[derive(Debug)]
pub struct UnaryExpr {
    /// The operator (e.g., -)
    pub operator: UnaryOperator,
    /// The operand
    pub right: Box<Expression>,
    /// Type of the unary expression
    pub expr_type: TypeId,
    /// Source code location information
    pub location: SourceLocation,
}

/// Possible values for literal expressions
#[derive(Debug)]
pub enum LiteralValue {
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
    /// 32-bit floating point
    F32(f32),
    /// 64-bit floating point
    F64(f64),
    /// Float without specified type (needs inference)
    UnspecifiedFloat(f64),
    /// String value
    String(String),
    /// Boolean value (true or false)
    Boolean(bool),
    /// Unit value (similar to Rust's ())
    Unit,
}

/// A binary expression (e.g., a + b)
#[derive(Debug)]
pub struct BinaryExpr {
    /// Left operand
    pub left: Box<Expression>,
    /// Operator
    pub operator: BinaryOperator,
    /// Right operand
    pub right: Box<Expression>,
    /// Type of the binary expression
    pub expr_type: TypeId,
    /// Source code location information
    pub location: SourceLocation,
}

/// A variable declaration statement
#[derive(Debug)]
pub struct LetStatement {
    /// Name of the variable
    pub name: String,
    /// Whether the variable is mutable
    pub is_mutable: bool,
    /// Initial value for the variable
    pub value: Expression,
    /// Type of the variable
    pub expr_type: TypeId,
    /// Source code location information
    pub location: SourceLocation,
}

/// A variable assignment statement
#[derive(Debug)]
pub struct AssignmentStatement {
    /// Name of the variable being assigned
    pub name: String,
    /// New value for the variable
    pub value: Expression,
    /// Source code location information
    pub location: SourceLocation,
}

/// A conditional statement (if/else)
#[derive(Debug)]
pub struct IfStatement {
    /// Condition to evaluate
    pub condition: Expression,
    /// Block expression to execute if condition is true
    pub then_branch: BlockExpr,
    /// Optional block expression to execute if condition is false
    pub else_branch: Option<BlockExpr>,
    /// Source code location information
    pub location: SourceLocation,
}

/// A return statement
#[derive(Debug)]
pub struct ReturnStatement {
    /// Optional expression to return
    pub value: Option<Expression>,
    /// Source code location information
    pub location: SourceLocation,
}

impl Statement {
    /// Accepts a visitor for this statement
    ///
    /// ### Arguments
    /// * `visitor` - The visitor to accept
    ///
    /// ### Returns
    /// The result of the visitor's visit method for this statement
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Statement::Let(let_stmt) => visitor.visit_let_statement(let_stmt),
            Statement::Assignment(assign_stmt) => visitor.visit_assignment_statement(assign_stmt),
            Statement::Expression(expr) => visitor.visit_expression_statement(expr),
            Statement::TypeDefinition(type_def) => {
                visitor.visit_type_definition_statement(type_def)
            }
            Statement::FunctionDeclaration(fn_decl) => {
                visitor.visit_function_declaration_statement(fn_decl)
            }
            Statement::Return(return_stmt) => visitor.visit_return_statement(return_stmt),
            Statement::If(if_stmt) => visitor.visit_if_statement(if_stmt),
        }
    }
}

impl Expression {
    /// Accepts a visitor for this expression
    ///
    /// ### Arguments
    /// * `visitor` - The visitor to accept
    ///
    /// ### Returns
    /// The result of the visitor's visit method for this expression
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expression::Literal(lit) => visitor.visit_literal_expression(lit),
            Expression::Binary(bin) => visitor.visit_binary_expression(bin),
            Expression::Variable(var) => visitor.visit_variable_expression(var),
            Expression::Unary(unary) => visitor.visit_unary_expression(unary),
            Expression::Call(call) => visitor.visit_call_expression(call),
            Expression::Conditional(cond) => visitor.visit_conditional_expression(cond),
            Expression::Block(block) => visitor.visit_block_expression(block),
            Expression::FunctionType(func_type) => visitor.visit_function_type_expression(func_type),
        }
    }
}
