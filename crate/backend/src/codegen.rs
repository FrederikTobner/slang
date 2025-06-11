use crate::bytecode::{Chunk, Function, OpCode};
use crate::value::Value;
use slang_error::{CompilerError, CompileResult, ErrorCode};
use slang_ir::Visitor;
use slang_ir::ast::{
    BinaryExpr, BinaryOperator, BlockExpr, ConditionalExpr, Expression, FunctionCallExpr,
    FunctionDeclarationStmt, FunctionTypeExpr, IfStatement, LetStatement, LiteralExpr, Statement, TypeDefinitionStmt,
    UnaryExpr, UnaryOperator,
};
use slang_ir::location::Location;

/// Compiles AST nodes into bytecode instructions
pub struct CodeGenerator {
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
    /// Accumulated errors during compilation
    errors: Vec<CompilerError>,
}

pub fn generate_bytecode(statements: &[Statement]) -> CompileResult<Chunk> {
    let compiler = CodeGenerator::new();
    compiler.compile(statements)
}

impl CodeGenerator {
    /// Creates a new compiler with an empty chunk
    pub fn new() -> Self {
        CodeGenerator {
            chunk: Chunk::new(),
            line: 1,
            variables: Vec::new(),
            functions: Vec::new(),
            local_scopes: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Updates the current line from a source location
    fn set_current_location(&mut self, location: &Location) {
        self.line = location.line;
    }

    /// Creates a CompilerError with the current location and adds it to the error list
    fn add_error(&mut self, message: String) {
        let error = CompilerError::new(
            ErrorCode::GenericCompileError,
            message,
            self.line,
            0, // column - we don't track this in codegen currently
            0, // position - we don't track this in codegen currently  
            None, // token_length - not applicable for codegen errors
        );
        self.errors.push(error);
    }

    /// Compiles a list of statements into bytecode
    ///
    /// ### Arguments
    ///
    /// * `statements` - The statements to compile
    ///
    /// ### Returns
    ///
    /// A CompileResult containing the compiled bytecode chunk or errors
    fn compile(mut self, statements: &[Statement]) -> CompileResult<Chunk> {
        for stmt in statements {
            stmt.accept(&mut self).unwrap_or_else(|_| {
                // Error already added to self.errors
            });
        }

        self.emit_op(OpCode::Return);

        if self.errors.is_empty() {
            Ok(self.chunk)
        } else {
            Err(self.errors)
        }
    }

    /// Compiles statements and appends them to the existing chunk
    ///
    /// ### Arguments
    ///
    /// * `statements` - The statements to compile
    ///
    /// ### Returns
    ///
    /// CompileResult indicating success or containing errors
    pub fn compile_statements(&mut self, statements: &[Statement]) -> CompileResult<()> {
        for stmt in statements {
            stmt.accept(self).unwrap_or_else(|_| {
                // Error already added to self.errors
            });
        }
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Gets a reference to the current chunk
    pub fn get_chunk(&self) -> &Chunk {
        &self.chunk
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
    }    /// Adds a constant value to the chunk and emits code to load it
    ///
    /// ### Arguments
    ///
    /// * `value` - The constant value to add
    fn emit_constant(&mut self, value: Value) -> Result<(), ()> {
        let constant_index = self.chunk.add_constant(value);
        if constant_index > 255 {
            self.add_error("Too many constants in one chunk".to_string());
            return Err(());
        }
        self.emit_op(OpCode::Constant);
        self.emit_byte(constant_index as u8);
        Ok(())
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
        self.emit_op(OpCode::BeginScope);
    }

    fn end_scope(&mut self) {
        self.local_scopes.pop();
        self.emit_op(OpCode::EndScope);
    }
}

impl Visitor<Result<(), ()>> for CodeGenerator {
    fn visit_statement(&mut self, stmt: &Statement) -> Result<(), ()> {
        // Update current line from the statement's location
        let location = match stmt {
            Statement::Let(let_stmt) => let_stmt.location,
            Statement::Assignment(assign_stmt) => assign_stmt.location,
            Statement::TypeDefinition(type_stmt) => type_stmt.location,
            Statement::Expression(expr) => expr.location(),
            Statement::FunctionDeclaration(fn_decl) => fn_decl.location,
            Statement::Return(return_stmt) => return_stmt.location,
            Statement::If(if_stmt) => if_stmt.location,
        };
        self.set_current_location(&location);
        
        match stmt {
            Statement::Let(let_stmt) => self.visit_let_statement(let_stmt),
            Statement::Assignment(assign_stmt) => self.visit_assignment_statement(assign_stmt),
            Statement::TypeDefinition(type_stmt) => self.visit_type_definition_statement(type_stmt),
            Statement::Expression(expr) => self.visit_expression_statement(expr),
            Statement::FunctionDeclaration(fn_decl) => {
                self.visit_function_declaration_statement(fn_decl)
            }
            Statement::Return(expr) => self.visit_return_statement(expr),
            Statement::If(if_stmt) => self.visit_if_statement(if_stmt),
        }
    }

    fn visit_expression(&mut self, expr: &Expression) -> Result<(), ()> {
        // Update current line from the expression's location
        self.set_current_location(&expr.location());
        
        match expr {
            Expression::Literal(lit_expr) => self.visit_literal_expression(lit_expr),
            Expression::Binary(bin_expr) => self.visit_binary_expression(bin_expr),
            Expression::Variable(var) => self.visit_variable_expression(var),
            Expression::Unary(unary_expr) => self.visit_unary_expression(unary_expr),
            Expression::Call(call_expr) => self.visit_call_expression(call_expr),
            Expression::Conditional(cond_expr) => self.visit_conditional_expression(cond_expr),
            Expression::Block(block_expr) => self.visit_block_expression(block_expr),
            Expression::FunctionType(func_type_expr) => self.visit_function_type_expression(func_type_expr),
        }
    }

    fn visit_function_declaration_statement(
        &mut self,
        fn_decl: &FunctionDeclarationStmt,
    ) -> Result<(), ()> {
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

        for stmt in &fn_decl.body.statements {
            stmt.accept(self)?;
        }

        if let Some(return_expr) = &fn_decl.body.return_expr {
            return_expr.accept(self)?;
        } else {
            self.emit_constant(Value::Unit(()))?;
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

    fn visit_return_statement(&mut self, return_stmt: &slang_ir::ast::ReturnStatement) -> Result<(), ()> {
        if let Some(expr) = &return_stmt.value {
            self.visit_expression(expr)?;
        } else {
            self.emit_constant(Value::Unit(()))?;
        }
        self.emit_op(OpCode::Return);
        Ok(())
    }

    fn visit_expression_statement(&mut self, expr: &Expression) -> Result<(), ()> {
        self.visit_expression(expr)?;
        Ok(())
    }

    fn visit_let_statement(&mut self, let_stmt: &LetStatement) -> Result<(), ()> {
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
            self.add_error("Too many variables in one scope".to_string());
            return Err(());
        }

        self.emit_op(OpCode::SetVariable);
        self.emit_byte(var_index as u8);

        self.emit_op(OpCode::Pop);

        Ok(())
    }

    fn visit_assignment_statement(
        &mut self,
        assign_stmt: &slang_ir::ast::AssignmentStatement,
    ) -> Result<(), ()> {
        self.visit_expression(&assign_stmt.value)?;
        let var_index = self.chunk.add_identifier(assign_stmt.name.clone());
        if var_index > 255 {
            self.add_error("Too many variables in one scope".to_string());
            return Err(());
        }
        self.emit_op(OpCode::SetVariable);
        self.emit_byte(var_index as u8);

        Ok(())
    }

    fn visit_call_expression(&mut self, call_expr: &FunctionCallExpr) -> Result<(), ()> {
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

    fn visit_literal_expression(&mut self, lit_expr: &LiteralExpr) -> Result<(), ()> {
        match &lit_expr.value {
            slang_ir::ast::LiteralValue::I32(i) => {
                self.emit_constant(Value::I32(*i))?;
            }
            slang_ir::ast::LiteralValue::I64(i) => {
                self.emit_constant(Value::I64(*i))?;
            }
            slang_ir::ast::LiteralValue::U32(i) => {
                self.emit_constant(Value::U32(*i))?;
            }
            slang_ir::ast::LiteralValue::U64(i) => {
                self.emit_constant(Value::U64(*i))?;
            }
            slang_ir::ast::LiteralValue::UnspecifiedInteger(i) => {
                self.emit_constant(Value::I64(*i))?;
            }
            slang_ir::ast::LiteralValue::F32(f) => {
                self.emit_constant(Value::F32(*f))?;
            }
            slang_ir::ast::LiteralValue::F64(f) => {
                self.emit_constant(Value::F64(*f))?;
            }
            slang_ir::ast::LiteralValue::UnspecifiedFloat(f) => {
                self.emit_constant(Value::F64(*f))?;
            }
            slang_ir::ast::LiteralValue::String(s) => {
                self.emit_constant(Value::String(Box::new(s.clone())))?;
            }
            slang_ir::ast::LiteralValue::Boolean(b) => {
                self.emit_constant(Value::Boolean(*b))?;
            }
            slang_ir::ast::LiteralValue::Unit => {
                self.emit_constant(Value::Unit(()))?;
            }
        }

        Ok(())
    }

    fn visit_binary_expression(&mut self, bin_expr: &BinaryExpr) -> Result<(), ()> {
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
                        self.add_error(format!(
                            "Unsupported binary operator: {:?}",
                            bin_expr.operator
                        ));
                        return Err(());
                    }
                }
            }
        }

        Ok(())
    }

    fn visit_unary_expression(&mut self, unary_expr: &UnaryExpr) -> Result<(), ()> {
        self.visit_expression(&unary_expr.right)?;

        match unary_expr.operator {
            UnaryOperator::Negate => self.emit_op(OpCode::Negate),
            UnaryOperator::Not => self.emit_op(OpCode::BoolNot),
        }

        Ok(())
    }

    fn visit_variable_expression(
        &mut self,
        var_expr: &slang_ir::ast::VariableExpr,
    ) -> Result<(), ()> {
        let var_index = self.chunk.add_identifier(var_expr.name.clone());
        if var_index > 255 {
            self.add_error("Too many variables".to_string());
            return Err(());
        }
        self.emit_op(OpCode::GetVariable);
        self.emit_byte(var_index as u8);
        Ok(())
    }

    fn visit_type_definition_statement(
        &mut self,
        _stmt: &TypeDefinitionStmt,
    ) -> Result<(), ()> {
        // Type definitions don't generate code at runtime
        // They're just used by the semantic analyzer
        Ok(())
    }

    fn visit_conditional_expression(&mut self, cond_expr: &ConditionalExpr) -> Result<(), ()> {
        self.visit_expression(&cond_expr.condition)?;

        let jump_to_else = self.emit_jump(OpCode::JumpIfFalse);
        self.emit_op(OpCode::Pop);
        self.visit_expression(&cond_expr.then_branch)?;

        let jump_over_else = self.emit_jump(OpCode::Jump);
        self.patch_jump(jump_to_else);
        self.emit_op(OpCode::Pop);
        self.visit_expression(&cond_expr.else_branch)?;

        self.patch_jump(jump_over_else);

        Ok(())
    }

    fn visit_if_statement(&mut self, if_stmt: &IfStatement) -> Result<(), ()> {
        self.visit_expression(&if_stmt.condition)?;

        let jump_to_else = self.emit_jump(OpCode::JumpIfFalse);
        self.emit_op(OpCode::Pop);

        self.visit_block_expression(&if_stmt.then_branch)?;

        if let Some(else_branch) = &if_stmt.else_branch {
            let jump_over_else = self.emit_jump(OpCode::Jump);

            self.patch_jump(jump_to_else);
            self.emit_op(OpCode::Pop);

            self.visit_block_expression(else_branch)?;

            self.patch_jump(jump_over_else);
        } else {
            self.patch_jump(jump_to_else);
            self.emit_op(OpCode::Pop);
        }

        Ok(())
    }

    fn visit_block_expression(&mut self, block_expr: &BlockExpr) -> Result<(), ()> {
        self.begin_scope();

        for stmt in &block_expr.statements {
            self.visit_statement(stmt)?;
        }

        if let Some(return_expr) = &block_expr.return_expr {
            self.visit_expression(return_expr)?;
        } else {
            self.emit_constant(Value::Unit(()))?;
        }

        self.end_scope();

        Ok(())
    }

    fn visit_function_type_expression(&mut self, _func_type_expr: &FunctionTypeExpr) -> Result<(), ()> {
        // Function type expressions are compile-time constructs that don't generate runtime bytecode
        // They are used for type checking and don't produce any values at runtime
        Ok(())
    }
}
