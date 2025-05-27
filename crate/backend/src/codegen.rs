use crate::bytecode::{Chunk, Function, OpCode};
use crate::value::Value;
use slang_ir::ast::{
    BinaryExpr, BinaryOperator, ConditionalExpr, Expression, FunctionCallExpr, FunctionDeclarationStmt,
    IfStatement, LetStatement, LiteralExpr, Statement, TypeDefinitionStmt, UnaryExpr, UnaryOperator,
};
use slang_ir::Visitor;

/// Compiles AST nodes into bytecode instructions
struct CodeGenerator {
    /// The bytecode chunk being constructed
    pub chunk: Chunk,
    /// Current line number for debugging information
    line: usize,
    /// Global variable names
    variables: Vec<String>,
    /// Function names for tracking declarations
    functions: Vec<String>,
    /// Stack of scopes for tracking local variables
    local_scopes: Vec<Vec<String>>,
}

pub fn generate_bytecode(statements: &[Statement]) -> Result<Chunk, String> {
    let compiler = CodeGenerator::new();
    compiler.compile(statements)
}

impl CodeGenerator {
    /// Creates a new compiler with an empty chunk
    fn new() -> Self {
        CodeGenerator {
            chunk: Chunk::new(),
            line: 1,
            variables: Vec::new(),
            functions: Vec::new(),
            local_scopes: Vec::new(),
        }
    }

    /// Compiles a list of statements into bytecode
    ///
    /// ### Arguments
    ///
    /// * `statements` - The statements to compile
    ///
    /// ### Returns
    ///
    /// A reference to the compiled bytecode chunk, or an error message
    fn compile(mut self, statements: &[Statement]) -> Result<Chunk, String> {
        for stmt in statements {
            match stmt.accept(&mut self) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        self.emit_op(OpCode::Return);

        Ok(self.chunk)
    }

    /// Emits a single byte to the bytecode chunk
    ///
    /// ### Arguments
    ///
    /// * `byte` - The byte to emit
    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_byte(byte, self.line);
    }

    /// Emits an opcode to the bytecode chunk
    ///
    /// ### Arguments
    ///
    /// * `op` - The opcode to emit
    fn emit_op(&mut self, op: OpCode) {
        self.chunk.write_op(op, self.line);
    }

    /// Adds a constant value to the chunk and emits code to load it
    ///
    /// ### Arguments
    ///
    /// * `value` - The constant value to add
    fn emit_constant(&mut self, value: Value) {
        let constant_index = self.chunk.add_constant(value);
        if constant_index > 255 {
            panic!("Too many constants in one chunk");
        }

        self.emit_op(OpCode::Constant);
        self.emit_byte(constant_index as u8);
    }

    /// Emits a jump instruction with placeholder offset
    ///
    /// ### Arguments
    ///
    /// * `op` - The jump opcode (Jump or JumpIfFalse)
    ///
    /// ### Returns
    ///
    /// The position where the jump offset needs to be patched later
    fn emit_jump(&mut self, op: OpCode) -> usize {
        self.emit_op(op);
        self.emit_byte(0xFF);
        self.emit_byte(0xFF);
        self.chunk.code.len() - 2
    }

    /// Patches a previously emitted jump instruction with the actual offset
    ///
    /// ### Arguments
    ///
    /// * `offset` - The position of the jump offset to patch
    fn patch_jump(&mut self, offset: usize) {
        let jump = self.chunk.code.len() - offset - 2;
        if jump > 0xFFFF {
            panic!("Jump too far");
        }

        self.chunk.code[offset] = ((jump >> 8) & 0xFF) as u8;
        self.chunk.code[offset + 1] = (jump & 0xFF) as u8;
    }

    fn begin_scope(&mut self) {
        self.local_scopes.push(Vec::new());
    }

    fn end_scope(&mut self) {
        if let Some(scope) = self.local_scopes.pop() {
            for _ in 0..scope.len() {
                self.emit_op(OpCode::Pop);
            }
        }
    }
}

