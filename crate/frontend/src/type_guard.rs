use crate::error::{CompileResult, CompilerError};
use slang_ir::ast::{
    BinaryExpr, BinaryOperator, Expression, FunctionCallExpr, FunctionDeclarationStmt,
    LetStatement, LiteralExpr, LiteralValue, Statement, TypeDefinitionStmt, UnaryExpr,
    UnaryOperator,
};
use slang_ir::visitor::Visitor;
use slang_types::types::*;
use slang_types::types::{
    StructType, TYPE_REGISTRY, TypeId, TypeKind, get_type_name, type_fullfills,
};
use std::collections::HashMap;

/// A scope represents a lexical scope for variables in the program.
/// Each scope contains a mapping of variable names to their types.
struct Scope {
    /// Map of variable names to their type IDs in this scope
    variables: HashMap<String, TypeId>,
}

impl Scope {
    /// Creates a new empty scope.
    ///
    /// ### Returns
    /// A new Scope with no variables defined
    fn new() -> Self {
        Scope {
            variables: HashMap::new(),
        }
    }
}

/// Performs static type checking on a list of statements.
/// This is the main entry point for the type checking system.
///
/// ### Arguments
/// * `statements` - The AST statements to type check
///
/// ### Returns
/// * `CompileResult<()>` - Ok if no type errors were found, otherwise Err with the list of errors
pub fn execute(statements: &[Statement]) -> CompileResult<()> {
    let mut type_checker = TypeGuard::new();
    type_checker.check(statements)
}

/// Performs static type checking on the AST
pub struct TypeGuard {
    /// Map of variable names to their types
    scopes: Vec<Scope>,
    /// Map of function names to their parameter and return types
    functions: HashMap<String, (Vec<TypeId>, TypeId)>,
    /// Current function's return type for validating return statements
    current_return_type: Option<TypeId>,
    /// Collected type errors
    errors: Vec<CompilerError>,
}

impl Default for TypeGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeGuard {
    /// Creates a new type checker with built-in functions registered
    pub fn new() -> Self {
        let mut tc = TypeGuard {
            scopes: vec![Scope::new()],
            functions: HashMap::new(),
            current_return_type: None,
            errors: Vec::new(),
        };
        tc.register_native_functions();
        tc
    }

    /// Adds a compiler error to the collection with source location information.
    ///
    /// ### Arguments
    /// * `message` - The error message
    /// * `line` - The line number where the error occurred (1-based)
    /// * `column` - The column number where the error occurred (1-based)
    fn add_error(&mut self, message: String, line: usize, column: usize) {
        self.errors.push(CompilerError::new(message, line, column));
    }

    /// Converts a String error to a CompilerError and adds it to the collection.
    /// Currently uses placeholder location information since we don't track source locations
    /// for type errors yet.
    ///
    /// ### Arguments
    /// * `message` - The error message
    fn add_string_error(&mut self, message: String) {
        // Since we don't have line/column info in the current error handling,
        // temporarily use 0,0 - this will be improved in a future update
        self.add_error(message, 0, 0);
    }

