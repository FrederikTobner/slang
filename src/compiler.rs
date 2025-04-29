use crate::ast::{BinaryExpr, Expression, FunctionCallExpr, FunctionDeclarationStmt, LetStatement, LiteralExpr, Statement, TypeDefinitionStmt, UnaryExpr};
use crate::bytecode::{Chunk, Function, OpCode, Value};
use crate::token::Tokentype;
use crate::visitor::Visitor;

/// Compiles AST nodes into bytecode instructions
pub struct Compiler {
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

impl Compiler {
    /// Creates a new compiler with an empty chunk
    pub fn new() -> Self {
        Compiler {
            chunk: Chunk::new(),
            line: 1, 
            variables: Vec::new(),
            functions: Vec::new(),
            local_scopes: Vec::new(),
        }
    }

    /// Compiles a list of statements into bytecode
    /// 
    /// ## Arguments
    /// 
    /// * `statements` - The statements to compile
    /// 
    /// ## Returns
    /// 
    /// A reference to the compiled bytecode chunk, or an error message
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

    /// Emits a single byte to the bytecode chunk
    /// 
    /// # Arguments
    /// 
    /// * `byte` - The byte to emit
    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_byte(byte, self.line);
    }

    /// Emits an opcode to the bytecode chunk
    /// 
    /// # Arguments
    /// 
    /// * `op` - The opcode to emit
    fn emit_op(&mut self, op: OpCode) {
        self.chunk.write_op(op, self.line);
    }

    /// Adds a constant value to the chunk and emits code to load it
    /// 
    /// # Arguments
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
    /// # Arguments
    /// 
    /// * `op` - The jump opcode (Jump or JumpIfFalse)
    /// 
    /// # Returns
    /// 
    /// The position where the jump offset needs to be patched later
    fn emit_jump(&mut self, op: OpCode) -> usize {
        self.emit_op(op);
        // Emit placeholder jump offset (to be patched later)
        self.emit_byte(0xFF);
        self.emit_byte(0xFF);
        self.chunk.code.len() - 2 // Return location to patch
    }
    
    /// Patches a previously emitted jump instruction with the actual offset
    /// 
    /// # Arguments
    /// 
    /// * `offset` - The position of the jump offset to patch
    fn patch_jump(&mut self, offset: usize) {
        // Calculate jump distance
        let jump = self.chunk.code.len() - offset - 2;
        if jump > 0xFFFF {
            panic!("Jump too far");
        }
        
        // Patch the bytecode with the actual jump distance
        self.chunk.code[offset] = ((jump >> 8) & 0xFF) as u8;
        self.chunk.code[offset + 1] = (jump & 0xFF) as u8;
    }
    
    /// Begins a new scope for local variables
    fn begin_scope(&mut self) {
        self.local_scopes.push(Vec::new());
    }
    
    /// Ends the current scope and removes local variables
    fn end_scope(&mut self) {
        if let Some(scope) = self.local_scopes.pop() {
            // Pop all locals from the scope
            for _ in 0..scope.len() {
                self.emit_op(OpCode::Pop);
            }
        }
    }
}

