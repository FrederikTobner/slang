use crate::ast::{
    BinaryExpr, Expression, FunctionCallExpr, FunctionDeclarationStmt, LetStatement, LiteralExpr, 
    Statement, TypeDefinitionStmt, UnaryExpr, LiteralValue,
};
use crate::token::Tokentype;
use crate::visitor::Visitor;
use crate::types::{TypeId, TypeKind, TYPE_REGISTRY, StructType};
use std::collections::HashMap;

/// Helper function to get the i32 type ID
fn i32_type() -> TypeId {
    crate::types::i32_type()
}

/// Helper function to get the i64 type ID
fn i64_type() -> TypeId {
    crate::types::i64_type()
}

/// Helper function to get the u32 type ID
fn u32_type() -> TypeId {
    crate::types::u32_type()
}

/// Helper function to get the u64 type ID
fn u64_type() -> TypeId {
    crate::types::u64_type()
}

/// Helper function to get the f32 type ID
fn f32_type() -> TypeId {
    crate::types::f32_type()
}

/// Helper function to get the f64 type ID
fn f64_type() -> TypeId {
    crate::types::f64_type()
}

/// Helper function to get the boolean type ID
fn bool_type() -> TypeId {
    crate::types::bool_type()
}

/// Helper function to get the unspecified float type ID
fn unspecified_float_type() -> TypeId {
    crate::types::unspecified_float_type()
}

/// Helper function to get the string type ID
fn string_type() -> TypeId {
    crate::types::string_type()
}

/// Helper function to get the unspecified integer type ID
fn unspecified_int_type() -> TypeId {
    crate::types::unspecified_int_type()
}

/// Helper function to get the unknown type ID
fn unknown_type() -> TypeId {
    crate::types::unknown_type()
}

/// Performs static type checking on the AST
pub struct TypeChecker {
    /// Map of variable names to their types
    variables: HashMap<String, TypeId>,
    /// Map of function names to their parameter and return types
    functions: HashMap<String, (Vec<TypeId>, TypeId)>,
    /// Current function's return type for validating return statements
    current_return_type: Option<TypeId>,
    /// Set of native functions that can accept any type of arguments
    native_variadic_functions: std::collections::HashSet<String>,
}

impl TypeChecker {
    /// Creates a new type checker with built-in functions registered
    pub fn new() -> Self {
        let mut tc = TypeChecker {
            variables: HashMap::new(),
            functions: HashMap::new(),
            current_return_type: None,
            native_variadic_functions: std::collections::HashSet::new(),
        };
        
        tc.register_native_functions();
        
        tc
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
        TYPE_REGISTRY.with(|registry| {
            let registry = registry.borrow();
            if let Some(type_info) = registry.get_type_info(type_id) {
                matches!(type_info.kind, TypeKind::Integer(_))
            } else {
                false
            }
        })
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
        TYPE_REGISTRY.with(|registry| {
            let registry = registry.borrow();
            if let Some(type_info) = registry.get_type_info(type_id) {
                matches!(type_info.kind, TypeKind::Float(_))
            } else {
                false
            }
        })
    }
    
    /// Registers the built-in native functions
    fn register_native_functions(&mut self) {
        self.functions.insert("print_value".to_string(), (vec![unknown_type()], i32_type()));
        // mark it as a special function that accepts any type
        self.native_variadic_functions.insert("print_value".to_string());
    }

