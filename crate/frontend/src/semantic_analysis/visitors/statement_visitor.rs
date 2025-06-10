use slang_ir::ast::*;
use slang_ir::Location;
use slang_shared::{CompilationContext, SymbolKind};
use slang_types::{PrimitiveType, TYPE_NAME_U32, TYPE_NAME_U64, TypeId};

use super::super::{
    traits::SemanticResult,
    error::SemanticAnalysisError,
    type_system,
};
use super::expression_visitor::ExpressionVisitor;

/// Handles semantic analysis for all statement types
/// 
/// This visitor is responsible for analyzing statement-level constructs
/// including function declarations, variable declarations, assignments,
/// return statements, type definitions, and control flow statements.
pub struct StatementVisitor<'a> {
    context: &'a mut CompilationContext,
    current_return_type: Option<TypeId>,
}

impl<'a> StatementVisitor<'a> {
    /// Create a new statement visitor
    /// 
    /// # Arguments
    /// * `context` - The compilation context for symbol management
    pub fn new(context: &'a mut CompilationContext) -> Self {
        Self {
            context,
            current_return_type: None,
        }
    }

    /// Create a new statement visitor with an inherited return type context
    /// 
    /// # Arguments
    /// * `context` - The compilation context for symbol management
    /// * `return_type` - The current function's return type for context inheritance
    pub fn with_return_type(context: &'a mut CompilationContext, return_type: Option<TypeId>) -> Self {
        Self {
            context,
            current_return_type: return_type,
        }
    }

    /// Set the current return type for function analysis
    /// 
    /// This is used when analyzing function bodies to validate return statements
    pub fn set_return_type(&mut self, return_type: Option<TypeId>) {
        self.current_return_type = return_type;
    }

    /// Get the current return type
    pub fn current_return_type(&self) -> Option<&TypeId> {
        self.current_return_type.as_ref()
    }

    /// Visit a function declaration statement
    pub fn visit_function_declaration(&mut self, fn_decl: &FunctionDeclarationStmt) -> SemanticResult {
        let mut param_types = Vec::new();
        for param in &fn_decl.parameters {
            param_types.push(param.param_type.clone());
        }
        
        let function_type_id = self.context.register_function_type(
            param_types.clone(), 
            fn_decl.return_type.clone()
        );
        
        if self.context.define_symbol(
            fn_decl.name.clone(), 
            SymbolKind::Function, 
            function_type_id, 
            false,
        ).is_err() {
            return Err(SemanticAnalysisError::SymbolRedefinition {
                name: fn_decl.name.clone(),
                kind: "function".to_string(),
                location: fn_decl.location,
            });
        }

        let previous_return_type = self.current_return_type.clone();
        self.current_return_type = Some(fn_decl.return_type.clone());

        self.context.begin_scope();
        for param in &fn_decl.parameters {
            if self.context.define_symbol(
                param.name.clone(), 
                SymbolKind::Variable, 
                param.param_type.clone(),
                true,
            ).is_err() {
                return Err(SemanticAnalysisError::SymbolRedefinition {
                    name: param.name.clone(),
                    kind: "parameter".to_string(),
                    location: fn_decl.location,
                });
            }
        }

        // For now, we'll need to handle block expression analysis differently
        // This will be resolved when we integrate with expression visitor
        let result = self.analyze_function_body(&fn_decl.body);

        self.current_return_type = previous_return_type;
        self.context.end_scope();
        
        result.and(Ok(fn_decl.return_type.clone()))
    }

    /// Visit a return statement
    pub fn visit_return_statement(&mut self, return_stmt: &ReturnStatement) -> SemanticResult {
        let error_location = match &return_stmt.value {
            Some(expr) => expr.location(),
            None => return_stmt.location,
        };

        if let Some(expected_type) = &self.current_return_type {
            let expected_type = expected_type.clone();
            if let Some(expr) = &return_stmt.value {
                return self.check_return_expr_type_internal(
                    expr,
                    &expected_type,
                    &expr.location(),
                );
            } else if expected_type != TypeId(PrimitiveType::Unknown as usize)
                && expected_type != TypeId(PrimitiveType::Unit as usize)
            {
                return Err(SemanticAnalysisError::MissingReturnValue {
                    expected: expected_type.clone(),
                    location: error_location,
                });
            }

            // Empty return is treated as returning unit
            Ok(TypeId(PrimitiveType::Unit as usize))
        } else {
            Err(SemanticAnalysisError::ReturnOutsideFunction {
                location: error_location,
            })
        }
    }

