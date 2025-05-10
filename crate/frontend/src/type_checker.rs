use slang_ir::ast::{
    BinaryExpr, Expression, FunctionCallExpr, FunctionDeclarationStmt, LetStatement, LiteralExpr,
    LiteralValue, Statement, TypeDefinitionStmt, UnaryExpr, UnaryOperator, BinaryOperator,
};
use slang_types::types::*;
use slang_types::types::{StructType, TYPE_REGISTRY, TypeId, TypeKind, get_type_name, type_fullfills};
use slang_ir::visitor::Visitor;
use std::collections::HashMap;
use crate::error::{CompilerError, CompileResult};

struct Scope {
    variables: HashMap<String, TypeId>,
}

impl Scope {
    fn new() -> Self {
        Scope {
            variables: HashMap::new(),
        }
    }
}

pub fn execute(statements: &[Statement]) -> CompileResult<()> {
    let mut type_checker = TypeChecker::new();
    type_checker.check(statements)
}

/// Performs static type checking on the AST
pub struct TypeChecker {
    /// Map of variable names to their types
    scopes: Vec<Scope>,
    /// Map of function names to their parameter and return types
    functions: HashMap<String, (Vec<TypeId>, TypeId)>,
    /// Current function's return type for validating return statements
    current_return_type: Option<TypeId>,
    /// Collected type errors
    errors: Vec<CompilerError>,
}

impl TypeChecker {
    /// Creates a new type checker with built-in functions registered
    pub fn new() -> Self {
        let mut tc = TypeChecker {
            scopes: vec![Scope::new()],
            functions: HashMap::new(),
            current_return_type: None,
            errors: Vec::new(),
        };
        tc.register_native_functions();
        tc
    }

    // Add an error to the collection
    fn add_error(&mut self, message: String, line: usize, column: usize) {
        self.errors.push(CompilerError::new(message, line, column));
    }

    // Convert a String error to a CompilerError and add it to the collection
    fn add_string_error(&mut self, message: String) {
        // Since we don't have line/column info in the current error handling,
        // temporarily use 0,0 - this will be improved in a future update
        self.add_error(message, 0, 0);
    }

    /// Begins a new scope
    fn begin_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    /// Ends the current scope
    /// Panics if trying to end the global scope
    fn end_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        } else {
            panic!("Cannot end the global scope");
        }
    }

    /// Defines a variable in the current scope
    ///
    /// ### Arguments
    /// name - The name of the variable
    /// type_id - The type ID of the variable
    fn define_variable(&mut self, name: String, type_id: TypeId) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.variables.insert(name, type_id);
        }
    }

    /// Looks up a variable in all scopes, starting from innermost
    ///
    /// ### Arguments
    /// name - The name of the variable to look up
    ///
    /// ### Returns
    /// The type ID of the variable if found, or None if not found
    fn resolve_variable(&self, name: &str) -> Option<TypeId> {
        // Search from innermost (last) to outermost (first) scope
        for scope in self.scopes.iter().rev() {
            if let Some(type_id) = scope.variables.get(name) {
                return Some(type_id.clone());
            }
        }
        None
    }

    /// Checks if a type is an integer type
    ///
    /// ## Arguments
    /// type_id - The type ID to check
    ///
    /// ## Returns
    /// True if the type is an integer type, false otherwise
    ///
    fn is_integer_type(&self, type_id: &TypeId) -> bool {
        type_fullfills(type_id, |info| matches!(info.kind, TypeKind::Integer(_)))
    }

    /// Checks if a type is a float type
    ///
    /// ## Arguments
    /// type_id - The type ID to check
    ///
    /// ## Returns
    /// True if the type is a float type, false otherwise
    ///
    fn is_float_type(&self, type_id: &TypeId) -> bool {
        type_fullfills(type_id, |info| matches!(info.kind, TypeKind::Float(_)))
    }

    /// Registers the built-in native functions
    fn register_native_functions(&mut self) {
        self.functions.insert(
            "print_value".to_string(),
            (vec![unknown_type()], i32_type()),
        );
    }

    /// Checks the type safety of a list of statements
    ///
    /// # Arguments
    ///
    /// * `statements` - The statements to check
    ///
    /// # Returns
    ///
    /// CompileResult with () if type-safe, or a list of errors
    pub fn check(&mut self, statements: &[Statement]) -> CompileResult<()> {
        for stmt in statements {
            match stmt.accept(self) {
                Ok(_) => continue,
                Err(e) => self.add_string_error(e),
            }
        }
        
        if !self.errors.is_empty() {
            Err(std::mem::take(&mut self.errors))
        } else {
            Ok(())
        }
    }
}