    /// Begins a new scope by pushing a new scope onto the stack.
    /// Used when entering a block or function body.
    fn begin_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    /// Ends the current scope by popping it from the stack.
    /// Used when exiting a block or function body.
    ///
    /// ### Panics
    /// Panics if trying to end the global scope (i.e., if there's only one scope on the stack)
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
        for scope in self.scopes.iter().rev() {
            if let Some(type_id) = scope.variables.get(name) {
                return Some(type_id.clone());
            }
        }
        None
    }

    /// Checks if a type is an integer type
    ///
    /// ### Arguments
    /// type_id - The type ID to check
    ///
    /// ### Returns
    /// True if the type is an integer type, false otherwise
    ///
    fn is_integer_type(&self, type_id: &TypeId) -> bool {
        type_fullfills(type_id, |info| matches!(info.kind, TypeKind::Integer(_)))
    }

    /// Checks if a type is a float type
    ///
    /// ### Arguments
    /// type_id - The type ID to check
    ///
    /// ### Returns
    /// True if the type is a float type, false otherwise
    ///
    fn is_float_type(&self, type_id: &TypeId) -> bool {
        type_fullfills(type_id, |info| matches!(info.kind, TypeKind::Float(_)))
    }

    /// Registers the built-in native functions that are available to all programs.
    /// Currently only registers the `print_value` function that accepts any type
    /// and returns an i32.
    fn register_native_functions(&mut self) {
        self.functions.insert(
            "print_value".to_string(),
            (vec![unknown_type()], i32_type()),
        );
    }

    /// Checks if types are compatible for logical operations (AND, OR).
    /// Both operands must be boolean types.
    ///
    /// ### Arguments
    /// * `left_type` - The type of the left operand
    /// * `right_type` - The type of the right operand
    /// * `operator` - The logical operator (either And or Or)
    ///
    /// ### Returns
    /// * `Ok(bool_type())` if both operands are boolean
    /// * `Err` with a descriptive error message otherwise
    fn check_logical_operation(
        &mut self,
        left_type: &TypeId,
        right_type: &TypeId,
        operator: &BinaryOperator,
    ) -> Result<TypeId, String> {
        if *left_type == bool_type() && *right_type == bool_type() {
            Ok(bool_type())
        } else {
            Err(format!(
                "Logical operator '{}' requires boolean operands, got {} and {}",
                operator,
                get_type_name(left_type),
                get_type_name(right_type)
            ))
        }
    }

    /// Checks if types are compatible for relational operations (>, <, >=, <=, ==, !=).
    /// Types must be comparable with each other, which means they're either:
    /// - Exactly the same type
    /// - Unspecified integer literal and an integer type
    /// - Unspecified float literal and a float type
    ///
    /// ### Arguments
    /// * `left_type` - The type of the left operand
    /// * `right_type` - The type of the right operand
    /// * `operator` - The relational operator
    ///
    /// ### Returns
    /// * `Ok(bool_type())` if the types are comparable
    /// * `Err` with a descriptive error message otherwise
    fn check_relational_operation(
        &mut self,
        left_type: &TypeId,
        right_type: &TypeId,
        operator: &BinaryOperator,
    ) -> Result<TypeId, String> {
        if left_type == right_type
            || (left_type == &unspecified_int_type() && self.is_integer_type(right_type))
            || (right_type == &unspecified_int_type() && self.is_integer_type(left_type))
            || (left_type == &unspecified_float_type() && self.is_float_type(right_type))
            || (right_type == &unspecified_float_type() && self.is_float_type(left_type))
        {
            Ok(bool_type())
        } else {
            Err(format!(
                "Cannot compare different types with {}: {} and {}",
                operator,
                get_type_name(left_type),
                get_type_name(right_type)
            ))
        }
    }

    /// Checks if a type is compatible with an arithmetic operation when both operands have the same type.
    /// Boolean types are not allowed for any arithmetic operation.
    /// String types are only allowed for the Add operator (concatenation).
    ///
    /// ### Arguments
    /// * `type_id` - The type of both operands
    /// * `operator` - The arithmetic operator (+, -, *, /)
    ///
    /// ### Returns
    /// * `Ok(type_id)` if the operation is allowed
    /// * `Err` with a descriptive error message otherwise
    fn check_same_type_arithmetic(
        &mut self,
        type_id: &TypeId,
        operator: &BinaryOperator,
    ) -> Result<TypeId, String> {
        if *type_id == bool_type()
            || (operator != &BinaryOperator::Add && *type_id == string_type())
        {
            Err(format!(
                "Type mismatch: cannot apply '{}' operator on {} and {}",
                operator,
                get_type_name(type_id),
                get_type_name(type_id)
            ))
        } else {
            Ok(type_id.clone())
        }
    }

    /// Checks if an unspecified integer literal is in the valid range for a target type.
    /// This is used when coercing an integer literal to a specific integer type.
    ///
    /// ### Arguments
    /// * `expr` - The expression that might contain an unspecified integer literal
    /// * `target_type` - The specific integer type to check against
    ///
    /// ### Returns
    /// * `Ok(target_type)` if the literal is in range for the target type
    /// * `Err` with a descriptive error message if the literal is out of range
    /// * `Ok(target_type)` if the expression isn't an unspecified integer literal
    fn check_unspecified_int_for_type(
        &self,
        expr: &Expression,
        target_type: &TypeId,
    ) -> Result<TypeId, String> {
        if let Expression::Literal(lit) = expr {
            if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                let value_in_range = TYPE_REGISTRY
                    .read()
                    .unwrap()
                    .check_value_in_range(n, target_type);

                if value_in_range {
                    return Ok(target_type.clone());
                } else {
                    let type_name = get_type_name(target_type);
                    return Err(format!(
                        "Integer literal {} is out of range for type {}",
                        n, type_name
                    ));
                }
            }
        }
        Ok(target_type.clone())
    }

    /// Checks if an unspecified float literal is in the valid range for a target type.
    /// This is used when coercing a float literal to a specific floating-point type.
    ///
    /// ### Arguments
    /// * `expr` - The expression that might contain an unspecified float literal
    /// * `target_type` - The specific float type to check against (e.g., f32, f64)
    ///
    /// ### Returns
    /// * `Ok(target_type)` if the literal is in range for the target type
    /// * `Err` with a descriptive error message if the literal is out of range
    /// * `Ok(target_type)` if the expression isn't an unspecified float literal
    fn check_unspecified_float_for_type(
        &self,
        expr: &Expression,
        target_type: &TypeId,
    ) -> Result<TypeId, String> {
        if let Expression::Literal(lit) = expr {
            if let LiteralValue::UnspecifiedFloat(f) = &lit.value {
                let value_in_range = TYPE_REGISTRY
                    .read()
                    .unwrap()
                    .check_float_value_in_range(f, target_type);

                if value_in_range {
                    return Ok(target_type.clone());
                } else {
                    return Err(format!(
                        "Float literal {} is out of range for type {}",
                        f,
                        get_type_name(target_type)
                    ));
                }
            }
        }
        Ok(target_type.clone())
    }

    /// Checks if mixed-type arithmetic operations are allowed, particularly handling
    /// unspecified literals that can be coerced to match the other operand's type.
    /// Handles the following cases:
    /// - Unspecified integer literal + specific integer type
    /// - Unspecified float literal + specific float type
    /// - String concatenation with the + operator
    ///
    /// ### Arguments
    /// * `left_type` - The type of the left operand
    /// * `right_type` - The type of the right operand
    /// * `bin_expr` - The binary expression containing both operands and the operator
    ///
    /// ### Returns
    /// * `Ok(type_id)` with the resulting operation type if allowed
    /// * `Err` with a descriptive error message if the operation is not allowed
    fn check_mixed_arithmetic_operation(
        &mut self,
        left_type: &TypeId,
        right_type: &TypeId,
        bin_expr: &BinaryExpr,
    ) -> Result<TypeId, String> {
        if *left_type == unspecified_int_type() && self.is_integer_type(right_type) {
            return self.check_unspecified_int_for_type(&bin_expr.left, right_type);
        }

        if *right_type == unspecified_int_type() && self.is_integer_type(left_type) {
            return self.check_unspecified_int_for_type(&bin_expr.right, left_type);
        }

        if *left_type == unspecified_float_type() && self.is_float_type(right_type) {
            return self.check_unspecified_float_for_type(&bin_expr.left, right_type);
        }

        if *right_type == unspecified_float_type() && self.is_float_type(left_type) {
            return self.check_unspecified_float_for_type(&bin_expr.right, left_type);
        }

        if bin_expr.operator == BinaryOperator::Add
            && *left_type == string_type()
            && *right_type == string_type()
        {
            return Ok(string_type());
        }

        Err(format!(
            "Type mismatch: cannot apply '{}' operator on {} and {}",
            bin_expr.operator,
            get_type_name(left_type),
            get_type_name(right_type)
        ))
    }

    /// Checks if a variable is already defined in the current scope.
    /// Used to prevent variable redefinition errors.
    ///
    /// ### Arguments
    /// * `name` - The name of the variable to check
    ///
    /// ### Returns
    /// * `Ok(())` if the variable is not defined in the current scope
    /// * `Err` with a descriptive error message if the variable is already defined
    fn check_variable_redefinition(&self, name: &str) -> Result<(), String> {
        if self.scopes.last().unwrap().variables.contains_key(name) {
            return Err(format!("Variable '{}' already defined", name));
        }
        Ok(())
    }

    /// Converts unspecified literal types to concrete types.
    /// This is used to assign a default concrete type when an unspecified literal
    /// is used in a context where the type wasn't explicitly given.
    ///
    /// ### Arguments
    /// * `type_id` - The type to finalize
    ///
    /// ### Returns
    /// * The concrete type (i64 for unspecified integers, f64 for unspecified floats)
    /// * The original type if it wasn't an unspecified literal type
    fn finalize_inferred_type(&self, type_id: TypeId) -> TypeId {
        if type_id == unspecified_int_type() {
            i64_type()
        } else if type_id == unspecified_float_type() {
            f64_type()
        } else {
            type_id
        }
    }

    /// Checks type compatibility for a literal integer expression against a target type
    fn check_integer_literal_compatibility(
        &self,
        expr: &Expression,
        target_type: &TypeId,
        var_name: &str,
    ) -> Result<TypeId, String> {
        if let Expression::Literal(lit) = expr {
            if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                if self.is_integer_type(target_type) {
                    let value_in_range = TYPE_REGISTRY
                        .read()
                        .unwrap()
                        .check_value_in_range(n, target_type);

                    if value_in_range {
                        return Ok(target_type.clone());
                    } else {
                        return Err(format!(
                            "Integer literal {} is out of range for type {}",
                            n,
                            get_type_name(target_type)
                        ));
                    }
                } else {
                    return Err(format!(
                        "Type mismatch: variable {} is {} but expression is int",
                        var_name,
                        get_type_name(target_type)
                    ));
                }
            }
        }
        Err("Expected integer literal, got different expression type".to_string())
    }

    /// Checks if a literal float value is compatible with a target type.
    /// Used when assigning float literals to typed variables to ensure the value
    /// fits within the range of the target float type.
    ///
    /// ### Arguments
    /// * `expr` - The expression containing a potential float literal
    /// * `target_type` - The type to check compatibility with
    /// * `var_name` - The name of the variable being assigned to (for error messages)
    ///
    /// ### Returns
    /// * `Ok(target_type)` if the literal is compatible with the target type
    /// * `Err` with a descriptive error message if the literal is out of range or the target type is not a float type
    fn check_float_literal_compatibility(
        &self,
        expr: &Expression,
        target_type: &TypeId,
        var_name: &str,
    ) -> Result<TypeId, String> {
        if let Expression::Literal(lit) = expr {
            if let LiteralValue::UnspecifiedFloat(f) = &lit.value {
                if self.is_float_type(target_type) {
                    let value_in_range = TYPE_REGISTRY
                        .read()
                        .unwrap()
                        .check_float_value_in_range(f, target_type);

                    if value_in_range {
                        return Ok(target_type.clone());
                    } else {
                        return Err(format!(
                            "Float literal {} is out of range for type {}",
                            f,
                            get_type_name(target_type)
                        ));
                    }
                } else {
                    return Err(format!(
                        "Type mismatch: variable {} is {} but expression is float",
                        var_name,
                        get_type_name(target_type)
                    ));
                }
            }
        }
        Err("Expected float literal, got different expression type".to_string())
    }

    /// Checks if a negated literal value (like -42 or -3.14) is compatible with a target type.
    /// This is particularly important because negation can affect the range checks
    /// (e.g., negating the minimum value of a signed integer type could overflow).
    ///
    /// ### Arguments
    /// * `unary_expr` - The unary expression containing the negation operation
    /// * `target_type` - The type to check compatibility with
    /// * `var_name` - The name of the variable being assigned to (for error messages)
    ///
    /// ### Returns
    /// * `Ok(target_type)` if the negated literal is compatible with the target type
    /// * `Err` with a descriptive error message if the literal is out of range or the target type is incompatible
    fn check_negated_literal_compatibility(
        &self,
        unary_expr: &UnaryExpr,
        target_type: &TypeId,
        var_name: &str,
    ) -> Result<TypeId, String> {
        if unary_expr.operator == UnaryOperator::Negate {
            if let Expression::Literal(lit) = &*unary_expr.right {
                if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                    if self.is_integer_type(target_type) {
                        let negated_value = -*n;
                        let value_in_range = TYPE_REGISTRY
                            .read()
                            .unwrap()
                            .check_value_in_range(&negated_value, target_type);

                        if value_in_range {
                            return Ok(target_type.clone());
                        } else {
                            return Err(format!(
                                "Integer literal {} is out of range for type {}",
                                negated_value,
                                get_type_name(target_type)
                            ));
                        }
                    } else {
                        return Err(format!(
                            "Type mismatch: variable {} is {} but expression is negated int",
                            var_name,
                            get_type_name(target_type)
                        ));
                    }
                }

                if let LiteralValue::UnspecifiedFloat(f) = &lit.value {
                    if self.is_float_type(target_type) {
                        let negated_value = -*f;
                        let value_in_range = TYPE_REGISTRY
                            .read()
                            .unwrap()
                            .check_float_value_in_range(&negated_value, target_type);

                        if value_in_range {
                            return Ok(target_type.clone());
                        } else {
                            return Err(format!(
                                "Float literal {} is out of range for type {}",
                                negated_value,
                                get_type_name(target_type)
                            ));
                        }
                    } else {
                        return Err(format!(
                            "Type mismatch: variable {} is {} but expression is negated float",
                            var_name,
                            get_type_name(target_type)
                        ));
                    }
                }
            }
        }

        Err(format!(
            "Type mismatch: variable {} is {} but expression is incompatible",
            var_name,
            get_type_name(target_type)
        ))
    }

    /// Determines the final type of a variable in a let statement based on both the
    /// declared type (if any) and the initialization expression's type.
    /// Handles type inference and coercion of unspecified literals.
    ///
    /// ### Arguments
    /// * `let_stmt` - The let statement being analyzed
    /// * `expr_type` - The type of the initialization expression
    ///
    /// ### Returns
    /// * `Ok(type_id)` with the final determined type if valid
    /// * `Err` with a descriptive error message if there's a type mismatch
    fn determine_let_statement_type(
        &mut self,
        let_stmt: &LetStatement,
        expr_type: TypeId,
    ) -> Result<TypeId, String> {
        if let_stmt.expr_type == unknown_type() {
            return Ok(expr_type);
        }

        if let_stmt.expr_type == expr_type {
            return Ok(let_stmt.expr_type.clone());
        }

        if expr_type == unspecified_int_type() {
            return self.handle_unspecified_int_assignment(let_stmt, &expr_type);
        }

        if expr_type == unspecified_float_type() {
            return self.handle_unspecified_float_assignment(let_stmt, &expr_type);
        }

        self.type_mismatch_error(let_stmt, &expr_type)
    }

    /// Handles the type checking and coercion of unspecified integer literals in assignments.
    /// Performs different checks based on the expression type (literal, negated literal, binary expr).
    ///
    /// ### Arguments
    /// * `let_stmt` - The let statement containing the assignment
    /// * `expr_type` - The type of the expression (should be unspecified_int_type)
    ///
    /// ### Returns
    /// * `Ok(type_id)` with the target type if coercion is possible
    /// * `Err` with a descriptive error message if coercion fails
    fn handle_unspecified_int_assignment(
        &mut self,
        let_stmt: &LetStatement,
        expr_type: &TypeId,
    ) -> Result<TypeId, String> {
        match &let_stmt.value {
            Expression::Literal(_) => {
                match self.check_integer_literal_compatibility(
                    &let_stmt.value,
                    &let_stmt.expr_type,
                    &let_stmt.name,
                ) {
                    Ok(_) => Ok(let_stmt.expr_type.clone()),
                    Err(msg) => Err(msg),
                }
            }
            Expression::Unary(unary_expr) => {
                if unary_expr.operator == UnaryOperator::Negate {
                    self.check_negated_literal_compatibility(
                        unary_expr,
                        &let_stmt.expr_type,
                        &let_stmt.name,
                    )
                    .map(|_| let_stmt.expr_type.clone())
                } else {
                    self.type_mismatch_error(let_stmt, expr_type)
                }
            }
            Expression::Binary(_) => {
                if self.is_integer_type(&let_stmt.expr_type) {
                    Ok(let_stmt.expr_type.clone())
                } else {
                    self.type_mismatch_error(let_stmt, expr_type)
                }
            }
            _ => self.type_mismatch_error(let_stmt, expr_type),
        }
    }

    /// Handles the type checking and coercion of unspecified float literals in assignments.
    /// Performs different checks based on the expression type (literal, negated literal, binary expr).
    ///
    /// ### Arguments
    /// * `let_stmt` - The let statement containing the assignment
    /// * `expr_type` - The type of the expression (should be unspecified_float_type)
    ///
    /// ### Returns
    /// * `Ok(type_id)` with the target type if coercion is possible
    /// * `Err` with a descriptive error message if coercion fails
    fn handle_unspecified_float_assignment(
        &mut self,
        let_stmt: &LetStatement,
        expr_type: &TypeId,
    ) -> Result<TypeId, String> {
        match &let_stmt.value {
            Expression::Literal(_) => {
                match self.check_float_literal_compatibility(
                    &let_stmt.value,
                    &let_stmt.expr_type,
                    &let_stmt.name,
                ) {
                    Ok(_) => Ok(let_stmt.expr_type.clone()),
                    Err(msg) => Err(msg),
                }
            }
            Expression::Unary(unary_expr) => {
                if unary_expr.operator == UnaryOperator::Negate {
                    self.check_negated_literal_compatibility(
                        unary_expr,
                        &let_stmt.expr_type,
                        &let_stmt.name,
                    )
                    .map(|_| let_stmt.expr_type.clone())
                } else {
                    self.type_mismatch_error(let_stmt, expr_type)
                }
            }
            Expression::Binary(_) => {
                if self.is_float_type(&let_stmt.expr_type) {
                    Ok(let_stmt.expr_type.clone())
                } else {
                    self.type_mismatch_error(let_stmt, expr_type)
                }
            }
            _ => self.type_mismatch_error(let_stmt, expr_type),
        }
    }

    /// Generates a type mismatch error for a let statement.
    /// Creates a standardized error message format for type mismatches in variable assignments.
    ///
    /// ### Arguments
    /// * `let_stmt` - The let statement with the type mismatch
    /// * `expr_type` - The type of the expression that doesn't match the variable's declared type
    ///
    /// ### Returns
    /// * `Err` with a formatted error message describing the type mismatch
    fn type_mismatch_error(
        &self,
        let_stmt: &LetStatement,
        expr_type: &TypeId,
    ) -> Result<TypeId, String> {
        Err(format!(
            "Type mismatch: variable {} is {} but expression is {}",
            let_stmt.name,
            get_type_name(&let_stmt.expr_type),
            get_type_name(expr_type)
        ))
    }

    /// Checks if a return expression's type matches the expected return type of the function.
    /// Handles special cases for unspecified literals that can be coerced to the return type.
    ///
    /// ### Arguments
    /// * `expr` - The expression being returned
    /// * `expected_type` - The function's declared return type
    ///
    /// ### Returns
    /// * `Ok(type_id)` if the expression type matches or can be coerced to the return type
    /// * `Err` with a descriptive error message if there's a type mismatch
    fn check_return_expr_type(
        &mut self,
        expr: &Expression,
        expected_type: &TypeId,
    ) -> Result<TypeId, String> {
        let actual_type = self.visit_expression(expr)?;

        if actual_type == *expected_type {
            return Ok(actual_type);
        }

        if actual_type == unspecified_int_type() {
            if let Expression::Literal(lit) = expr {
                if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                    let value_in_range = TYPE_REGISTRY
                        .read()
                        .unwrap()
                        .check_value_in_range(n, expected_type);

                    if value_in_range {
                        return Ok(expected_type.clone());
                    }
                }
            }
        }

        if actual_type == unspecified_float_type() {
            if let Expression::Literal(lit) = expr {
                if let LiteralValue::UnspecifiedFloat(f) = &lit.value {
                    let value_in_range = TYPE_REGISTRY
                        .read()
                        .unwrap()
                        .check_float_value_in_range(f, expected_type);

                    if value_in_range {
                        return Ok(expected_type.clone());
                    }
                }
            }
        }

        Err(format!(
            "Type mismatch: function returns {} but got {}",
            get_type_name(expected_type),
            get_type_name(&actual_type)
        ))
    }

    /// Checks the type safety of a list of statements by recursively analyzing the AST.
    /// This is the main entry point for type checking within the TypeGuard struct.
    ///
    /// ### Arguments
    /// * `statements` - The AST statements to type check
    ///
    /// ### Returns
    /// * `CompileResult<()>` - Ok if no type errors were found, otherwise Err with the list of errors
    ///
    /// This function performs static type analysis on the entire program, collecting all
    /// type errors before returning them as a single result.
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