    /// Checks the type safety of a list of statements
    /// 
    /// # Arguments
    /// 
    /// * `statements` - The statements to check
    /// 
    /// # Returns
    /// 
    /// Ok(()) if type-safe, or an error message
    pub fn check(&mut self, statements: &[Statement]) -> Result<(), String> {
        for stmt in statements {
            match stmt.accept(self) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

}

impl Visitor<Result<TypeId, String>> for TypeChecker {
    fn visit_statement(&mut self, stmt: &Statement) -> Result<TypeId, String> {
        match stmt {
            Statement::Let(let_stmt) => self.visit_let_statement(let_stmt),
            Statement::Expression(expr) => self.visit_expression_statement(expr),
            Statement::TypeDefinition(type_def) => self.visit_type_definition_statement(type_def),
            Statement::FunctionDeclaration(fn_decl) => self.visit_function_declaration_statement(fn_decl),
            Statement::Block(stmts) => self.visit_block_statement(stmts),
            Statement::Return(expr) => self.visit_return_statement(expr),
        }
    }
    
    fn visit_function_declaration_statement(&mut self, fn_decl: &FunctionDeclarationStmt) -> Result<TypeId, String> {
        // Check parameter types
        let mut param_types = Vec::new();
        for param in &fn_decl.parameters {
            param_types.push(param.param_type.clone());
        }
        
        self.functions.insert(fn_decl.name.clone(), (param_types, fn_decl.return_type.clone()));
        
        let previous_return_type = self.current_return_type.clone();
        self.current_return_type = Some(fn_decl.return_type.clone());
        
        let mut saved_variables = HashMap::new();
        for param in &fn_decl.parameters {
            if self.variables.contains_key(&param.name) {
                saved_variables.insert(param.name.clone(), self.variables[&param.name].clone());
            }
            self.variables.insert(param.name.clone(), param.param_type.clone());
        }
        
        let result = self.visit_block_statement(&fn_decl.body);
        
        for (name, type_id) in saved_variables {
            self.variables.insert(name, type_id);
        }
        self.current_return_type = previous_return_type;
        
        result.and(Ok(fn_decl.return_type.clone()))
    }
    
    fn visit_block_statement(&mut self, stmts: &Vec<Statement>) -> Result<TypeId, String> {
        // Save current variables to restore scope later
        let saved_variables = self.variables.clone();
        
        // Type-check each statement
        for stmt in stmts {
            stmt.accept(self)?;
        }
        
        // Restore previous scope, removing all locally defined variables
        self.variables = saved_variables;
        
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
                                registry.borrow().check_float_value_in_range(f, &expected_type)
                            });
                            
                            if value_in_range {
                                return Ok(expected_type.clone());
                            }
                        }
                    }
                }
                
                // Type mismatch
                let actual_type_name = TYPE_REGISTRY.with(|registry| {
                    registry.borrow()
                        .get_type_info(&actual_type)
                        .map(|t| t.name.clone())
                        .unwrap_or_else(|| format!("{:?}", actual_type))
                });
                
                let expected_type_name = TYPE_REGISTRY.with(|registry| {
                    registry.borrow()
                        .get_type_info(&expected_type)
                        .map(|t| t.name.clone())
                        .unwrap_or_else(|| format!("{:?}", expected_type))
                });
                
                return Err(format!(
                    "Type mismatch: function returns {} but got {}",
                    expected_type_name, actual_type_name
                ));
            } else if expected_type != unknown_type() {
                // Missing return value when one is expected
                let expected_type_name = TYPE_REGISTRY.with(|registry| {
                    registry.borrow()
                        .get_type_info(&expected_type)
                        .map(|t| t.name.clone())
                        .unwrap_or_else(|| format!("{:?}", expected_type))
                });
                
                return Err(format!(
                    "Type mismatch: function returns {} but no return value provided",
                    expected_type_name
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
            
            // Special handling for native functions that accept any type
            let is_special_native = self.native_variadic_functions.contains(&call_expr.name);
            
            // Check each argument
            for (i, arg) in call_expr.arguments.iter().enumerate() {
                let arg_type = self.visit_expression(arg)?;
                let param_type = &param_types[i];
                
                // function that can accept any type(s)
                if is_special_native {
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
                    
                    // Type mismatch
                    let arg_type_name = TYPE_REGISTRY.with(|registry| {
                        registry.borrow()
                            .get_type_info(&arg_type)
                            .map(|t| t.name.clone())
                            .unwrap_or_else(|| format!("{:?}", arg_type))
                    });
                    
                    let param_type_name = TYPE_REGISTRY.with(|registry| {
                        registry.borrow()
                            .get_type_info(param_type)
                            .map(|t| t.name.clone())
                            .unwrap_or_else(|| format!("{:?}", param_type))
                    });
                    
                    return Err(format!(
                        "Type mismatch: function '{}' expects argument {} to be {}, but got {}",
                        call_expr.name, i + 1, param_type_name, arg_type_name
                    ));
                }
            }
            
            // All arguments match, return the function's return type
            Ok(return_type)
        } else {
            Err(format!("Undefined function: {}", call_expr.name))
        }
    }
    
    fn visit_type_definition_statement(&mut self, type_def: &TypeDefinitionStmt) -> Result<TypeId, String> {
        // Register the new struct type
        TYPE_REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            
            // Create a new struct type
            let type_kind = crate::types::TypeKind::Struct(StructType {
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

        // Add to symbol table with the placeholder type
        self.variables.insert(let_stmt.name.clone(), placeholder_type);
        
        // Now process the initialization expression
        let expr_type = self.visit_expression(&let_stmt.value)?;
        
        // If type wasn't specified, infer it
        let final_type = if let_stmt.expr_type == unknown_type() {
            expr_type
        } else if let_stmt.expr_type != expr_type {
            // Only allow UnspecifiedInteger to be assigned to specific integer types
            // with value range check
            if expr_type == unspecified_int_type() {
                // Check for both direct literals and literals inside unary expressions
                match &let_stmt.value {
                    Expression::Literal(lit) => {
                        if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                            // Check if the value is in range for the target type
                            let value_in_range = TYPE_REGISTRY.with(|registry| {
                                registry.borrow().check_value_in_range(n, &let_stmt.expr_type)
                            });
                            
                            if value_in_range {
                                let_stmt.expr_type.clone()
                            } else {
                                // Get type name for error message
                                let target_type_name = TYPE_REGISTRY.with(|registry| {
                                    registry.borrow()
                                        .get_type_info(&let_stmt.expr_type)
                                        .map(|t| t.name.clone())
                                        .unwrap_or_else(|| format!("{:?}", let_stmt.expr_type))
                                });
                                
                                return Err(format!(
                                    "Integer literal {} is out of range for type {}",
                                    n, target_type_name
                                ));
                            }
                        } else {
                            // Non-integer literals can't be assigned to integer types
                            let expr_type_name = TYPE_REGISTRY.with(|registry| {
                                registry.borrow()
                                    .get_type_info(&expr_type)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| format!("{:?}", expr_type))
                            });
                            
                            let target_type_name = TYPE_REGISTRY.with(|registry| {
                                registry.borrow()
                                    .get_type_info(&let_stmt.expr_type)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| format!("{:?}", let_stmt.expr_type))
                            });
                            
                            return Err(format!(
                                "Type mismatch: variable {} is {} but expression is {}",
                                let_stmt.name, target_type_name, expr_type_name
                            ));
                        }
                    },
                    Expression::Unary(unary_expr) => {
                        if unary_expr.operator == Tokentype::Minus {
                            // Handle negated integer literals
                            if let Expression::Literal(lit) = &*unary_expr.right {
                                if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                                    // For negation, we need to check the negative value
                                    let negated_value = -*n;
                                    let value_in_range = TYPE_REGISTRY.with(|registry| {
                                        registry.borrow().check_value_in_range(&negated_value, &let_stmt.expr_type)
                                    });
                                    
                                    if value_in_range {
                                        return Ok(let_stmt.expr_type.clone());
                                    } else {
                                        // Get type name for error message
                                        let target_type_name = TYPE_REGISTRY.with(|registry| {
                                            registry.borrow()
                                                .get_type_info(&let_stmt.expr_type)
                                                .map(|t| t.name.clone())
                                                .unwrap_or_else(|| format!("{:?}", let_stmt.expr_type))
                                        });
                                        
                                        return Err(format!(
                                            "Integer literal {} is out of range for type {}",
                                            negated_value, target_type_name
                                        ));
                                    }
                                }
                            }
                        }
                        
                        // Non-literal expressions can't be assigned if types don't match exactly
                        let expr_type_name = TYPE_REGISTRY.with(|registry| {
                            registry.borrow()
                                .get_type_info(&expr_type)
                                .map(|t| t.name.clone())
                                .unwrap_or_else(|| format!("{:?}", expr_type))
                        });
                        
                        let target_type_name = TYPE_REGISTRY.with(|registry| {
                            registry.borrow()
                                .get_type_info(&let_stmt.expr_type)
                                .map(|t| t.name.clone())
                                .unwrap_or_else(|| format!("{:?}", let_stmt.expr_type))
                        });
                        
                        return Err(format!(
                            "Type mismatch: variable {} is {} but expression is {}",
                            let_stmt.name, target_type_name, expr_type_name
                        ));
                    },
                    _ => {
                        // Non-literal expressions can't be assigned if types don't match exactly
                        let expr_type_name = TYPE_REGISTRY.with(|registry| {
                            registry.borrow()
                                .get_type_info(&expr_type)
                                .map(|t| t.name.clone())
                                .unwrap_or_else(|| format!("{:?}", expr_type))
                        });
                        
                        let target_type_name = TYPE_REGISTRY.with(|registry| {
                            registry.borrow()
                                .get_type_info(&let_stmt.expr_type)
                                .map(|t| t.name.clone())
                                .unwrap_or_else(|| format!("{:?}", let_stmt.expr_type))
                        });
                        
                        return Err(format!(
                            "Type mismatch: variable {} is {} but expression is {}",
                            let_stmt.name, target_type_name, expr_type_name
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
                                    registry.borrow().check_float_value_in_range(f, &let_stmt.expr_type)
                                });
                                
                                if value_in_range {
                                    let_stmt.expr_type.clone()
                                } else {
                                    // Get type name for error message
                                    let target_type_name = TYPE_REGISTRY.with(|registry| {
                                        registry.borrow()
                                            .get_type_info(&let_stmt.expr_type)
                                            .map(|t| t.name.clone())
                                            .unwrap_or_else(|| format!("{:?}", let_stmt.expr_type))
                                    });
                                    
                                    return Err(format!(
                                        "Float literal {} is out of range for type {}",
                                        f, target_type_name
                                    ));
                                }
                            } else {
                                // Non-float types can't be assigned float literals
                                let expr_type_name = TYPE_REGISTRY.with(|registry| {
                                    registry.borrow()
                                        .get_type_info(&expr_type)
                                        .map(|t| t.name.clone())
                                        .unwrap_or_else(|| format!("{:?}", expr_type))
                                });
                                
                                let target_type_name = TYPE_REGISTRY.with(|registry| {
                                    registry.borrow()
                                        .get_type_info(&let_stmt.expr_type)
                                        .map(|t| t.name.clone())
                                        .unwrap_or_else(|| format!("{:?}", let_stmt.expr_type))
                                });
                                
                                return Err(format!(
                                    "Type mismatch: variable {} is {} but expression is {}",
                                    let_stmt.name, target_type_name, expr_type_name
                                ));
                            }
                        } else {
                            // This branch should never be reached, but just in case
                            let expr_type_name = TYPE_REGISTRY.with(|registry| {
                                registry.borrow()
                                    .get_type_info(&expr_type)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| format!("{:?}", expr_type))
                            });
                            
                            let target_type_name = TYPE_REGISTRY.with(|registry| {
                                registry.borrow()
                                    .get_type_info(&let_stmt.expr_type)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| format!("{:?}", let_stmt.expr_type))
                            });
                            
                            return Err(format!(
                                "Type mismatch: variable {} is {} but expression is {}",
                                let_stmt.name, target_type_name, expr_type_name
                            ));
                        }
                    },
                    Expression::Unary(unary_expr) => {
                        if unary_expr.operator == Tokentype::Minus {
                            // Handle negated float literals
                            if let Expression::Literal(lit) = &*unary_expr.right {
                                if let LiteralValue::UnspecifiedFloat(f) = &lit.value {
                                    if self.is_float_type(&let_stmt.expr_type) {
                                        // For negation, we need to check the negative value
                                        let negated_value = -*f;
                                        let value_in_range = TYPE_REGISTRY.with(|registry| {
                                            registry.borrow().check_float_value_in_range(&negated_value, &let_stmt.expr_type)
                                        });
                                        
                                        if value_in_range {
                                            return Ok(let_stmt.expr_type.clone());
                                        } else {
                                            // Get type name for error message
                                            let target_type_name = TYPE_REGISTRY.with(|registry| {
                                                registry.borrow()
                                                    .get_type_info(&let_stmt.expr_type)
                                                    .map(|t| t.name.clone())
                                                    .unwrap_or_else(|| format!("{:?}", let_stmt.expr_type))
                                            });
                                            
                                            return Err(format!(
                                                "Float literal {} is out of range for type {}",
                                                negated_value, target_type_name
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Non-literal expressions can't be assigned if types don't match exactly
                        let expr_type_name = TYPE_REGISTRY.with(|registry| {
                            registry.borrow()
                                .get_type_info(&expr_type)
                                .map(|t| t.name.clone())
                                .unwrap_or_else(|| format!("{:?}", expr_type))
                        });
                        
                        let target_type_name = TYPE_REGISTRY.with(|registry| {
                            registry.borrow()
                                .get_type_info(&let_stmt.expr_type)
                                .map(|t| t.name.clone())
                                .unwrap_or_else(|| format!("{:?}", let_stmt.expr_type))
                        });
                        
                        return Err(format!(
                            "Type mismatch: variable {} is {} but expression is {}",
                            let_stmt.name, target_type_name, expr_type_name
                        ));
                    },
                    _ => {
                        // Non-literal expressions can't be assigned if types don't match exactly
                        let expr_type_name = TYPE_REGISTRY.with(|registry| {
                            registry.borrow()
                                .get_type_info(&expr_type)
                                .map(|t| t.name.clone())
                                .unwrap_or_else(|| format!("{:?}", expr_type))
                        });
                        
                        let target_type_name = TYPE_REGISTRY.with(|registry| {
                            registry.borrow()
                                .get_type_info(&let_stmt.expr_type)
                                .map(|t| t.name.clone())
                                .unwrap_or_else(|| format!("{:?}", let_stmt.expr_type))
                        });
                        
                        return Err(format!(
                            "Type mismatch: variable {} is {} but expression is {}",
                            let_stmt.name, target_type_name, expr_type_name
                        ));
                    }
                }
            } else {
                // Types don't match and no coercion is allowed
                let expr_type_name = TYPE_REGISTRY.with(|registry| {
                    registry.borrow()
                        .get_type_info(&expr_type)
                        .map(|t| t.name.clone())
                        .unwrap_or_else(|| format!("{:?}", expr_type))
                });
                
                let target_type_name = TYPE_REGISTRY.with(|registry| {
                    registry.borrow()
                        .get_type_info(&let_stmt.expr_type)
                        .map(|t| t.name.clone())
                        .unwrap_or_else(|| format!("{:?}", let_stmt.expr_type))
                });
                
                return Err(format!(
                    "Type mismatch: variable {} is {} but expression is {}",
                    let_stmt.name, target_type_name, expr_type_name
                ));
            }
        } else {
            let_stmt.expr_type.clone()
        };

        // Update symbol table with the final type
        self.variables.insert(let_stmt.name.clone(), final_type.clone());
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
        if bin_expr.operator == Tokentype::And || bin_expr.operator == Tokentype::Or {
            // Check that both operands are boolean
            if left_type == bool_type() && right_type == bool_type() {
                return Ok(bool_type());
            } else {
                // Get type names for better error messages
                let left_type_name = TYPE_REGISTRY.with(|registry| {
                    registry.borrow()
                        .get_type_info(&left_type)
                        .map(|t| t.name.clone())
                        .unwrap_or_else(|| format!("{:?}", left_type))
                });
                
                let right_type_name = TYPE_REGISTRY.with(|registry| {
                    registry.borrow()
                        .get_type_info(&right_type)
                        .map(|t| t.name.clone())
                        .unwrap_or_else(|| format!("{:?}", right_type))
                });
                
                let operator_str = if bin_expr.operator == Tokentype::And { "&&" } else { "||" };
                
                return Err(format!(
                    "Logical operator '{}' requires boolean operands, got {} and {}",
                    operator_str, left_type_name, right_type_name
                ));
            }
        }
        
        // Handle relational operators (>, <, >=, <=, ==, !=)
        if matches!(
            bin_expr.operator,
            Tokentype::Greater | Tokentype::Less | Tokentype::GreaterEqual | 
            Tokentype::LessEqual | Tokentype::EqualEqual | Tokentype::NotEqual
        ) {
            // For now, only allow comparing same types (or unspecified literals with specific types)
            if left_type == right_type || 
               (left_type == unspecified_int_type() && self.is_integer_type(&right_type)) ||
               (right_type == unspecified_int_type() && self.is_integer_type(&left_type)) ||
               (left_type == unspecified_float_type() && self.is_float_type(&right_type)) ||
               (right_type == unspecified_float_type() && self.is_float_type(&left_type)) {
                return Ok(bool_type());
            }
            
            // Get type names for better error messages
            let left_type_name = TYPE_REGISTRY.with(|registry| {
                registry.borrow()
                    .get_type_info(&left_type)
                    .map(|t| t.name.clone())
                    .unwrap_or_else(|| format!("{:?}", left_type))
            });
            
            let right_type_name = TYPE_REGISTRY.with(|registry| {
                registry.borrow()
                    .get_type_info(&right_type)
                    .map(|t| t.name.clone())
                    .unwrap_or_else(|| format!("{:?}", right_type))
            });
            
            let operator_str = match bin_expr.operator {
                Tokentype::Greater => ">",
                Tokentype::Less => "<",
                Tokentype::GreaterEqual => ">=",
                Tokentype::LessEqual => "<=",
                Tokentype::EqualEqual => "==",
                Tokentype::NotEqual => "!=",
                _ => unreachable!(),
            };
            
            return Err(format!(
                "Cannot compare different types with '{}': {} and {}",
                operator_str, left_type_name, right_type_name
            ));
        }
        
        // Handle arithmetic operations using the type registry
        if matches!(
            bin_expr.operator,
            Tokentype::Plus | Tokentype::Minus | Tokentype::Multiply | Tokentype::Divide
        ) {
            // Only allow operations between same types, with special handling for unspecified literals
            if left_type == right_type {
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
                            let right_type_name = TYPE_REGISTRY.with(|registry| {
                                registry.borrow()
                                    .get_type_info(&right_type)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| format!("{:?}", right_type))
                            });
                            
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
                if let Expression::Literal(lit) = &*bin_expr.right {
                    if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                        let value_in_range = TYPE_REGISTRY.with(|registry| {
                            registry.borrow().check_value_in_range(n, &left_type)
                        });
                        
                        if value_in_range {
                            return Ok(left_type);
                        } else {
                            let left_type_name = TYPE_REGISTRY.with(|registry| {
                                registry.borrow()
                                    .get_type_info(&left_type)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| format!("{:?}", left_type))
                            });
                            
                            return Err(format!(
                                "Integer literal {} is out of range for type {}",
                                n, left_type_name
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
                            let right_type_name = TYPE_REGISTRY.with(|registry| {
                                registry.borrow()
                                    .get_type_info(&right_type)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| format!("{:?}", right_type))
                            });
                            
                            return Err(format!(
                                "Float literal {} is out of range for type {}",
                                f, right_type_name
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
                            let left_type_name = TYPE_REGISTRY.with(|registry| {
                                registry.borrow()
                                    .get_type_info(&left_type)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| format!("{:?}", left_type))
                            });
                            
                            return Err(format!(
                                "Float literal {} is out of range for type {}",
                                f, left_type_name
                            ));
                        }
                    }
                }
                return Ok(left_type);
            }
        }
        
        // String concatenation - still allowed
        if bin_expr.operator == Tokentype::Plus 
            && left_type == string_type() 
            && right_type == string_type() {
                return Ok(string_type());
        }
        
        // Get type names for better error messages
        let left_type_name = TYPE_REGISTRY.with(|registry| {
            registry.borrow()
                .get_type_info(&left_type)
                .map(|t| t.name.clone())
                .unwrap_or_else(|| format!("{:?}", left_type))
        });
        
        let right_type_name = TYPE_REGISTRY.with(|registry| {
            registry.borrow()
                .get_type_info(&right_type)
                .map(|t| t.name.clone())
                .unwrap_or_else(|| format!("{:?}", right_type))
        });
            
        Err(format!(
            "Type mismatch: cannot perform {:?} operation with {} and {}",
            bin_expr.operator, left_type_name, right_type_name
        ))
    }

    fn visit_unary_expression(&mut self, unary_expr: &UnaryExpr) -> Result<TypeId, String> {
        let operand_type = self.visit_expression(&unary_expr.right)?;
        
        match unary_expr.operator {
            Tokentype::Minus => {
                // Special case for unspecified integers - preserve unspecified_int_type
                // This allows them to be later coerced to specific types like i32
                if operand_type == unspecified_int_type() {
                    if let Expression::Literal(_) = &*unary_expr.right {
                        return Ok(unspecified_int_type());
                    }
                }
                
                // Check if the type is numeric using the registry
                let is_numeric = TYPE_REGISTRY.with(|registry| {
                    let registry = registry.borrow();
                    if let Some(type_info) = registry.get_type_info(&operand_type) {
                        matches!(type_info.kind, 
                            TypeKind::Integer(_) | TypeKind::Float(_))
                    } else {
                        false
                    }
                });
                
                if is_numeric {
                    // Special case for unsigned integers
                    if operand_type == u32_type() || operand_type == u64_type() {
                        return Err(format!("Cannot negate unsigned type"));
                    }
                    return Ok(operand_type);
                }
                
                Err(format!("Cannot negate non-numeric type"))
            },
            Tokentype::Not => {
                // Check if the operand is a boolean
                if operand_type == bool_type() {
                    return Ok(bool_type());
                }
                
                // Get type name for better error message
                let operand_type_name = TYPE_REGISTRY.with(|registry| {
                    registry.borrow()
                        .get_type_info(&operand_type)
                        .map(|t| t.name.clone())
                        .unwrap_or_else(|| format!("{:?}", operand_type))
                });
                
                Err(format!("Boolean negation operator '!' can only be applied to boolean types, but got {}", operand_type_name))
            },
            _ => Err(format!(
                "Invalid unary operation: {:?}",
                unary_expr.operator
            )),
        }
    }
    
    fn visit_variable_expression(&mut self, name: &str) -> Result<TypeId, String> {
        if let Some(var_type) = self.variables.get(name) {
            Ok(var_type.clone())
        } else {
            Err(format!("Undefined variable: {}", name))
        }
    }
}
