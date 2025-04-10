use crate::ast::{Expression, Statement, Type, Value, LiteralExpr, BinaryExpr, LetStatement};
use crate::token::Tokentype;
use crate::visitor::Visitor;

pub struct TypeChecker {
    // Could store a symbol table for variable types
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
            // Process the statement and propagate any errors
            match stmt.accept(self) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
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
            // Type mismatch
            return Err(format!(
                "Type mismatch: variable {} is {:?} but expression is {:?}",
                let_stmt.name, let_stmt.expr_type, expr_type
            ));
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
        }
    }

    fn visit_literal_expression(&mut self, lit_expr: &LiteralExpr) -> Result<Type, String> {
        // Infer type from literal
        let inferred = match lit_expr.value {
            Value::Integer(_) => Type::Integer,
            Value::String(_) => Type::String,
        };
        Ok(inferred)
    }

    fn visit_binary_expression(&mut self, bin_expr: &BinaryExpr) -> Result<Type, String> {
        // Check operand types
        let left_type = self.visit_expression(&bin_expr.left)?;
        let right_type = self.visit_expression(&bin_expr.right)?;

        // Type checking rules for binary operations
        match (bin_expr.operator, &left_type, &right_type) {
            // Integer arithmetic
            (Tokentype::Plus, Type::Integer, Type::Integer) => Ok(Type::Integer),
            (Tokentype::Minus, Type::Integer, Type::Integer) => Ok(Type::Integer),
            (Tokentype::Multiply, Type::Integer, Type::Integer) => Ok(Type::Integer),
            (Tokentype::Divide, Type::Integer, Type::Integer) => Ok(Type::Integer),

            // String concatenation
            (Tokentype::Plus, Type::String, Type::String) => Ok(Type::String),

            // Type error
            _ => Err(format!(
                "Invalid operation: {:?} {:?} {:?}",
                left_type, bin_expr.operator, right_type
            )),
        }
    }

    fn visit_variable_expression(&mut self, name: &str) -> Result<Type, String> {
        // Look up variable type
        if let Some(var_type) = self.variables.get(name) {
            Ok(var_type.clone())
        } else {
            Err(format!("Undefined variable: {}", name))
        }
    }
}