impl Visitor<Result<TypeId, String>> for TypeGuard {
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
        if let Some(expected_type) = &self.current_return_type {
            let expected_type = expected_type.clone();
            if let Some(expr) = expr {
                return self.check_return_expr_type(expr, &expected_type);
            } else if expected_type != unknown_type() {
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
        if let Some((param_types, return_type)) = self.functions.get(&call_expr.name).cloned() {
            if param_types.len() != call_expr.arguments.len() {
                return Err(format!(
                    "Function '{}' expects {} arguments, but got {}",
                    call_expr.name,
                    param_types.len(),
                    call_expr.arguments.len()
                ));
            }

            for (i, arg) in call_expr.arguments.iter().enumerate() {
                let arg_type = self.visit_expression(arg)?;
                let param_type = &param_types[i];

                // function that can accept any type(s)
                if param_type == &unknown_type() {
                    continue;
                }

                if arg_type != *param_type {
                    if arg_type == unspecified_int_type() {
                        if let Expression::Literal(lit) = arg {
                            if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                                let value_in_range = TYPE_REGISTRY
                                    .read()
                                    .unwrap()
                                    .check_value_in_range(n, param_type);

                                if value_in_range {
                                    continue;
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

            Ok(return_type)
        } else {
            Err(format!("Undefined function: {}", call_expr.name))
        }
    }

    fn visit_type_definition_statement(
        &mut self,
        type_def: &TypeDefinitionStmt,
    ) -> Result<TypeId, String> {
        let type_id = TYPE_REGISTRY.write().unwrap().register_type(
            &type_def.name,
            TypeKind::Struct(StructType {
                name: type_def.name.clone(),
                fields: type_def.fields.clone(),
            }),
        );
        Ok(type_id)
    }

    fn visit_expression_statement(&mut self, expr: &Expression) -> Result<TypeId, String> {
        self.visit_expression(expr)
    }

    fn visit_let_statement(&mut self, let_stmt: &LetStatement) -> Result<TypeId, String> {
        self.check_variable_redefinition(&let_stmt.name)?;

        let placeholder_type = let_stmt.expr_type.clone();
        self.define_variable(let_stmt.name.clone(), placeholder_type);

        let expr_type = self.visit_expression(&let_stmt.value)?;

        let mut final_type = self.determine_let_statement_type(let_stmt, expr_type)?;

        final_type = self.finalize_inferred_type(final_type);

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

        if bin_expr.operator == BinaryOperator::And || bin_expr.operator == BinaryOperator::Or {
            return self.check_logical_operation(&left_type, &right_type, &bin_expr.operator);
        }

        if matches!(
            bin_expr.operator,
            BinaryOperator::GreaterThan
                | BinaryOperator::LessThan
                | BinaryOperator::GreaterThanOrEqual
                | BinaryOperator::LessThanOrEqual
                | BinaryOperator::Equal
                | BinaryOperator::NotEqual
        ) {
            return self.check_relational_operation(&left_type, &right_type, &bin_expr.operator);
        }

        if matches!(
            bin_expr.operator,
            BinaryOperator::Add
                | BinaryOperator::Subtract
                | BinaryOperator::Multiply
                | BinaryOperator::Divide
        ) {
            if left_type == right_type {
                return self.check_same_type_arithmetic(&left_type, &bin_expr.operator);
            }

            return self.check_mixed_arithmetic_operation(&left_type, &right_type, bin_expr);
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
                if operand_type == unspecified_int_type() {
                    if let Expression::Literal(_) = &*unary_expr.right {
                        return Ok(unspecified_int_type());
                    }
                }

                let is_numeric = type_fullfills(&operand_type, |typeinfo| {
                    matches!(typeinfo.kind, TypeKind::Integer(_) | TypeKind::Float(_))
                });

                if is_numeric {
                    if operand_type == u32_type() || operand_type == u64_type() {
                        return Err("Cannot negate unsigned type".to_string());
                    }
                    return Ok(operand_type);
                }
                let type_name = get_type_name(&operand_type);
                Err(format!("Cannot negate non-numeric type '{}'", type_name))
            }
            UnaryOperator::Not => {
                if operand_type == bool_type() {
                    return Ok(bool_type());
                }

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
