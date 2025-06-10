use slang_ir::ast::*;
use slang_shared::{CompilationContext, SymbolKind};
use slang_types::{TypeId};

use super::super::{
    traits::SemanticResult,
    error::SemanticAnalysisError,
    operations,
    validation::TypeCheckingCoordinator,
};

/// Handles semantic analysis for all expression types
/// 
/// This visitor is responsible for analyzing expression-level constructs
/// including binary operations, unary operations, function calls, variable
/// references, literals, conditionals, and block expressions.
pub struct ExpressionVisitor<'a> {
    context: &'a mut CompilationContext,
    current_return_type: Option<TypeId>,
}

impl<'a> ExpressionVisitor<'a> {
    /// Create a new expression visitor
    /// 
    /// # Arguments
    /// * `context` - The compilation context for type information and symbol lookup
    pub fn new(context: &'a mut CompilationContext) -> Self {
        Self { 
            context,
            current_return_type: None,
        }
    }

    /// Create a new expression visitor with a specific return type context
    /// 
    /// # Arguments
    /// * `context` - The compilation context for type information and symbol lookup
    /// * `current_return_type` - The current function's return type for validation
    pub fn with_return_type(context: &'a mut CompilationContext, current_return_type: Option<TypeId>) -> Self {
        Self { 
            context,
            current_return_type,
        }
    }

    /// Set the current return type for function analysis
    pub fn set_return_type(&mut self, return_type: Option<TypeId>) {
        self.current_return_type = return_type;
    }

    /// Create a type checking coordinator for this visitor's context
    fn create_type_coordinator(&self) -> TypeCheckingCoordinator {
        TypeCheckingCoordinator::new(self.context)
    }

    /// Visit an expression and determine its type
    pub fn visit_expression(&mut self, expr: &Expression) -> SemanticResult {
        match expr {
            Expression::Binary(bin_expr) => self.visit_binary_expression(bin_expr),
            Expression::Unary(unary_expr) => self.visit_unary_expression(unary_expr),
            Expression::Call(call_expr) => self.visit_call_expression(call_expr),
            Expression::Variable(var_expr) => self.visit_variable_expression(var_expr),
            Expression::Literal(lit_expr) => self.visit_literal_expression(lit_expr),
            Expression::Conditional(cond_expr) => self.visit_conditional_expression(cond_expr),
            Expression::Block(block_expr) => self.visit_block_expression(block_expr),
            Expression::FunctionType(func_type_expr) => self.visit_function_type_expression(func_type_expr),
        }
    }

    /// Visit a binary expression
    pub fn visit_binary_expression(&mut self, bin_expr: &BinaryExpr) -> SemanticResult {
        let left_type = self.visit_expression(&bin_expr.left)?;
        let right_type = self.visit_expression(&bin_expr.right)?;

        // Handle logical operations
        if bin_expr.operator == BinaryOperator::And || bin_expr.operator == BinaryOperator::Or {
            return operations::check_logical_operation(
                &left_type,
                &right_type,
                &bin_expr.operator,
                &bin_expr.location,
            );
        }

        // Handle relational operations
        if matches!(
            bin_expr.operator,
            BinaryOperator::GreaterThan
                | BinaryOperator::LessThan
                | BinaryOperator::GreaterThanOrEqual
                | BinaryOperator::LessThanOrEqual
                | BinaryOperator::Equal
                | BinaryOperator::NotEqual
        ) {
            return operations::check_relational_operation(
                self.context,
                &left_type,
                &right_type,
                &bin_expr.operator,
                &bin_expr.location,
            );
        }

        // Handle arithmetic operations
        if matches!(
            bin_expr.operator,
            BinaryOperator::Add
                | BinaryOperator::Subtract
                | BinaryOperator::Multiply
                | BinaryOperator::Divide
        ) {
            if left_type == right_type {
                return operations::check_same_type_arithmetic(
                    self.context,
                    &left_type,
                    &bin_expr.operator,
                    &bin_expr.location,
                );
            }

            // Use coordinator for mixed arithmetic with coercion
            let coordinator = self.create_type_coordinator();
            return coordinator.check_mixed_arithmetic_with_coercion(
                &left_type,
                &right_type,
                bin_expr,
            );
        }

        Err(SemanticAnalysisError::OperationTypeMismatch {
            operator: bin_expr.operator.to_string(),
            left_type: left_type.clone(),
            right_type: right_type.clone(),
            location: bin_expr.location,
        })
    }

