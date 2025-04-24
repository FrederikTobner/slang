use crate::ast::{
    BinaryExpr, Expression, LetStatement, LiteralExpr, Statement, TypeDefinitionStmt, UnaryExpr, Value,
};
use crate::token::Tokentype;
use crate::visitor::Visitor;
use crate::types::{TypeId, TypeKind, TYPE_REGISTRY};
use std::collections::HashMap;

// Use the accessor functions instead of static references
fn i32_type() -> TypeId {
    crate::types::i32_type()
}

fn i64_type() -> TypeId {
    crate::types::i64_type()
}

fn u32_type() -> TypeId {
    crate::types::u32_type()
}

fn u64_type() -> TypeId {
    crate::types::u64_type()
}

fn f64_type() -> TypeId {
    crate::types::f64_type()
}

fn string_type() -> TypeId {
    crate::types::string_type()
}

fn unspecified_int_type() -> TypeId {
    crate::types::unspecified_int_type()
}

fn unknown_type() -> TypeId {
    crate::types::unknown_type()
}

pub struct TypeChecker {
    variables: HashMap<String, TypeId>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            variables: HashMap::new(),
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

    #[allow(dead_code)]
    pub fn get_variables(&self) -> &HashMap<String, TypeId> {
        &self.variables
    }
    
    #[allow(dead_code)]
    pub fn set_variable(&mut self, name: String, type_id: TypeId) {
        self.variables.insert(name, type_id);
    }
}

impl Visitor<Result<TypeId, String>> for TypeChecker {
    fn visit_statement(&mut self, stmt: &Statement) -> Result<TypeId, String> {
        match stmt {
            Statement::Let(let_stmt) => self.visit_let_statement(let_stmt),
            Statement::Expression(expr) => self.visit_expression_statement(expr),
            Statement::TypeDefinition(type_def) => self.visit_type_definition_statement(type_def),
        }
    }
    
    fn visit_type_definition_statement(&mut self, type_def: &TypeDefinitionStmt) -> Result<TypeId, String> {
        // Register the new struct type
        TYPE_REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            
            // Create a new struct type
            let type_kind = crate::types::TypeKind::Struct(crate::types::StructType {
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
        let expr_type = self.visit_expression(&let_stmt.value)?;
        
        // If type wasn't specified, infer it
        let final_type = if let_stmt.expr_type == unknown_type() {
            expr_type
        } else if let_stmt.expr_type != expr_type {
            // Only allow UnspecifiedInteger to be assigned to specific integer types
            // with value range check
            if expr_type == unspecified_int_type() {
                if let Expression::Literal(lit) = &let_stmt.value {
                    if let Value::UnspecifiedInteger(n) = &lit.value {
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
                } else {
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

        // Add to symbol table
        self.variables.insert(let_stmt.name.clone(), final_type.clone());
        Ok(final_type)
    }

    fn visit_expression(&mut self, expr: &Expression) -> Result<TypeId, String> {
        match expr {
            Expression::Literal(lit_expr) => self.visit_literal_expression(lit_expr),
            Expression::Binary(bin_expr) => self.visit_binary_expression(bin_expr),
            Expression::Variable(name) => self.visit_variable_expression(name),
            Expression::Unary(unary_expr) => self.visit_unary_expression(unary_expr),
        }
    }

    fn visit_literal_expression(&mut self, lit_expr: &LiteralExpr) -> Result<TypeId, String> {
        // Infer type from literal
        match lit_expr.value {
            Value::I32(_) => Ok(i32_type()),
            Value::I64(_) => Ok(i64_type()),
            Value::U32(_) => Ok(u32_type()),
            Value::U64(_) => Ok(u64_type()),
            Value::F64(_) => Ok(f64_type()),
            Value::UnspecifiedInteger(_) => Ok(unspecified_int_type()),
            Value::String(_) => Ok(string_type()),
        }
    }

    fn visit_binary_expression(&mut self, bin_expr: &BinaryExpr) -> Result<TypeId, String> {
        let left_type = self.visit_expression(&bin_expr.left)?;
        let right_type = self.visit_expression(&bin_expr.right)?;

        // Handle arithmetic operations using the type registry
        if matches!(
            bin_expr.operator,
            Tokentype::Plus | Tokentype::Minus | Tokentype::Multiply | Tokentype::Divide
        ) {
            // Only allow operations between same types, with special handling for unspecified integers
            if left_type == right_type {
                return Ok(left_type);
            }
            
            // Special case for unspecified integers - allow them to be used with specific types
            // but adopt the specific type rather than promoting/converting
            if left_type == unspecified_int_type() {
                // Check if the unspecified integer literal value is in range
                if let Expression::Literal(lit) = &*bin_expr.left {
                    if let Value::UnspecifiedInteger(n) = &lit.value {
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
            
            if right_type == unspecified_int_type() {
                // Similar check for right side
                if let Expression::Literal(lit) = &*bin_expr.right {
                    if let Value::UnspecifiedInteger(n) = &lit.value {
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
