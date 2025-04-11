use crate::ast::{BinaryExpr, Expression, LetStatement, LiteralExpr, Statement};
use crate::bytecode::{Chunk, OpCode, Value};
use crate::token::Tokentype;
use crate::visitor::Visitor;

pub struct Compiler {
    pub chunk: Chunk,
    line: usize,
    // Track variables that are in scope
    variables: Vec<String>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            chunk: Chunk::new(),
            line: 1, // Start at line 1
            variables: Vec::new(),
        }
    }

    pub fn compile(&mut self, statements: &[Statement]) -> Result<&Chunk, String> {
        for stmt in statements {
            match stmt.accept(self) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        // Add implicit return at the end
        self.emit_op(OpCode::Return);

        Ok(&self.chunk)
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_byte(byte, self.line);
    }

    fn emit_op(&mut self, op: OpCode) {
        self.chunk.write_op(op, self.line);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant_index = self.chunk.add_constant(value);
        if constant_index > 255 {
            // This is a limitation for now - we can extend later to support more constants
            panic!("Too many constants in one chunk");
        }

        self.emit_op(OpCode::Constant);
        self.emit_byte(constant_index as u8);
    }

}

impl Visitor<Result<(), String>> for Compiler {
    fn visit_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Let(let_stmt) => self.visit_let_statement(let_stmt),
            Statement::Expression(expr) => self.visit_expression_statement(expr),
        }
    }

    fn visit_expression_statement(&mut self, expr: &Expression) -> Result<(), String> {
        // Compile the expression
        self.visit_expression(expr)?;

        // For expression statements, we print the value
        self.emit_op(OpCode::Print);

        // Then pop it from the stack (clean up)
        self.emit_op(OpCode::Pop);

        Ok(())
    }

    fn visit_let_statement(&mut self, let_stmt: &LetStatement) -> Result<(), String> {
        // First compile the value expression
        self.visit_expression(&let_stmt.value)?;

        // Store the variable name for scope tracking
        self.variables.push(let_stmt.name.clone());

        // Add variable name to identifiers table
        let var_index = self.chunk.add_identifier(let_stmt.name.clone());
        if var_index > 255 {
            return Err("Too many variables in one scope".to_string());
        }

        // Emit the set variable instruction
        self.emit_op(OpCode::SetVariable);
        self.emit_byte(var_index as u8);

        Ok(())
    }

    fn visit_expression(&mut self, expr: &Expression) -> Result<(), String> {
        match expr {
            Expression::Literal(lit_expr) => self.visit_literal_expression(lit_expr),
            Expression::Binary(bin_expr) => self.visit_binary_expression(bin_expr),
            Expression::Variable(name) => self.visit_variable_expression(name),
        }
    }

    fn visit_literal_expression(&mut self, lit_expr: &LiteralExpr) -> Result<(), String> {
        match &lit_expr.value {
            crate::ast::Value::Integer(i) => {
                self.emit_constant(Value::Integer(*i));
            }
            crate::ast::Value::String(s) => {
                self.emit_constant(Value::String(s.clone()));
            }
        }

        Ok(())
    }

    fn visit_binary_expression(&mut self, bin_expr: &BinaryExpr) -> Result<(), String> {
        // Compile the left side
        self.visit_expression(&bin_expr.left)?;

        // Compile the right side
        self.visit_expression(&bin_expr.right)?;

        // Emit the appropriate operation
        match bin_expr.operator {
            Tokentype::Plus => self.emit_op(OpCode::Add),
            Tokentype::Minus => self.emit_op(OpCode::Subtract),
            Tokentype::Multiply => self.emit_op(OpCode::Multiply),
            Tokentype::Divide => self.emit_op(OpCode::Divide),
            _ => {
                return Err(format!(
                    "Unsupported binary operator: {:?}",
                    bin_expr.operator
                ));
            }
        }

        Ok(())
    }

    fn visit_variable_expression(&mut self, name: &str) -> Result<(), String> {
        // Add variable name to identifiers table
        let var_index = self.chunk.add_identifier(name.to_string());
        if var_index > 255 {
            return Err("Too many variables".to_string());
        }

        // Emit the get variable instruction
        self.emit_op(OpCode::GetVariable);
        self.emit_byte(var_index as u8);

        Ok(())
    }
}