impl Visitor<Result<(), String>> for CodeGenerator {
    fn visit_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Let(let_stmt) => self.visit_let_statement(let_stmt),
            Statement::Assignment(assign_stmt) => self.visit_assignment_statement(assign_stmt),
            Statement::TypeDefinition(type_stmt) => self.visit_type_definition_statement(type_stmt),
            Statement::Expression(expr) => self.visit_expression_statement(expr),
            Statement::FunctionDeclaration(fn_decl) => {
                self.visit_function_declaration_statement(fn_decl)
            }
            Statement::Block(stmts) => self.visit_block_statement(stmts),
            Statement::Return(expr) => self.visit_return_statement(expr),
            Statement::If(if_stmt) => self.visit_if_statement(if_stmt),
        }
    }

    fn visit_block_statement(&mut self, stmts: &[Statement]) -> Result<(), String> {
        self.begin_scope();

        for stmt in stmts {
            stmt.accept(self)?;
        }

        self.end_scope();
        Ok(())
    }

    fn visit_function_declaration_statement(
        &mut self,
        fn_decl: &FunctionDeclarationStmt,
    ) -> Result<(), String> {
        self.functions.push(fn_decl.name.clone());
        let function_name_idx = self.chunk.add_identifier(fn_decl.name.clone());

        let jump_over = self.emit_jump(OpCode::Jump);

        let code_offset = self.chunk.code.len();
        let mut locals = Vec::new();

        self.begin_scope();
        for param in &fn_decl.parameters {
            locals.push(param.name.clone());
            if let Some(current_scope) = self.local_scopes.last_mut() {
                current_scope.push(param.name.clone());
            }
        }

        for stmt in &fn_decl.body {
            stmt.accept(self)?;
        }

        self.emit_op(OpCode::Return);

        self.end_scope();
        self.patch_jump(jump_over);

        let function = Value::Function(Box::new(Function {
            name: fn_decl.name.clone(),
            arity: fn_decl.parameters.len() as u8,
            code_offset,
            locals,
        }));
        let fn_constant = self.chunk.add_constant(function);

        self.emit_op(OpCode::DefineFunction);
        self.emit_byte(function_name_idx as u8);
        self.emit_byte(fn_constant as u8);

        Ok(())
    }

    fn visit_return_statement(&mut self, expr: &Option<Expression>) -> Result<(), String> {
        if let Some(expr) = expr {
            self.visit_expression(expr)?;
        } else {
            self.emit_constant(Value::I32(0));
        }
        self.emit_op(OpCode::Return);
        Ok(())
    }

    fn visit_expression_statement(&mut self, expr: &Expression) -> Result<(), String> {
        self.visit_expression(expr)?;
        Ok(())
    }

    fn visit_let_statement(&mut self, let_stmt: &LetStatement) -> Result<(), String> {
        let is_local = !self.local_scopes.is_empty();

        if is_local {
            if let Some(current_scope) = self.local_scopes.last_mut() {
                current_scope.push(let_stmt.name.clone());
            }
        } else {
            self.variables.push(let_stmt.name.clone());
        }

        self.visit_expression(&let_stmt.value)?;

        let var_index = self.chunk.add_identifier(let_stmt.name.clone());
        if var_index > 255 {
            return Err("Too many variables in one scope".to_string());
        }

        self.emit_op(OpCode::SetVariable);
        self.emit_byte(var_index as u8);

        Ok(())
    }

    fn visit_assignment_statement(&mut self, assign_stmt: &slang_ir::ast::AssignmentStatement) -> Result<(), String> {
        self.visit_expression(&assign_stmt.value)?;
        let var_index = self.chunk.add_identifier(assign_stmt.name.clone());
        if var_index > 255 {
            return Err("Too many variables in one scope".to_string());
        }
        self.emit_op(OpCode::SetVariable);
        self.emit_byte(var_index as u8);

        Ok(())
    }

    fn visit_expression(&mut self, expr: &Expression) -> Result<(), String> {
        match expr {
            Expression::Literal(lit_expr) => self.visit_literal_expression(lit_expr),
            Expression::Binary(bin_expr) => self.visit_binary_expression(bin_expr),
            Expression::Variable(name, location) => self.visit_variable_expression(name, location),
            Expression::Unary(unary_expr) => self.visit_unary_expression(unary_expr),
            Expression::Call(call_expr) => self.visit_call_expression(call_expr),
            Expression::Conditional(cond_expr) => self.visit_conditional_expression(cond_expr),
        }
    }

    fn visit_call_expression(&mut self, call_expr: &FunctionCallExpr) -> Result<(), String> {
        for arg in &call_expr.arguments {
            self.visit_expression(arg)?;
        }

        let fn_name_idx = self.chunk.add_identifier(call_expr.name.clone());
        self.emit_op(OpCode::GetVariable);
        self.emit_byte(fn_name_idx as u8);

        self.emit_op(OpCode::Call);
        self.emit_byte(call_expr.arguments.len() as u8);

        Ok(())
    }

    fn visit_literal_expression(&mut self, lit_expr: &LiteralExpr) -> Result<(), String> {
        match &lit_expr.value {
            slang_ir::ast::LiteralValue::I32(i) => {
                self.emit_constant(Value::I32(*i));
            }
            slang_ir::ast::LiteralValue::I64(i) => {
                self.emit_constant(Value::I64(*i));
            }
            slang_ir::ast::LiteralValue::U32(i) => {
                self.emit_constant(Value::U32(*i));
            }
            slang_ir::ast::LiteralValue::U64(i) => {
                self.emit_constant(Value::U64(*i));
            }
            slang_ir::ast::LiteralValue::UnspecifiedInteger(i) => {
                self.emit_constant(Value::I64(*i));
            }
            slang_ir::ast::LiteralValue::F32(f) => {
                self.emit_constant(Value::F32(*f));
            }
            slang_ir::ast::LiteralValue::F64(f) => {
                self.emit_constant(Value::F64(*f));
            }
            slang_ir::ast::LiteralValue::UnspecifiedFloat(f) => {
                self.emit_constant(Value::F64(*f));
            }
            slang_ir::ast::LiteralValue::String(s) => {
                self.emit_constant(Value::String(Box::new(s.clone())));
            }
            slang_ir::ast::LiteralValue::Boolean(b) => {
                self.emit_constant(Value::Boolean(*b));
            }
        }

        Ok(())
    }

    fn visit_binary_expression(&mut self, bin_expr: &BinaryExpr) -> Result<(), String> {
        match bin_expr.operator {
            BinaryOperator::And => {
                self.visit_expression(&bin_expr.left)?;
                let jump_if_false = self.emit_jump(OpCode::JumpIfFalse);
                self.emit_op(OpCode::Pop);
                self.visit_expression(&bin_expr.right)?;
                self.patch_jump(jump_if_false);
                return Ok(());
            }

            BinaryOperator::Or => {
                self.visit_expression(&bin_expr.left)?;
                let jump_if_true = self.emit_jump(OpCode::JumpIfFalse);
                let jump_to_end = self.emit_jump(OpCode::Jump);
                self.patch_jump(jump_if_true);
                self.emit_op(OpCode::Pop);
                self.visit_expression(&bin_expr.right)?;
                self.patch_jump(jump_to_end);
                return Ok(());
            }

            _ => {
                self.visit_expression(&bin_expr.left)?;
                self.visit_expression(&bin_expr.right)?;

                match bin_expr.operator {
                    BinaryOperator::Add => self.emit_op(OpCode::Add),
                    BinaryOperator::Subtract => self.emit_op(OpCode::Subtract),
                    BinaryOperator::Multiply => self.emit_op(OpCode::Multiply),
                    BinaryOperator::Divide => self.emit_op(OpCode::Divide),
                    BinaryOperator::GreaterThan => self.emit_op(OpCode::Greater),
                    BinaryOperator::LessThan => self.emit_op(OpCode::Less),
                    BinaryOperator::GreaterThanOrEqual => self.emit_op(OpCode::GreaterEqual),
                    BinaryOperator::LessThanOrEqual => self.emit_op(OpCode::LessEqual),
                    BinaryOperator::Equal => self.emit_op(OpCode::Equal),
                    BinaryOperator::NotEqual => self.emit_op(OpCode::NotEqual),
                    _ => {
                        return Err(format!(
                            "Unsupported binary operator: {:?}",
                            bin_expr.operator
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    fn visit_unary_expression(&mut self, unary_expr: &UnaryExpr) -> Result<(), String> {
        self.visit_expression(&unary_expr.right)?;

        match unary_expr.operator {
            UnaryOperator::Negate => self.emit_op(OpCode::Negate),
            UnaryOperator::Not => self.emit_op(OpCode::BoolNot),
        }

        Ok(())
    }

    fn visit_variable_expression(
        &mut self,
        name: &str,
        _location: &slang_ir::source_location::SourceLocation,
    ) -> Result<(), String> {
        let var_index = self.chunk.add_identifier(name.to_string());
        if var_index > 255 {
            return Err("Too many variables".to_string());
        }
        self.emit_op(OpCode::GetVariable);
        self.emit_byte(var_index as u8);
        Ok(())
    }

    fn visit_type_definition_statement(
        &mut self,
        _stmt: &TypeDefinitionStmt,
    ) -> Result<(), String> {
        // Type definitions don't generate code at runtime
        // They're just for the type checker
        Ok(())
    }

    fn visit_conditional_expression(&mut self, cond_expr: &ConditionalExpr) -> Result<(), String> {
        self.visit_expression(&cond_expr.condition)?;
        
        let jump_to_else = self.emit_jump(OpCode::JumpIfFalse);
        self.emit_op(OpCode::Pop); // Pop the condition value
        self.visit_expression(&cond_expr.then_branch)?;
        
        let jump_over_else = self.emit_jump(OpCode::Jump);
        self.patch_jump(jump_to_else);
        self.emit_op(OpCode::Pop); // Pop the condition value
        self.visit_expression(&cond_expr.else_branch)?;
        
        self.patch_jump(jump_over_else);
        
        Ok(())
    }

    fn visit_if_statement(&mut self, if_stmt: &IfStatement) -> Result<(), String> {
        self.visit_expression(&if_stmt.condition)?;
        
        let jump_to_else = self.emit_jump(OpCode::JumpIfFalse);
        self.emit_op(OpCode::Pop); 
        
        self.visit_block_statement(&if_stmt.then_branch)?;
        
        if let Some(else_branch) = &if_stmt.else_branch {
            let jump_over_else = self.emit_jump(OpCode::Jump);
            
            self.patch_jump(jump_to_else);
            self.emit_op(OpCode::Pop); 
            
            self.visit_block_statement(else_branch)?;
            
            self.patch_jump(jump_over_else);
        } else {
            self.patch_jump(jump_to_else);
            self.emit_op(OpCode::Pop); 
        }
        
        Ok(())
    }
}