impl Visitor<Result<TypeId, String>> for TypeChecker {
    fn visit_statement(&mut self, stmt: &Statement) -> Result<TypeId, String> {
        match stmt {
            Statement::Let(let_stmt) => self.visit_let_statement(let_stmt),
            Statement::Expression(expr) => self.visit_expression_statement(expr),
            Statement::TypeDefinition(type_def) => self.visit_type_definition_statement(type_def),
            Statement::FunctionDeclaration(fn_decl) => {
                self.visit_function_declaration_statement(fn_decl)
            }

            Statement::Block(stmts) => self.visit_block_statement(stmts),
            Statement::Return(expr) => self.visit_return_statement(expr),
        }
    }

    fn visit_function_declaration_statement(
        &mut self,
        fn_decl: &FunctionDeclarationStmt,
    ) -> Result<TypeId, String> {
        let mut param_types = Vec::new();
        for param in &fn_decl.parameters {
            param_types.push(param.param_type.clone());
        }
        self.functions.insert(
            fn_decl.name.clone(),
            (param_types, fn_decl.return_type.clone()),
        );
        let previous_return_type = self.current_return_type.clone();
        self.current_return_type = Some(fn_decl.return_type.clone());

        self.begin_scope();
        for param in &fn_decl.parameters {
            self.define_variable(param.name.clone(), param.param_type.clone());
        }

        let result = self.visit_block_statement(&fn_decl.body);

        self.current_return_type = previous_return_type;
        self.end_scope();
        result.and(Ok(fn_decl.return_type.clone()))
    }

    fn visit_block_statement(&mut self, stmts: &[Statement]) -> Result<TypeId, String> {
        self.begin_scope();
        for stmt in stmts {
            stmt.accept(self)?;
        }

        self.end_scope();
        Ok(unknown_type())
    }