    /// Visit a let statement
    pub fn visit_let_statement(&mut self, let_stmt: &LetStatement) -> SemanticResult {
        // Check for negative values assigned to unsigned types
        if let Expression::Unary(unary_expr) = &let_stmt.value {
            if unary_expr.operator == UnaryOperator::Negate {
                if let Expression::Literal(lit) = &*unary_expr.right {
                    if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                        if self.context.get_type_name(&let_stmt.expr_type) == TYPE_NAME_U32
                            || self.context.get_type_name(&let_stmt.expr_type) == TYPE_NAME_U64
                        {
                            let negative_value = -n;
                            return Err(SemanticAnalysisError::ValueOutOfRange {
                                value: negative_value.to_string(),
                                target_type: let_stmt.expr_type.clone(),
                                is_float: false,
                                location: let_stmt.location,
                            });
                        }
                    }
                }
            }
        }

        // Check for symbol conflicts
        if let Some(symbol) = self.context.lookup_symbol(&let_stmt.name) {
            if symbol.kind() == SymbolKind::Type {
                return Err(SemanticAnalysisError::SymbolRedefinition {
                    name: let_stmt.name.clone(),
                    kind: "variable (conflicts with type)".to_string(),
                    location: let_stmt.location,
                });
            } else if symbol.kind() == SymbolKind::Function {
                return Err(SemanticAnalysisError::SymbolRedefinition {
                    name: let_stmt.name.clone(),
                    kind: "variable (conflicts with function)".to_string(),
                    location: let_stmt.location,
                });
            }
        }

        // TODO: This will need to be updated to use expression visitor
        let expr_type = self.visit_expression(&let_stmt.value)?;
        let final_type = self.determine_let_statement_type(let_stmt, expr_type)?;
        let final_type = type_system::finalize_inferred_type(final_type);

        if self.context.define_symbol(
            let_stmt.name.clone(),
            SymbolKind::Variable,
            final_type.clone(),
            let_stmt.is_mutable,
        ).is_err() {
            return Err(SemanticAnalysisError::VariableRedefinition {
                name: let_stmt.name.clone(),
                location: let_stmt.location,
            });
        }
        