    /// Visit a unary expression
    pub fn visit_unary_expression(&mut self, unary_expr: &UnaryExpr) -> SemanticResult {
        let operand_type = self.visit_expression(&unary_expr.right)?;

        operations::unary::check_unary_operation(self.context, unary_expr, &operand_type)
    }

    /// Visit a function call expression
    pub fn visit_call_expression(&mut self, call_expr: &FunctionCallExpr) -> SemanticResult {
        let function_type = if let Some(symbol) = self.context.lookup_symbol(&call_expr.name) {
            match symbol.kind() {
                SymbolKind::Function => {
                    if self.context.is_function_type(&symbol.type_id) {
                        self.context.get_function_type(&symbol.type_id).cloned()
                    } else {
                        return Err(SemanticAnalysisError::UndefinedFunction {
                            name: call_expr.name.clone(),
                            location: call_expr.location,
                        });
                    }
                }
                SymbolKind::Variable => {
                    if self.context.is_function_type(&symbol.type_id) {
                        self.context.get_function_type(&symbol.type_id).cloned()
                    } else {
                        return Err(SemanticAnalysisError::VariableNotCallable {
                            variable_name: call_expr.name.clone(),
                            variable_type: symbol.type_id.clone(),
                            location: call_expr.location,
                        });
                    }
                }
                _ => {
                    return Err(SemanticAnalysisError::UndefinedFunction {
                        name: call_expr.name.clone(),
                        location: call_expr.location,
                    });
                }
            }
        } else {
            return Err(SemanticAnalysisError::UndefinedFunction {
                name: call_expr.name.clone(),
                location: call_expr.location,
            });
        };

        if let Some(func_type) = function_type {
            // Check argument count
            if func_type.param_types.len() != call_expr.arguments.len() {
                return Err(SemanticAnalysisError::ArgumentCountMismatch {
                    function_name: call_expr.name.clone(),
                    expected: func_type.param_types.len(),
                    actual: call_expr.arguments.len(),
                    location: call_expr.location,
                });
            }

            // Check argument types
            for (i, arg) in call_expr.arguments.iter().enumerate() {
                let param_type = func_type.param_types[i].clone();
                let arg_type = self.visit_expression(arg)?;

                if param_type == TypeId::unknown() {
                    continue;
                }

                // Use coordinator for assignment compatibility checking
                let coordinator = self.create_type_coordinator();
                if !coordinator.check_assignment_compatibility(&param_type, &arg_type) {
                    // For unspecified literals, try range validation
                    if arg_type == TypeId::unspecified_int() 
                        || arg_type == TypeId::unspecified_float() {
                        if coordinator.validate_literal_range(arg, &param_type).is_err() {
                            return Err(SemanticAnalysisError::ArgumentTypeMismatch {
                                function_name: call_expr.name.clone(),
                                argument_position: i + 1,
                                expected: param_type.clone(),
                                actual: arg_type,
                                location: arg.location(),
                            });
                        }
                    } else {
                        // For non-literal types, it's a direct type mismatch
                        return Err(SemanticAnalysisError::ArgumentTypeMismatch {
                            function_name: call_expr.name.clone(),
                            argument_position: i + 1,
                            expected: param_type.clone(),
                            actual: arg_type,
                            location: arg.location(),
                        });
                    }
                }
            }

            Ok(func_type.return_type.clone())
        } else {
            Err(SemanticAnalysisError::UndefinedFunction {
                name: call_expr.name.clone(),
                location: call_expr.location,
            })
        }
    }

    /// Visit a variable expression
    pub fn visit_variable_expression(&mut self, var_expr: &VariableExpr) -> SemanticResult {
        if let Some(var_info) = self.resolve_value(&var_expr.name) {
            Ok(var_info.type_id.clone())
        } else {
            Err(SemanticAnalysisError::UndefinedVariable {
                name: var_expr.name.clone(),
                location: var_expr.location,
            })
        }
    }