    fn visit_return_statement(&mut self, expr: &Option<Expression>) -> Result<TypeId, String> {
        // Check if we're inside a function
        if let Some(expected_type) = &self.current_return_type {
            let expected_type = expected_type.clone();
            // Check return value
            if let Some(expr) = expr {
                let actual_type = self.visit_expression(expr)?;

                // Check if types match
                if actual_type == expected_type {
                    return Ok(actual_type);
                }

                // Handle special case for unspecified integers
                if actual_type == unspecified_int_type() {
                    if let Expression::Literal(lit) = expr {
                        if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                            let value_in_range = TYPE_REGISTRY.with(|registry| {
                                registry.borrow().check_value_in_range(n, &expected_type)
                            });

                            if value_in_range {
                                return Ok(expected_type.clone());
                            }
                        }
                    }
                }

                // Handle special case for unspecified floats
                if actual_type == unspecified_float_type() {
                    if let Expression::Literal(lit) = expr {
                        if let LiteralValue::UnspecifiedFloat(f) = &lit.value {
                            let value_in_range = TYPE_REGISTRY.with(|registry| {
                                registry
                                    .borrow()
                                    .check_float_value_in_range(f, &expected_type)
                            });

                            if value_in_range {
                                return Ok(expected_type.clone());
                            }
                        }
                    }
                }
                return Err(format!(
                    "Type mismatch: function returns {} but got {}",
                    get_type_name(&expected_type),
                    get_type_name(&actual_type)
                ));
            } else if expected_type != unknown_type() {
                // Missing return value when one is expected
                return Err(format!(
                    "Type mismatch: function returns {} but no return value provided",
                    get_type_name(&expected_type)
                ));
            }

            Ok(expected_type)
        } else {
            Err("Return statement outside of function".to_string())
        }
    }

    fn visit_call_expression(&mut self, call_expr: &FunctionCallExpr) -> Result<TypeId, String> {
        // Check if function exists
        if let Some((param_types, return_type)) = self.functions.get(&call_expr.name).cloned() {
            // Check argument count
            if param_types.len() != call_expr.arguments.len() {
                return Err(format!(
                    "Function '{}' expects {} arguments, but got {}",
                    call_expr.name,
                    param_types.len(),
                    call_expr.arguments.len()
                ));
            }

            // Check each argument
            for (i, arg) in call_expr.arguments.iter().enumerate() {
                let arg_type = self.visit_expression(arg)?;
                let param_type = &param_types[i];

                // function that can accept any type(s)
                if param_type == &unknown_type() {
                    continue;
                }

                // Check if types match
                if arg_type != *param_type {
                    // Handle special case for unspecified integers
                    if arg_type == unspecified_int_type() {
                        if let Expression::Literal(lit) = arg {
                            if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                                let value_in_range = TYPE_REGISTRY.with(|registry| {
                                    registry.borrow().check_value_in_range(n, param_type)
                                });

                                if value_in_range {
                                    continue; // This argument is valid
                                }
                            }
                        }
                    }

                    return Err(format!(
                        "Type mismatch: function '{}' expects argument {} to be {}, but got {}",
                        call_expr.name,
                        i + 1,
                        get_type_name(param_type),
                        get_type_name(&arg_type)
                    ));
                }
            }

            // All arguments match, return the function's return type
            Ok(return_type)
        } else {
            Err(format!("Undefined function: {}", call_expr.name))
        }
    }

    fn visit_type_definition_statement(
        &mut self,
        type_def: &TypeDefinitionStmt,
    ) -> Result<TypeId, String> {
        // Register the new struct type
        TYPE_REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();

            // Create a new struct type
            let type_kind = TypeKind::Struct(StructType {
                name: type_def.name.clone(),
                fields: type_def.fields.clone(),
            });

            let type_id = registry.register_type(&type_def.name, type_kind);
            Ok(type_id)
        })
    }

    fn visit_expression_statement(&mut self, expr: &Expression) -> Result<TypeId, String> {
        self.visit_expression(expr)
    }

    fn visit_let_statement(&mut self, let_stmt: &LetStatement) -> Result<TypeId, String> {
        // Register the variable with a placeholder type first
        let placeholder_type = let_stmt.expr_type.clone();

        if self
            .scopes
            .last()
            .unwrap()
            .variables
            .contains_key(&let_stmt.name)
        {
            return Err(format!("Variable '{}' already defined", let_stmt.name));
        }

        // Add to symbol table with the placeholder type
        self.define_variable(let_stmt.name.clone(), placeholder_type);

        // Now process the initialization expression
        let expr_type = self.visit_expression(&let_stmt.value)?;

        // If type wasn't specified, infer it
        let mut final_type = if let_stmt.expr_type == unknown_type() {
            expr_type
        } else if let_stmt.expr_type != expr_type {
            // Only allow UnspecifiedInteger to be assigned to specific integer types
            // with value range check
            if expr_type == unspecified_int_type() {
                // Check for both direct literals and literals inside unary expressions
                match &let_stmt.value {
                    Expression::Literal(lit) => {
                        if self.is_integer_type(&let_stmt.expr_type) {
                            if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                                // Check if the value is in range for the target type
                                let value_in_range = TYPE_REGISTRY.with(|registry| {
                                    registry
                                        .borrow()
                                        .check_value_in_range(n, &let_stmt.expr_type)
                                });

                                if value_in_range {
                                    let_stmt.expr_type.clone()
                                } else {
                                    return Err(format!(
                                        "Integer literal {} is out of range for type {}",
                                        n,
                                        get_type_name(&let_stmt.expr_type)
                                    ));
                                }
                            } else {
                                // Non-integer literals can't be assigned to integer types
                                return Err(format!(
                                    "Type mismatch: variable {} is {} but expression is {}",
                                    let_stmt.name,
                                    get_type_name(&let_stmt.expr_type),
                                    get_type_name(&expr_type)
                                ));
                            }
                        } else {
                            // integer literals can't be assigned to non-integer types
                            return Err(format!(
                                "Type mismatch: variable {} is {} but expression is {}",
                                let_stmt.name,
                                get_type_name(&let_stmt.expr_type),
                                get_type_name(&expr_type)
                            ));
                        }
                    }
                    Expression::Unary(unary_expr) => {
                        if unary_expr.operator == UnaryOperator::Negate {
                            // Handle negated integer literals
                            if let Expression::Literal(lit) = &*unary_expr.right {
                                if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                                    // For negation, we need to check the negative value
                                    let negated_value = -*n;
                                    let value_in_range = TYPE_REGISTRY.with(|registry| {
                                        registry.borrow().check_value_in_range(
                                            &negated_value,
                                            &let_stmt.expr_type,
                                        )
                                    });

                                    if value_in_range {
                                        return Ok(let_stmt.expr_type.clone());
                                    } else {
                                        return Err(format!(
                                            "Integer literal {} is out of range for type {}",
                                            negated_value,
                                            get_type_name(&let_stmt.expr_type)
                                        ));
                                    }
                                }
                            }
                        }

                        // Non-literal expressions can't be assigned if types don't match exactly
                        return Err(format!(
                            "Type mismatch: variable {} is {} but expression is {}",
                            let_stmt.name,
                            get_type_name(&let_stmt.expr_type),
                            get_type_name(&expr_type)
                        ));
                    }
                    Expression::Binary(_) => {
                        // Handle binary expressions
                        if self.is_integer_type(&let_stmt.expr_type) {
                            return Ok(let_stmt.expr_type.clone());
                        }

                        // Non-literal expressions can't be assigned if types don't match exactly
                        return Err(format!(
                            "Type mismatch: variable {} is {} but expression is {}",
                            let_stmt.name,
                            get_type_name(&let_stmt.expr_type),
                            get_type_name(&expr_type)
                        ));
                    }
                    _ => {
                        // Non-literal expressions can't be assigned if types don't match exactly
                        return Err(format!(
                            "Type mismatch: variable {} is {} but expression is {}",
                            let_stmt.name,
                            get_type_name(&let_stmt.expr_type),
                            get_type_name(&expr_type)
                        ));
                    }
                }
            } else if expr_type == unspecified_float_type() {
                // Handle unspecified float literals
                match &let_stmt.value {
                    Expression::Literal(lit) => {
                        if let LiteralValue::UnspecifiedFloat(f) = &lit.value {
                            if self.is_float_type(&let_stmt.expr_type) {
                                // Check if the float is in range for the target type
                                let value_in_range = TYPE_REGISTRY.with(|registry| {
                                    registry
                                        .borrow()
                                        .check_float_value_in_range(f, &let_stmt.expr_type)
                                });

                                if value_in_range {
                                    let_stmt.expr_type.clone()
                                } else {
                                    return Err(format!(
                                        "Float literal {} is out of range for type {}",
                                        f,
                                        get_type_name(&let_stmt.expr_type)
                                    ));
                                }
                            } else {
                                // Non-float types can't be assigned float literals
                                return Err(format!(
                                    "Type mismatch: variable {} is {} but expression is {}",
                                    let_stmt.name,
                                    get_type_name(&let_stmt.expr_type),
                                    get_type_name(&expr_type)
                                ));
                            }
                        } else {
                            return Err(format!(
                                "Type mismatch: variable {} is {} but expression is {}",
                                let_stmt.name,
                                get_type_name(&let_stmt.expr_type),
                                get_type_name(&expr_type)
                            ));
                        }
                    }
                    Expression::Unary(unary_expr) => {
                        if unary_expr.operator == UnaryOperator::Negate {
                            // Handle negated float literals
                            if let Expression::Literal(lit) = &*unary_expr.right {
                                if let LiteralValue::UnspecifiedFloat(f) = &lit.value {
                                    if self.is_float_type(&let_stmt.expr_type) {
                                        // For negation, we need to check the negative value
                                        let negated_value = -*f;
                                        let value_in_range = TYPE_REGISTRY.with(|registry| {
                                            registry.borrow().check_float_value_in_range(
                                                &negated_value,
                                                &let_stmt.expr_type,
                                            )
                                        });

                                        if value_in_range {
                                            return Ok(let_stmt.expr_type.clone());
                                        } else {
                                            return Err(format!(
                                                "Float literal {} is out of range for type {}",
                                                negated_value,
                                                get_type_name(&let_stmt.expr_type)
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        return Err(format!(
                            "Type mismatch: variable {} is {} but expression is {}",
                            let_stmt.name,
                            get_type_name(&let_stmt.expr_type),
                            get_type_name(&expr_type)
                        ));
                    }
                    Expression::Binary(_) => {
                        // Handle binary expressions
                        if self.is_float_type(&let_stmt.expr_type) {
                            return Ok(let_stmt.expr_type.clone());
                        }
                        return Err(format!(
                            "Type mismatch: variable {} is {} but expression is {}",
                            let_stmt.name,
                            get_type_name(&let_stmt.expr_type),
                            get_type_name(&expr_type)
                        ));
                    }
                    _ => {
                        return Err(format!(
                            "Type mismatch: variable {} is {} but expression is {}",
                            let_stmt.name,
                            get_type_name(&let_stmt.expr_type),
                            get_type_name(&expr_type)
                        ));
                    }
                }
            } else {
                // Types don't match and no coercion is allowed
                let expr_type_name = get_type_name(&expr_type);
                let target_type_name = get_type_name(&let_stmt.expr_type);

                return Err(format!(
                    "Type mismatch: variable {} is {} but expression is {}",
                    let_stmt.name, target_type_name, expr_type_name
                ));
            }
        } else {
            let_stmt.expr_type.clone()
        };
        if final_type == unspecified_int_type() {
            final_type = i64_type();
        } else if final_type == unspecified_float_type() {
            final_type = f64_type();
        }
        // Update symbol table with the final type
        self.define_variable(let_stmt.name.clone(), final_type.clone());
        Ok(final_type)
    }

    fn visit_expression(&mut self, expr: &Expression) -> Result<TypeId, String> {
        match expr {
            Expression::Binary(bin_expr) => self.visit_binary_expression(bin_expr),
            Expression::Literal(lit_expr) => self.visit_literal_expression(lit_expr),
            Expression::Variable(name) => self.visit_variable_expression(name),
            Expression::Unary(unary_expr) => self.visit_unary_expression(unary_expr),
            Expression::Call(call_expr) => self.visit_call_expression(call_expr),
        }
    }

    fn visit_literal_expression(&mut self, lit_expr: &LiteralExpr) -> Result<TypeId, String> {
        // Infer type from literal
        match lit_expr.value {
            LiteralValue::I32(_) => Ok(i32_type()),
            LiteralValue::I64(_) => Ok(i64_type()),
            LiteralValue::U32(_) => Ok(u32_type()),
            LiteralValue::U64(_) => Ok(u64_type()),
            LiteralValue::F32(_) => Ok(f32_type()),
            LiteralValue::F64(_) => Ok(f64_type()),
            LiteralValue::UnspecifiedInteger(_) => Ok(unspecified_int_type()),
            LiteralValue::UnspecifiedFloat(_) => Ok(unspecified_float_type()),
            LiteralValue::String(_) => Ok(string_type()),
            LiteralValue::Boolean(_) => Ok(bool_type()),
        }
    }

    fn visit_binary_expression(&mut self, bin_expr: &BinaryExpr) -> Result<TypeId, String> {
        let left_type = self.visit_expression(&bin_expr.left)?;
        let right_type = self.visit_expression(&bin_expr.right)?;

        // Handle logical operators (AND, OR)
        if bin_expr.operator == BinaryOperator::And || bin_expr.operator == BinaryOperator::Or {
            // Check that both operands are boolean
            if left_type == bool_type() && right_type == bool_type() {
                return Ok(bool_type());
            } else {
                return Err(format!(
                    "Logical operator '{}' requires boolean operands, got {} and {}",
                    bin_expr.operator,
                    get_type_name(&left_type),
                    get_type_name(&right_type)
                ));
            }
        }

        // Handle relational operators (>, <, >=, <=, ==, !=)
        if matches!(
            bin_expr.operator,
            BinaryOperator::GreaterThan
                | BinaryOperator::LessThan
                | BinaryOperator::GreaterThanOrEqual
                | BinaryOperator::LessThanOrEqual
                | BinaryOperator::Equal
                | BinaryOperator::NotEqual
        ) {
            // For now, only allow comparing same types (or unspecified literals with specific types)
            if left_type == right_type
                || (left_type == unspecified_int_type() && self.is_integer_type(&right_type))
                || (right_type == unspecified_int_type() && self.is_integer_type(&left_type))
                || (left_type == unspecified_float_type() && self.is_float_type(&right_type))
                || (right_type == unspecified_float_type() && self.is_float_type(&left_type))
            {
                return Ok(bool_type());
            }
            return Err(format!(
                "Cannot compare different types with {}: {} and {}",
                bin_expr.operator,
                get_type_name(&left_type),
                get_type_name(&right_type)
            ));
        }

        // Handle arithmetic operations using the type registry
        if matches!(
            bin_expr.operator,
            BinaryOperator::Add | BinaryOperator::Subtract | BinaryOperator::Multiply | BinaryOperator::Divide
        ) {
            // Only allow operations between same types, with special handling for unspecified literals
            if left_type == right_type {
                if left_type == bool_type()
                    || (bin_expr.operator != BinaryOperator::Add && left_type == string_type())
                {
                    return Err(format!(
                        "Type mismatch: cannot apply '{}' operator on {} and {}",
                        bin_expr.operator,
                        get_type_name(&left_type),
                        get_type_name(&right_type)
                    ));
                }
                return Ok(left_type);
            }

            // Special case for unspecified integers
            if left_type == unspecified_int_type() && self.is_integer_type(&right_type) {
                // Check if the unspecified integer literal value is in range
                if let Expression::Literal(lit) = &*bin_expr.left {
                    if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                        // Check if value is in valid range for the target type
                        let value_in_range = TYPE_REGISTRY.with(|registry| {
                            registry.borrow().check_value_in_range(n, &right_type)
                        });

                        if value_in_range {
                            return Ok(right_type);
                        } else {
                            let right_type_name = get_type_name(&right_type);

                            return Err(format!(
                                "Integer literal {} is out of range for type {}",
                                n, right_type_name
                            ));
                        }
                    }
                }
                return Ok(right_type);
            }

            if right_type == unspecified_int_type() && self.is_integer_type(&left_type) {
                // Similar check for right side
                if let Expression::Literal(literal) = &*bin_expr.right {
                    if let LiteralValue::UnspecifiedInteger(num) = &literal.value {
                        let value_in_range = TYPE_REGISTRY.with(|registry| {
                            registry.borrow().check_value_in_range(num, &left_type)
                        });

                        if value_in_range {
                            return Ok(left_type);
                        } else {
                            let left_type_name = get_type_name(&left_type);

                            return Err(format!(
                                "Integer literal {} is out of range for type {}",
                                num, left_type_name
                            ));
                        }
                    }
                }
                return Ok(left_type);
            }

            // Special case for unspecified floats
            if left_type == unspecified_float_type() && self.is_float_type(&right_type) {
                // Check if the unspecified float literal value is in range
                if let Expression::Literal(lit) = &*bin_expr.left {
                    if let LiteralValue::UnspecifiedFloat(f) = &lit.value {
                        // Check if value is in valid range for the target type
                        let value_in_range = TYPE_REGISTRY.with(|registry| {
                            registry.borrow().check_float_value_in_range(f, &right_type)
                        });

                        if value_in_range {
                            return Ok(right_type);
                        } else {
                            return Err(format!(
                                "Float literal {} is out of range for type {}",
                                f,
                                get_type_name(&right_type)
                            ));
                        }
                    }
                }
                return Ok(right_type);
            }

            if right_type == unspecified_float_type() && self.is_float_type(&left_type) {
                // Similar check for right side with float literals
                if let Expression::Literal(lit) = &*bin_expr.right {
                    if let LiteralValue::UnspecifiedFloat(f) = &lit.value {
                        let value_in_range = TYPE_REGISTRY.with(|registry| {
                            registry.borrow().check_float_value_in_range(f, &left_type)
                        });

                        if value_in_range {
                            return Ok(left_type);
                        } else {
                            return Err(format!(
                                "Float literal {} is out of range for type {}",
                                f,
                                get_type_name(&left_type)
                            ));
                        }
                    }
                }
                return Ok(left_type);
            }
        }

        // String concatenation - still allowed
        if bin_expr.operator == BinaryOperator::Add
            && left_type == string_type()
            && right_type == string_type()
        {
            return Ok(string_type());
        }
        Err(format!(
            "Type mismatch: cannot apply '{}' operator on {} and {}",
            bin_expr.operator,
            get_type_name(&left_type),
            get_type_name(&right_type)
        ))
    }

    fn visit_unary_expression(&mut self, unary_expr: &UnaryExpr) -> Result<TypeId, String> {
        let operand_type = self.visit_expression(&unary_expr.right)?;

        match unary_expr.operator {
            UnaryOperator::Negate => {
                // Special case for unspecified integers - preserve unspecified_int_type
                // This allows them to be later coerced to specific types like i32
                if operand_type == unspecified_int_type() {
                    if let Expression::Literal(_) = &*unary_expr.right {
                        return Ok(unspecified_int_type());
                    }
                }

                // Check if the type is numeric using the registry
                let is_numeric = type_fullfills(&operand_type, |typeinfo| {
                    matches!(typeinfo.kind, TypeKind::Integer(_) | TypeKind::Float(_))
                });

                if is_numeric {
                    // Special case for unsigned integers
                    if operand_type == u32_type() || operand_type == u64_type() {
                        return Err("Cannot negate unsigned type".to_string());
                    }
                    return Ok(operand_type);
                }
                let type_name = get_type_name(&operand_type);
                Err(format!("Cannot negate non-numeric type '{}'", type_name))
            }
            UnaryOperator::Not => {
                // Check if the operand is a boolean
                if operand_type == bool_type() {
                    return Ok(bool_type());
                }

                // Get type name for better error message
                let operand_type_name = get_type_name(&operand_type);

                Err(format!(
                    "Boolean not operator '!' can only be applied to boolean types, but got {}",
                    operand_type_name
                ))
            }
        }
    }

    fn visit_variable_expression(&mut self, name: &str) -> Result<TypeId, String> {
        if let Some(var_type) = self.resolve_variable(name) {
            Ok(var_type.clone())
        } else {
            Err(format!("Undefined variable: {}", name))
        }
    }
}