        Ok(final_type)
    }

    /// Visit an assignment statement
    pub fn visit_assignment_statement(&mut self, assign_stmt: &AssignmentStatement) -> SemanticResult {
        // First check if variable exists and get its type and mutability
        let (var_type_id, is_mutable) = if let Some(var_info) = self.resolve_variable(&assign_stmt.name) {
            (var_info.type_id.clone(), var_info.is_mutable())
        } else {
            return Err(SemanticAnalysisError::UndefinedVariable {
                name: assign_stmt.name.clone(),
                location: assign_stmt.location,
            });
        };

        // Check mutability
        if !is_mutable {
            return Err(SemanticAnalysisError::AssignmentToImmutableVariable {
                name: assign_stmt.name.clone(),
                location: assign_stmt.location,
            });
        }

        // TODO: This will need to be updated to use expression visitor
        let expr_type = self.visit_expression(&assign_stmt.value)?;

        if var_type_id == expr_type
            || expr_type == TypeId(PrimitiveType::UnspecifiedInt as usize)
            || expr_type == TypeId(PrimitiveType::UnspecifiedFloat as usize)
        {
            Ok(var_type_id)
        } else {
            Err(SemanticAnalysisError::TypeMismatch {
                expected: var_type_id,
                actual: expr_type,
                context: Some(format!("assignment to variable '{}'", assign_stmt.name)),
                location: assign_stmt.location,
            })
        }
    }

    /// Visit a type definition statement
    pub fn visit_type_definition_statement(&mut self, type_def: &TypeDefinitionStmt) -> SemanticResult {
        if self.context.lookup_symbol(&type_def.name).is_some() {
            return Err(SemanticAnalysisError::SymbolRedefinition {
                name: type_def.name.clone(),
                kind: "type".to_string(),
                location: type_def.location,
            });
        }

        let mut field_types_for_registration = Vec::new();
        for (name, type_id) in &type_def.fields {
            if *type_id == TypeId(PrimitiveType::Unknown as usize)
                || *type_id == TypeId(PrimitiveType::UnspecifiedInt as usize)
                || *type_id == TypeId(PrimitiveType::UnspecifiedFloat as usize)
            {
                return Err(SemanticAnalysisError::InvalidFieldType {
                    struct_name: type_def.name.clone(),
                    field_name: name.clone(),
                    type_id: type_id.clone(),
                    location: type_def.location,
                });
            }
            field_types_for_registration.push((name.clone(), type_id.clone()));
        }

        match self.context.register_struct_type(type_def.name.clone(), field_types_for_registration) {
            Ok(type_id) => Ok(type_id),
            Err(_) => Err(SemanticAnalysisError::SymbolRedefinition {
                name: type_def.name.clone(),
                kind: "type".to_string(),
                location: type_def.location,
            }),
        }
    }

    /// Visit an expression statement
    pub fn visit_expression_statement(&mut self, expr: &Expression) -> SemanticResult {
        self.visit_expression(expr)
    }

    /// Visit an if statement
    pub fn visit_if_statement(&mut self, if_stmt: &IfStatement) -> SemanticResult {
        let condition_type = self.visit_expression(&if_stmt.condition)?;
        if condition_type != TypeId(PrimitiveType::Bool as usize) {
            return Err(SemanticAnalysisError::TypeMismatch {
                expected: TypeId(PrimitiveType::Bool as usize),
                actual: condition_type,
                context: Some("if condition".to_string()),
                location: if_stmt.condition.location(),
            });
        }

        self.visit_block_expression(&if_stmt.then_branch)?;

        if let Some(else_branch) = &if_stmt.else_branch {
            self.visit_block_expression(else_branch)?;
        }

        Ok(TypeId(PrimitiveType::Unit as usize))
    }

    // Helper methods that will be replaced when integrating with expression visitor
    
    fn resolve_variable(&self, name: &str) -> Option<&slang_shared::Symbol> {
        self.context.lookup_symbol(name)
            .filter(|symbol| symbol.kind() == SymbolKind::Variable)
    }

    fn check_return_expr_type_internal(
        &mut self,
        expr: &Expression,
        expected_type: &TypeId,
        location: &Location,
    ) -> SemanticResult {
        let actual_type = self.visit_expression(expr)?;

        if actual_type == *expected_type {
            return Ok(actual_type);
        }

        // Handle coercion of unspecified int to specific integer types
        if actual_type == TypeId(PrimitiveType::UnspecifiedInt as usize) {
            if type_system::is_integer_type(self.context, expected_type) {
                return type_system::check_unspecified_int_for_type(self.context, expr, expected_type);
            }
        }

        // Handle coercion of unspecified float to specific float types
        if actual_type == TypeId(PrimitiveType::UnspecifiedFloat as usize) {
            if type_system::is_float_type(self.context, expected_type) {
                return type_system::check_unspecified_float_for_type(self.context, expr, expected_type);
            }
        }

        Err(SemanticAnalysisError::ReturnTypeMismatch {
            expected: expected_type.clone(),
            actual: actual_type,
            location: *location,
        })
    }

    fn determine_let_statement_type(
        &mut self,
        let_stmt: &LetStatement,
        expr_type: TypeId,
    ) -> SemanticResult {
        type_system::determine_let_statement_type(self.context, let_stmt, expr_type)
    }

    // Helper methods for expression analysis
    
    fn visit_expression(&mut self, expr: &Expression) -> SemanticResult {
        // Create a new expression visitor with the current return type context
        let mut expr_visitor = ExpressionVisitor::with_return_type(self.context, self.current_return_type.clone());
        expr_visitor.visit_expression(expr)
    }

    fn visit_block_expression(&mut self, block: &BlockExpr) -> SemanticResult {
        // Create a new expression visitor with the current return type context
        let mut expr_visitor = ExpressionVisitor::with_return_type(self.context, self.current_return_type.clone());
        expr_visitor.visit_block_expression(block)
    }

    fn analyze_function_body(&mut self, body: &BlockExpr) -> SemanticResult {
        self.visit_block_expression(body)
    }
}