    /// Visit a literal expression
    pub fn visit_literal_expression(&mut self, literal_expr: &LiteralExpr) -> SemanticResult {
        Ok(literal_expr.expr_type.clone())
    }

    /// Visit a conditional expression
    pub fn visit_conditional_expression(&mut self, cond_expr: &ConditionalExpr) -> SemanticResult {
        let condition_type = self.visit_expression(&cond_expr.condition)?;
        if condition_type != TypeId::bool() {
            return Err(SemanticAnalysisError::TypeMismatch {
                expected: TypeId::bool(),
                actual: condition_type,
                context: Some("if condition".to_string()),
                location: cond_expr.condition.location(),
            });
        }

        let then_type = self.visit_expression(&cond_expr.then_branch)?;
        let else_type = self.visit_expression(&cond_expr.else_branch)?;

        if then_type == TypeId::unknown() {
            Ok(else_type)
        } else if else_type == TypeId::unknown() || then_type == else_type {
            Ok(then_type)
        } else {
            Err(SemanticAnalysisError::TypeMismatch {
                expected: then_type,
                actual: else_type,
                context: Some(
                    "conditional expression branches must have the same type".to_string(),
                ),
                location: cond_expr.location,
            })
        }
    }

    /// Visit a block expression
    pub fn visit_block_expression(&mut self, block_expr: &BlockExpr) -> SemanticResult {
        self.context.begin_scope();

        // Process all statements in the block
        for stmt in &block_expr.statements {
            // Create a statement visitor with the current return type context
            let mut stmt_visitor = super::statement_visitor::StatementVisitor::with_return_type(
                self.context, 
                self.current_return_type.clone()
            );
            match stmt {
                Statement::Let(let_stmt) => {
                    stmt_visitor.visit_let_statement(let_stmt)?;
                }
                Statement::Assignment(assign_stmt) => {
                    stmt_visitor.visit_assignment_statement(assign_stmt)?;
                }
                Statement::Expression(expr) => {
                    self.visit_expression(expr)?;
                }
                Statement::If(if_stmt) => {
                    stmt_visitor.visit_if_statement(if_stmt)?;
                }
                Statement::Return(return_stmt) => {
                    stmt_visitor.visit_return_statement(return_stmt)?;
                }
                Statement::FunctionDeclaration(fn_decl) => {
                    stmt_visitor.visit_function_declaration(fn_decl)?;
                }
                Statement::TypeDefinition(type_def) => {
                    stmt_visitor.visit_type_definition_statement(type_def)?;
                }
            }
        }

        let block_type = if let Some(return_expr) = &block_expr.return_expr {
            self.visit_expression(return_expr)?
        } else {
            TypeId::unit()
        };

        self.context.end_scope();

        Ok(block_type)
    }

    /// Visit a function type expression
    pub fn visit_function_type_expression(&mut self, func_type_expr: &FunctionTypeExpr) -> SemanticResult {
        // Validate all parameter types exist
        for param_type in &func_type_expr.param_types {
            if self.context.get_type_info(param_type).is_none() {
                return Err(SemanticAnalysisError::InvalidFieldType {
                    struct_name: "function type".to_string(),
                    field_name: "parameter".to_string(),
                    type_id: param_type.clone(),
                    location: func_type_expr.location,
                });
            }
        }

        // Validate return type exists
        if self.context.get_type_info(&func_type_expr.return_type).is_none() {
            return Err(SemanticAnalysisError::InvalidFieldType {
                struct_name: "function type".to_string(),
                field_name: "return type".to_string(),
                type_id: func_type_expr.return_type.clone(),
                location: func_type_expr.location,
            });
        }

        Ok(func_type_expr.expr_type.clone())
    }

    // Helper methods

    /// Resolve a symbol that can be used as a value (variables and functions)
    fn resolve_value(&self, name: &str) -> Option<&slang_shared::Symbol> {
        self.context.lookup_symbol(name)
            .filter(|symbol| matches!(symbol.kind(), SymbolKind::Variable | SymbolKind::Function))
    }
}
