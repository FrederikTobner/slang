use crate::ast::{
    BinaryExpr, Expression, LetStatement, LiteralExpr, Statement, Type, UnaryExpr, Value,
};
use crate::token::Tokentype;
use crate::visitor::Visitor;

pub struct TypeChecker {
    variables: std::collections::HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            variables: std::collections::HashMap::new(),
        }
    }

    pub fn check(&mut self, statements: &[Statement]) -> Result<(), String> {
        for stmt in statements {
            match stmt.accept(self) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn can_coerce_numeric(&self, value: &Value, target_type: &Type) -> bool {
        match (value, target_type) {
            (Value::I32(_), Type::I64) => true,

            (Value::U32(_), Type::U64) => true,
            (Value::U32(_), Type::I64) => true,

            (Value::I32(n), Type::U32) => *n >= 0 && *n <= i32::MAX,
            (Value::I32(n), Type::U64) => *n >= 0,

            (Value::I64(n), Type::I32) => *n >= i32::MIN as i64 && *n <= i32::MAX as i64,

            (Value::I64(n), Type::U32) => *n >= 0 && *n <= u32::MAX as i64,
            (Value::I64(n), Type::U64) => *n >= 0,

            (Value::U64(n), Type::U32) => *n <= u32::MAX as u64,
            (Value::U64(n), Type::I32) => *n <= i32::MAX as u64,
            (Value::U64(n), Type::I64) => *n <= i64::MAX as u64,

            (Value::I32(_), Type::I32) => true,
            (Value::I64(_), Type::I64) => true,
            (Value::U32(_), Type::U32) => true,
            (Value::U64(_), Type::U64) => true,
            (Value::String(_), Type::String) => true,

            _ => false,
        }
    }
}

impl Visitor<Result<Type, String>> for TypeChecker {
    fn visit_statement(&mut self, stmt: &Statement) -> Result<Type, String> {
        match stmt {
            Statement::Let(let_stmt) => self.visit_let_statement(let_stmt),
            Statement::Expression(expr) => self.visit_expression_statement(expr),
        }
    }

    fn visit_expression_statement(&mut self, expr: &Expression) -> Result<Type, String> {
        self.visit_expression(expr)
    }

    fn visit_let_statement(&mut self, let_stmt: &LetStatement) -> Result<Type, String> {
        // Check and infer type of the expression
        let expr_type = self.visit_expression(&let_stmt.value)?;

        // If type wasn't specified, infer it
        let final_type = if let_stmt.expr_type == Type::Unknown {
            expr_type.clone()
        } else if let_stmt.expr_type != expr_type {
            // Check if this is a numeric literal that can be coerced
            match &let_stmt.value {
                Expression::Literal(lit) => {
                    if self.can_coerce_numeric(&lit.value, &let_stmt.expr_type) {
                        let_stmt.expr_type.clone()
                    } else {
                        // Type mismatch
                        return Err(format!(
                            "Type mismatch: variable {} is {:?} but expression is {:?}",
                            let_stmt.name, let_stmt.expr_type, expr_type
                        ));
                    }
                }
                _ => {
                    // Type mismatch for non-literal expressions
                    return Err(format!(
                        "Type mismatch: variable {} is {:?} but expression is {:?}",
                        let_stmt.name, let_stmt.expr_type, expr_type
                    ));
                }
            }
        } else {
            let_stmt.expr_type.clone()
        };

        // Add to symbol table
        self.variables
            .insert(let_stmt.name.clone(), final_type.clone());
        Ok(final_type)
    }

    fn visit_expression(&mut self, expr: &Expression) -> Result<Type, String> {
        match expr {
            Expression::Literal(lit_expr) => self.visit_literal_expression(lit_expr),
            Expression::Binary(bin_expr) => self.visit_binary_expression(bin_expr),
            Expression::Variable(name) => self.visit_variable_expression(name),
            Expression::Unary(unary_expr) => self.visit_unary_expression(unary_expr),
        }
    }

    fn visit_literal_expression(&mut self, lit_expr: &LiteralExpr) -> Result<Type, String> {
        // Infer type from literal
        let inferred = match lit_expr.value {
            Value::I32(_) => Type::I32,
            Value::I64(_) => Type::I64,
            Value::U32(_) => Type::U32,
            Value::U64(_) => Type::U64,
            Value::String(_) => Type::String,
        };
        Ok(inferred)
    }

    fn visit_binary_expression(&mut self, bin_expr: &BinaryExpr) -> Result<Type, String> {
        let left_type = self.visit_expression(&bin_expr.left)?;
        let right_type = self.visit_expression(&bin_expr.right)?;

        match (bin_expr.operator, &left_type, &right_type) {
            (
                Tokentype::Plus | Tokentype::Minus | Tokentype::Multiply | Tokentype::Divide,
                Type::I32,
                Type::I32,
            ) => Ok(Type::I32),

            (
                Tokentype::Plus | Tokentype::Minus | Tokentype::Multiply | Tokentype::Divide,
                Type::I64,
                Type::I64,
            ) => Ok(Type::I64),

            (
                Tokentype::Plus | Tokentype::Minus | Tokentype::Multiply | Tokentype::Divide,
                Type::U32,
                Type::U32,
            ) => Ok(Type::U32),

            (
                Tokentype::Plus | Tokentype::Minus | Tokentype::Multiply | Tokentype::Divide,
                Type::U64,
                Type::U64,
            ) => Ok(Type::U64),

            (Tokentype::Plus, Type::String, Type::String) => Ok(Type::String),

            _ => Err(format!(
                "Invalid operation: {:?} {:?} {:?}",
                left_type, bin_expr.operator, right_type
            )),
        }
    }

    fn visit_unary_expression(&mut self, unary_expr: &UnaryExpr) -> Result<Type, String> {
        let operand_type = self.visit_expression(&unary_expr.right)?;

        match (unary_expr.operator, &operand_type) {
            (Tokentype::Minus, Type::I32) => Ok(Type::I32),
            (Tokentype::Minus, Type::I64) => Ok(Type::I64),
            (Tokentype::Minus, Type::U32) => Err("Cannot negate unsigned type U32".to_string()),
            (Tokentype::Minus, Type::U64) => Err("Cannot negate unsigned type U64".to_string()),
            (Tokentype::Minus, Type::String) => Err("Cannot negate string type".to_string()),
            _ => Err(format!(
                "Invalid unary operation: {:?} {:?}",
                unary_expr.operator, operand_type
            )),
        }
    }
    fn visit_variable_expression(&mut self, name: &str) -> Result<Type, String> {
        if let Some(var_type) = self.variables.get(name) {
            Ok(var_type.clone())
        } else {
            Err(format!("Undefined variable: {}", name))
        }
    }
}