impl Visitor<Result<(), String>> for Compiler {
    fn visit_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Let(let_stmt) => self.visit_let_statement(let_stmt),
            Statement::TypeDefinition(type_stmt) => self.visit_type_definition_statement(type_stmt),
            Statement::Expression(expr) => self.visit_expression_statement(expr),
            Statement::FunctionDeclaration(fn_decl) => self.visit_function_declaration_statement(fn_decl),
            Statement::Block(stmts) => self.visit_block_statement(stmts),
            Statement::Return(expr) => self.visit_return_statement(expr),
        }
    }
    
    fn visit_block_statement(&mut self, stmts: &Vec<Statement>) -> Result<(), String> {
        self.begin_scope();
        
        for stmt in stmts {
            stmt.accept(self)?;
        }
        
        self.end_scope();
        Ok(())
    }
    
    fn visit_function_declaration_statement(&mut self, fn_decl: &FunctionDeclarationStmt) -> Result<(), String> {
        // Register function name
        self.functions.push(fn_decl.name.clone());
        let function_name_idx = self.chunk.add_identifier(fn_decl.name.clone());
        
        // Store the current position where we'll jump over the function body
        let jump_over = self.emit_jump(OpCode::Jump);
        
        // Store function metadata
        let code_offset = self.chunk.code.len();
        let mut locals = Vec::new();
        
        // Push parameters as local variables
        self.begin_scope();
        for param in &fn_decl.parameters {
            locals.push(param.name.clone());
            if let Some(current_scope) = self.local_scopes.last_mut() {
                current_scope.push(param.name.clone());
            }
        }
        
        // Compile function body
        for stmt in &fn_decl.body {
            stmt.accept(self)?;
        }
        
        // Ensure there's a return at the end
        self.emit_op(OpCode::Return);
        
        // End function scope
        self.end_scope();
        
        // Now patch the jump over for the main flow
        self.patch_jump(jump_over);
        
        let function = Value::Function(Function {
            name: fn_decl.name.clone(),
            arity: fn_decl.parameters.len() as u8,
            code_offset,
            locals,
        });
        let fn_constant = self.chunk.add_constant(function);
        
        self.emit_op(OpCode::DefineFunction);
        self.emit_byte(function_name_idx as u8);
        self.emit_byte(fn_constant as u8);
        
        Ok(())
    }
    
    fn visit_return_statement(&mut self, expr: &Option<Expression>) -> Result<(), String> {
        if let Some(expr) = expr {
            // Compile the return value
            self.visit_expression(expr)?;
        } else {
            // If no return value provided, implicitly return a default value like 0
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
        // First register the variable so that even complex initializers can reference it
        // Check if we're in a local scope
        let is_local = !self.local_scopes.is_empty();
        
        if is_local {
            // Add to current scope
            if let Some(current_scope) = self.local_scopes.last_mut() {
                current_scope.push(let_stmt.name.clone());
            }
        } else {
            // Global variable
            self.variables.push(let_stmt.name.clone());
        }

        // Now compile the initialization expression
        self.visit_expression(&let_stmt.value)?;

        let var_index = self.chunk.add_identifier(let_stmt.name.clone());
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
            Expression::Variable(name) => self.visit_variable_expression(name),
            Expression::Unary(unary_expr) => self.visit_unary_expression(unary_expr),
            Expression::Call(call_expr) => self.visit_call_expression(call_expr),
        }
    }
    
    fn visit_call_expression(&mut self, call_expr: &FunctionCallExpr) -> Result<(), String> {
        // First compile arguments (in reverse order)
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
            crate::ast::Value::I32(i) => {
                self.emit_constant(Value::I32(*i));
            }
            crate::ast::Value::I64(i) => {
                self.emit_constant(Value::I64(*i));
            }
            crate::ast::Value::U32(i) => {
                self.emit_constant(Value::U32(*i));
            }
            crate::ast::Value::U64(i) => {
                self.emit_constant(Value::U64(*i));
            }
            crate::ast::Value::UnspecifiedInteger(i) => {
                // Unspecified integers default to i64 in the VM
                self.emit_constant(Value::I64(*i));
            }
            crate::ast::Value::F32(f) => {
                self.emit_constant(Value::F32(*f));
            }
            crate::ast::Value::F64(f) => {
                self.emit_constant(Value::F64(*f));
            }
            crate::ast::Value::UnspecifiedFloat(f) => {
                // Unspecified floats default to f64 in the VM
                self.emit_constant(Value::F64(*f));
            }
            crate::ast::Value::String(s) => {
                self.emit_constant(Value::String(s.clone()));
            }
            crate::ast::Value::Boolean(b) => {
                self.emit_constant(Value::Boolean(*b));
            }
        }

        Ok(())
    }

    fn visit_binary_expression(&mut self, bin_expr: &BinaryExpr) -> Result<(), String> {
        match bin_expr.operator {
            // Special handling for && (AND) operator with short-circuit evaluation
            Tokentype::And => {
                // Evaluate the left operand
                self.visit_expression(&bin_expr.left)?;
                
                // If left is false, short-circuit (jump over right operand evaluation)
                let jump_if_false = self.emit_jump(OpCode::JumpIfFalse);
                
                // Pop the left value since we don't need it anymore if we didn't jump
                self.emit_op(OpCode::Pop);
                
                // Evaluate the right operand (only reached if left was true)
                self.visit_expression(&bin_expr.right)?;
                
                // Patch the jump to this point
                self.patch_jump(jump_if_false);
                
                return Ok(());
            },
            
            // Special handling for || (OR) operator with short-circuit evaluation
            Tokentype::Or => {
                // Evaluate the left operand
                self.visit_expression(&bin_expr.left)?;
                
                // If left is true, short-circuit (skip right operand)
                let jump_if_true = self.emit_jump(OpCode::JumpIfFalse);
                let jump_to_end = self.emit_jump(OpCode::Jump);
                
                // Patch jump_if_true to jump to here
                self.patch_jump(jump_if_true);
                
                // Pop the left value since we don't need it anymore
                self.emit_op(OpCode::Pop);
                
                // Evaluate right operand (only reached if left was false)
                self.visit_expression(&bin_expr.right)?;
                
                // Patch jump_to_end
                self.patch_jump(jump_to_end);
                
                return Ok(());
            },
            
            // Handle regular arithmetic and comparison operators
            _ => {
                self.visit_expression(&bin_expr.left)?;
                self.visit_expression(&bin_expr.right)?;
                
                match bin_expr.operator {
                    Tokentype::Plus => self.emit_op(OpCode::Add),
                    Tokentype::Minus => self.emit_op(OpCode::Subtract),
                    Tokentype::Multiply => self.emit_op(OpCode::Multiply),
                    Tokentype::Divide => self.emit_op(OpCode::Divide),
                    // Relational operators
                    Tokentype::Greater => self.emit_op(OpCode::Greater),
                    Tokentype::Less => self.emit_op(OpCode::Less),
                    Tokentype::GreaterEqual => self.emit_op(OpCode::GreaterEqual),
                    Tokentype::LessEqual => self.emit_op(OpCode::LessEqual),
                    Tokentype::EqualEqual => self.emit_op(OpCode::Equal),
                    Tokentype::NotEqual => self.emit_op(OpCode::NotEqual),
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
            Tokentype::Minus => self.emit_op(OpCode::Negate),
            Tokentype::Not => self.emit_op(OpCode::BoolNot),
            _ => return Err(format!("Unsupported unary operator: {:?}", unary_expr.operator)),
        }
        
        Ok(())
    }
    
    fn visit_variable_expression(&mut self, name: &str) -> Result<(), String> {
        let var_index = self.chunk.add_identifier(name.to_string());
        if var_index > 255 {
            return Err("Too many variables".to_string());
        }

        self.emit_op(OpCode::GetVariable);
        self.emit_byte(var_index as u8);

        Ok(())
    }

    fn visit_type_definition_statement(&mut self, _stmt: &TypeDefinitionStmt) -> Result<(), String> {
        // Type definitions don't generate code at runtime
        // They're just for the type checker
        Ok(())
    }
}
