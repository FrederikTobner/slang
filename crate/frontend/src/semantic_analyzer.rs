use std::collections::HashMap;

use crate::error::{CompileResult, CompilerError};
use crate::semantic_error::SemanticAnalysisError;

use slang_shared::{CompilationContext, SymbolKind};
use slang_ir::SourceLocation;
use slang_ir::Visitor;
use slang_ir::ast::{
    BinaryExpr, BinaryOperator, ConditionalExpr, Expression, FunctionCallExpr, FunctionDeclarationStmt,
    IfStatement, LetStatement, LiteralExpr, LiteralValue, Statement, TypeDefinitionStmt, UnaryExpr,
    UnaryOperator,
};
use slang_types::{PrimitiveType, TYPE_NAME_U32, TYPE_NAME_U64, TypeId};

/// Type alias for result of semantic analysis operations
/// Contains either a valid TypeId or a SemanticAnalysisError
pub type SemanticResult = Result<TypeId, SemanticAnalysisError>;

/// Information about a variable in a scope
#[derive(Clone)]
struct VariableInfo {
    /// Type ID of the variable
    type_id: TypeId,
    /// Whether the variable is mutable
    is_mutable: bool,
}

/// A scope represents a lexical scope for variables in the program.
/// Each scope contains a mapping of variable names to their types and mutability.
struct Scope {
    /// Map of variable names to their variable information in this scope
    variables: HashMap<String, VariableInfo>,
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

/// Performs semantic analysis including type checking on a list of statements.
/// This is the main entry point for the semantic analysis system.
///
/// ### Arguments
/// * `statements` - The AST statements to analyze
/// * `context` - The compilation context
///
/// ### Returns
/// * `CompileResult<()>` - Ok if no semantic errors were found, otherwise Err with the list of errors
pub fn execute(statements: &[Statement], context: &mut CompilationContext) -> CompileResult<()> {
    let mut analyzer = SemanticAnalyzer::new(context);
    analyzer.analyze(statements)
}

/// Performs semantic analysis including static type checking on the AST
pub struct SemanticAnalyzer<'a> {
    /// Map of variable names to their types
    scopes: Vec<Scope>,
    /// Map of function names to their parameter and return types
    functions: HashMap<String, FunctionSignature>,
    /// Current function's return type for validating return statements
    current_return_type: Option<TypeId>,
    /// Collected semantic errors
    errors: Vec<CompilerError>,
    /// Compilation context for type information and symbol table
    context: &'a mut CompilationContext,
}

/// Signature of a function, including its parameter types and return type
#[derive(Clone)]
struct FunctionSignature {
    /// List of parameter types
    param_types: Vec<TypeId>,
    /// Return type of the function
    return_type: TypeId,
}

impl<'a> SemanticAnalyzer<'a> {
    /// Creates a new semantic analyzer with built-in functions registered
    pub fn new(context: &'a mut CompilationContext) -> Self {
        let mut analyzer = SemanticAnalyzer {
            scopes: vec![Scope::new()],
            functions: HashMap::new(),
            current_return_type: None,
            errors: Vec::new(),
            context,
        };
        analyzer.register_native_functions();
        analyzer
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
    /// is_mutable - Whether the variable is mutable
    fn define_variable(&mut self, name: String, type_id: TypeId, is_mutable: bool) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.variables.insert(name, VariableInfo {
                type_id,
                is_mutable,
            });
        }
    }

    /// Looks up a variable in all scopes, starting from innermost
    ///
    /// ### Arguments
    /// name - The name of the variable to look up
    ///
    /// ### Returns
    /// The variable information if found, or None if not found
    fn resolve_variable(&self, name: &str) -> Option<VariableInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(var_info) = scope.variables.get(name) {
                return Some(var_info.clone());
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
        self.context.is_integer_type(type_id)
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
        self.context.is_float_type(type_id)
    }

    /// Checks if a type is an unsigned integer type
    ///
    /// ### Arguments
    /// * `type_id` - The type to check
    ///
    /// ### Returns
    /// * `true` if the type is u32 or u64, `false` otherwise
    fn is_unsigned_type(&self, type_id: &TypeId) -> bool {
        let type_name = self.context.get_type_name(type_id);
        type_name == TYPE_NAME_U64 || type_name == TYPE_NAME_U32
    }

    /// Registers the built-in native functions that are available to all programs.
    /// Currently only registers the `print_value` function that accepts any type
    /// and returns an i32.
    fn register_native_functions(&mut self) {
        self.functions.insert(
            "print_value".to_string(),
            FunctionSignature {
                param_types: vec![TypeId(PrimitiveType::Unknown as usize)],
                return_type: TypeId(PrimitiveType::I32 as usize),
            },
        );
    }

    /// Checks if types are compatible for logical operations (AND, OR).
    /// Both operands must be boolean types.
    ///
    /// ### Arguments
    /// * `left_type` - The type of the left operand
    /// * `right_type` - The type of the right operand
    /// * `operator` - The logical operator (either And or Or)
    /// * `location` - The source location of the operation
    ///
    /// ### Returns
    /// * `Ok(bool_type())` if both operands are boolean
    /// * `Err` with a descriptive error message otherwise
    fn check_logical_operation(
        &mut self,
        left_type: &TypeId,
        right_type: &TypeId,
        operator: &BinaryOperator,
        location: &SourceLocation,
    ) -> SemanticResult {
        if *left_type == TypeId(PrimitiveType::Bool as usize)
            && *right_type == TypeId(PrimitiveType::Bool as usize)
        {
            Ok(TypeId(PrimitiveType::Bool as usize))
        } else {
            Err(SemanticAnalysisError::LogicalOperatorTypeMismatch {
                operator: operator.to_string(),
                left_type: left_type.clone(),
                right_type: right_type.clone(),
                location: location.clone(),
            })
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
    /// * `location` - The source location of the operation
    ///
    /// ### Returns
    /// * `Ok(bool_type())` if the types are comparable
    /// * `Err` with a descriptive error message otherwise
    fn check_relational_operation(
        &mut self,
        left_type: &TypeId,
        right_type: &TypeId,
        operator: &BinaryOperator,
        location: &SourceLocation,
    ) -> SemanticResult {
        if left_type == right_type
            || (*left_type == TypeId(PrimitiveType::UnspecifiedInt as usize)
                && self.is_integer_type(right_type))
            || (*right_type == TypeId(PrimitiveType::UnspecifiedInt as usize)
                && self.is_integer_type(left_type))
            || (*left_type == TypeId(PrimitiveType::UnspecifiedFloat as usize)
                && self.is_float_type(right_type))
            || (*right_type == TypeId(PrimitiveType::UnspecifiedFloat as usize)
                && self.is_float_type(left_type))
        {
            Ok(TypeId(PrimitiveType::Bool as usize))
        } else {
            Err(SemanticAnalysisError::OperationTypeMismatch {
                operator: operator.to_string(),
                left_type: left_type.clone(),
                right_type: right_type.clone(),
                location: location.clone(),
            })
        }
    }

    /// Checks if a type is compatible with an arithmetic operation when both operands have the same type.
    /// Boolean types are not allowed for any arithmetic operation.
    /// String types are only allowed for the Add operator (concatenation).
    ///
    /// ### Arguments
    /// * `type_id` - The type of both operands
    /// * `operator` - The arithmetic operator (+, -, *, /)
    /// * `location` - The source location of the operation
    ///
    /// ### Returns
    /// * `Ok(type_id)` if the operation is allowed
    /// * `Err` with a descriptive error message otherwise
    fn check_same_type_arithmetic(
        &mut self,
        type_id: &TypeId,
        operator: &BinaryOperator,
        location: &SourceLocation,
    ) -> SemanticResult {
        if *type_id == TypeId(PrimitiveType::Bool as usize)
            || (operator != &BinaryOperator::Add
                && *type_id == TypeId(PrimitiveType::String as usize))
        {
            Err(SemanticAnalysisError::OperationTypeMismatch {
                operator: operator.to_string(),
                left_type: type_id.clone(),
                right_type: type_id.clone(),
                location: location.clone(),
            })
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
    ) -> SemanticResult {
        if let Expression::Unary(unary_expr) = expr {
            if unary_expr.operator == UnaryOperator::Negate {
                if let Expression::Literal(lit) = &*unary_expr.right {
                    if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                        if self.context.get_type_name(target_type) == "u32"
                            || self.context.get_type_name(target_type) == "u64"
                        {
                            return Err(SemanticAnalysisError::ValueOutOfRange {
                                value: format!("-{}", n),
                                target_type: target_type.clone(),
                                is_float: false,
                                location: expr.location(),
                            });
                        }
                    }
                }
            }
        }

        // Original check for regular literals
        if let Expression::Literal(lit) = expr {
            if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                let value_in_range = self.context.check_value_in_range(n, target_type);

                if value_in_range {
                    return Ok(target_type.clone());
                } else {
                    return Err(SemanticAnalysisError::ValueOutOfRange {
                        value: n.to_string(),
                        target_type: target_type.clone(),
                        is_float: false,
                        location: expr.location(),
                    });
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
    ) -> SemanticResult {
        if let Expression::Literal(lit) = expr {
            if let LiteralValue::UnspecifiedFloat(f) = &lit.value {
                let value_in_range = self.context.check_float_value_in_range(f, target_type);

                if value_in_range {
                    return Ok(target_type.clone());
                } else {
                    return Err(SemanticAnalysisError::ValueOutOfRange {
                        value: f.to_string(),
                        target_type: target_type.clone(),
                        is_float: true,
                        location: expr.location(),
                    });
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
    ) -> SemanticResult {
        if *left_type == TypeId(PrimitiveType::UnspecifiedInt as usize)
            && self.is_integer_type(right_type)
        {
            return self.check_unspecified_int_for_type(&bin_expr.left, right_type);
        }

        if *right_type == TypeId(PrimitiveType::UnspecifiedInt as usize)
            && self.is_integer_type(left_type)
        {
            return self.check_unspecified_int_for_type(&bin_expr.right, left_type);
        }

        if *left_type == TypeId(PrimitiveType::UnspecifiedFloat as usize)
            && self.is_float_type(right_type)
        {
            return self.check_unspecified_float_for_type(&bin_expr.left, right_type);
        }

        if *right_type == TypeId(PrimitiveType::UnspecifiedFloat as usize)
            && self.is_float_type(left_type)
        {
            return self.check_unspecified_float_for_type(&bin_expr.right, left_type);
        }

        if bin_expr.operator == BinaryOperator::Add
            && *left_type == TypeId(PrimitiveType::String as usize)
            && *right_type == TypeId(PrimitiveType::String as usize)
        {
            return Ok(TypeId(PrimitiveType::String as usize));
        }

        Err(SemanticAnalysisError::OperationTypeMismatch {
            operator: bin_expr.operator.to_string(),
            left_type: left_type.clone(),
            right_type: right_type.clone(),
            location: bin_expr.location.clone(),
        })
    }

    /// Checks if a variable is being redefined in the current scope.
    ///
    /// ### Arguments
    /// * `name` - The name of the variable
    /// * `location` - The source location of the variable definition
    ///
    /// ### Returns
    /// * `Ok(())` if the variable is not defined in the current scope
    /// * `Err(SemanticAnalysisError)` if the variable is already defined
    fn check_variable_redefinition(
        &self,
        name: &str,
        location: &SourceLocation,
    ) -> Result<(), SemanticAnalysisError> {
        if self.scopes.last().unwrap().variables.contains_key(name) {
            return Err(SemanticAnalysisError::VariableRedefinition {
                name: name.to_string(),
                location: location.clone(),
            });
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
        if type_id == TypeId(PrimitiveType::UnspecifiedInt as usize) {
            TypeId(PrimitiveType::I64 as usize)
        } else if type_id == TypeId(PrimitiveType::UnspecifiedFloat as usize) {
            TypeId(PrimitiveType::F64 as usize)
        } else {
            type_id
        }
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
    /// * `Err` with a SemanticAnalysisError if there's a type mismatch
    fn determine_let_statement_type(
        &mut self,
        let_stmt: &LetStatement,
        expr_type: TypeId,
    ) -> SemanticResult {
        if let_stmt.expr_type == TypeId(PrimitiveType::Unknown as usize) {
            return Ok(expr_type);
        }

        if let_stmt.expr_type == expr_type {
            if self.is_unsigned_type(&let_stmt.expr_type) {
                self.check_unspecified_int_for_type(&let_stmt.value, &let_stmt.expr_type)?;
            }
            return Ok(let_stmt.expr_type.clone());
        }

        if expr_type == TypeId(PrimitiveType::UnspecifiedInt as usize) {
            return self.handle_unspecified_int_assignment(let_stmt, &expr_type);
        }

        if expr_type == TypeId(PrimitiveType::UnspecifiedFloat as usize) {
            return self.handle_unspecified_float_assignment(let_stmt, &expr_type);
        }

        Err(SemanticAnalysisError::TypeMismatch {
            expected: let_stmt.expr_type.clone(),
            actual: expr_type,
            context: Some(let_stmt.name.clone()),
            location: let_stmt.location.clone(),
        })
    }

    /// Handles assignment of an unspecified integer literal to a variable with a declared type.
    ///
    /// ### Arguments
    /// * `let_stmt` - The let statement being analyzed.
    /// * `expr_type` - The type of the initialization expression (should be unspecified_int_type).
    ///
    /// ### Returns
    /// * `Ok(type_id)` with the declared type if the literal is valid for that type.
    /// * `Err` with a SemanticAnalysisError if there's a type mismatch or value out of range.
    fn handle_unspecified_int_assignment(
        &mut self,
        let_stmt: &LetStatement,
        _expr_type: &TypeId,
    ) -> SemanticResult {
        if self.is_integer_type(&let_stmt.expr_type) {
            self.check_unspecified_int_for_type(&let_stmt.value, &let_stmt.expr_type)
        } else {
            Err(SemanticAnalysisError::TypeMismatch {
                expected: let_stmt.expr_type.clone(),
                actual: TypeId(PrimitiveType::UnspecifiedInt as usize),
                context: Some(let_stmt.name.clone()),
                location: let_stmt.location.clone(),
            })
        }
    }

    /// Handles assignment of an unspecified float literal to a variable with a declared type.
    ///
    /// ### Arguments
    /// * `let_stmt` - The let statement being analyzed.
    /// * `expr_type` - The type of the initialization expression (should be unspecified_float_type).
    ///
    /// ### Returns
    /// * `Ok(type_id)` with the declared type if the literal is valid for that type.
    /// * `Err` with a SemanticAnalysisError if there's a type mismatch or value out of range.
    fn handle_unspecified_float_assignment(
        &mut self,
        let_stmt: &LetStatement,
        _expr_type: &TypeId,
    ) -> SemanticResult {
        if self.is_float_type(&let_stmt.expr_type) {
            self.check_unspecified_float_for_type(&let_stmt.value, &let_stmt.expr_type)
        } else {
            Err(SemanticAnalysisError::TypeMismatch {
                expected: let_stmt.expr_type.clone(),
                actual: TypeId(PrimitiveType::UnspecifiedFloat as usize),
                context: Some(let_stmt.name.clone()),
                location: let_stmt.location.clone(),
            })
        }
    }

    /// Internal version of analyze_return_expr_type that returns TypeCheckResult
    /// This allows us to gradually migrate the codebase to use SemanticAnalysisError directly
    ///
    /// ### Arguments
    /// * `expr` - The expression being returned
    /// * `expected_type` - The function's declared return type
    /// * `location` - The source location of the return statement
    ///
    /// ### Returns
    /// * `Ok(type_id)` if the expression type matches or can be coerced to the return type
    /// * `Err` with a SemanticAnalysisError if there's a type mismatch
    fn check_return_expr_type_internal(
        &mut self,
        expr: &Expression,
        expected_type: &TypeId,
        location: &SourceLocation,
    ) -> SemanticResult {
        let actual_type = self.visit_expression(expr)?;

        if actual_type == *expected_type {
            return Ok(actual_type);
        }

        if actual_type == TypeId(PrimitiveType::UnspecifiedInt as usize) {
            if let Expression::Literal(lit) = expr {
                if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                    let value_in_range = self.context.check_value_in_range(n, expected_type);

                    if value_in_range {
                        return Ok(expected_type.clone());
                    }
                }
            }
        }

        if actual_type == TypeId(PrimitiveType::UnspecifiedFloat as usize) {
            if let Expression::Literal(lit) = expr {
                if let LiteralValue::UnspecifiedFloat(f) = &lit.value {
                    let value_in_range = self.context.check_float_value_in_range(f, expected_type);

                    if value_in_range {
                        return Ok(expected_type.clone());
                    }
                }
            }
        }

        Err(SemanticAnalysisError::ReturnTypeMismatch {
            expected: expected_type.clone(),
            actual: actual_type,
            location: location.clone(),
        })
    }

    /// Performs semantic analysis on a list of statements by recursively analyzing the AST.
    /// This is the main entry point for semantic analysis within the SemanticAnalyzer struct.
    ///
    /// ### Arguments
    /// * `statements` - The AST statements to analyze
    ///
    /// ### Returns
    /// * `CompileResult<()>` - Ok if no semantic errors were found, otherwise Err with the list of errors
    ///
    /// This function performs static semantic analysis on the entire program, collecting all
    /// semantic errors before returning them as a single result.
    pub fn analyze(&mut self, statements: &[Statement]) -> CompileResult<()> {
        for stmt in statements {
            if let Err(error) = stmt.accept(self) {
                let compiler_error = error.to_compiler_error(self.context); // Pass self.context
                if !self.errors.iter().any(|e| {
                    e.message == compiler_error.message
                        && e.line == compiler_error.line
                        && e.column == compiler_error.column
                }) {
                    self.errors.push(compiler_error);
                }
            }
        }

        if !self.errors.is_empty() {
            Err(std::mem::take(&mut self.errors))
        } else {
            Ok(())
        }
    }
}

impl<'a> Visitor<SemanticResult> for SemanticAnalyzer<'a> {
    fn visit_statement(&mut self, stmt: &Statement) -> SemanticResult {
        match stmt {
            Statement::Let(let_stmt) => self.visit_let_statement(let_stmt),
            Statement::Assignment(assign_stmt) => self.visit_assignment_statement(assign_stmt),
            Statement::Expression(expr) => self.visit_expression_statement(expr),
            Statement::TypeDefinition(type_def) => self.visit_type_definition_statement(type_def),
            Statement::FunctionDeclaration(fn_decl) => {
                self.visit_function_declaration_statement(fn_decl)
            }
            Statement::Block(stmts) => self.visit_block_statement(stmts),
            Statement::Return(opt_expr) => self.visit_return_statement(opt_expr),
            Statement::If(if_stmt) => self.visit_if_statement(if_stmt),
        }
    }

    fn visit_function_declaration_statement(
        &mut self,
        fn_decl: &FunctionDeclarationStmt,
    ) -> SemanticResult {
        let mut param_types = Vec::new();
        for param in &fn_decl.parameters {
            param_types.push(param.param_type.clone());
        }
        self.functions.insert(
            fn_decl.name.clone(),
            FunctionSignature {
                param_types: param_types.clone(),
                return_type: fn_decl.return_type.clone(),
            },
        );
        let previous_return_type = self.current_return_type.clone();
        self.current_return_type = Some(fn_decl.return_type.clone());

        self.begin_scope();
        for param in &fn_decl.parameters {
            self.define_variable(param.name.clone(), param.param_type.clone(), true);
        }

        let result = self.visit_block_statement(&fn_decl.body);

        self.current_return_type = previous_return_type;
        self.end_scope();
        result.and(Ok(fn_decl.return_type.clone()))
    }

    fn visit_block_statement(&mut self, stmts: &[Statement]) -> SemanticResult {
        self.begin_scope();
        for stmt in stmts {
            if let Err(e) = stmt.accept(self) {
                return Err(e);
            }
        }

        self.end_scope();
        Ok(TypeId(PrimitiveType::Unknown as usize))
    }

    fn visit_return_statement(&mut self, opt_expr: &Option<Expression>) -> SemanticResult {
        let error_location = match opt_expr {
            Some(expr) => expr.location(),
            None => SourceLocation::default(),
        };

        if let Some(expected_type) = &self.current_return_type {
            let expected_type = expected_type.clone();
            if let Some(expr) = opt_expr {
                return self.check_return_expr_type_internal(
                    expr,
                    &expected_type,
                    &expr.location(),
                );
            } else if expected_type != TypeId(PrimitiveType::Unknown as usize) {
                return Err(SemanticAnalysisError::MissingReturnValue {
                    expected: expected_type.clone(),
                    location: error_location,
                });
            }

            Ok(expected_type)
        } else {
            Err(SemanticAnalysisError::ReturnOutsideFunction {
                location: error_location,
            })
        }
    }

    fn visit_call_expression(&mut self, call_expr: &FunctionCallExpr) -> SemanticResult {
        if let Some(function_semanitic) = self.functions.get(&call_expr.name).cloned() {
            if function_semanitic.param_types.len() != call_expr.arguments.len() {
                return Err(SemanticAnalysisError::ArgumentCountMismatch {
                    function_name: call_expr.name.clone(),
                    expected: function_semanitic.param_types.len(),
                    actual: call_expr.arguments.len(),
                    location: call_expr.location.clone(),
                });
            }

            for (i, arg) in call_expr.arguments.iter().enumerate() {
                let arg_type = self.visit_expression(arg)?;
                let param_type = &function_semanitic.param_types[i];

                if *param_type == TypeId(PrimitiveType::Unknown as usize) {
                    continue;
                }

                if arg_type != *param_type {
                    if arg_type == TypeId(PrimitiveType::UnspecifiedInt as usize) {
                        if let Err(_) = self.check_unspecified_int_for_type(arg, param_type) {
                            return Err(SemanticAnalysisError::ArgumentTypeMismatch {
                                function_name: call_expr.name.clone(),
                                argument_position: i + 1,
                                expected: param_type.clone(),
                                actual: arg_type,
                                location: arg.location(),
                            });
                        }
                        continue;
                    }

                    if arg_type == TypeId(PrimitiveType::UnspecifiedFloat as usize) {
                        if let Err(_) = self.check_unspecified_float_for_type(arg, param_type) {
                            return Err(SemanticAnalysisError::ArgumentTypeMismatch {
                                function_name: call_expr.name.clone(),
                                argument_position: i + 1,
                                expected: param_type.clone(),
                                actual: arg_type,
                                location: arg.location(),
                            });
                        }
                        continue;
                    }

                    return Err(SemanticAnalysisError::ArgumentTypeMismatch {
                        function_name: call_expr.name.clone(),
                        argument_position: i + 1,
                        expected: param_type.clone(),
                        actual: arg_type,
                        location: arg.location(),
                    });
                }
            }

            Ok(function_semanitic.return_type)
        } else {
            Err(SemanticAnalysisError::UndefinedFunction {
                name: call_expr.name.clone(),
                location: call_expr.location.clone(),
            })
        }
    }

    fn visit_type_definition_statement(&mut self, type_def: &TypeDefinitionStmt) -> SemanticResult {
        if self.context.lookup_symbol(&type_def.name).is_some() {
            return Err(SemanticAnalysisError::SymbolRedefinition {
                name: type_def.name.clone(),
                kind: "type".to_string(),
                location: type_def.location.clone(),
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
                    location: type_def.location.clone(),
                });
            }
            field_types_for_registration.push((name.clone(), type_id.clone()));
        }

        match self
            .context
            .register_struct_type(type_def.name.clone(), field_types_for_registration)
        {
            Ok(type_id) => Ok(type_id),
            Err(err_msg) => Err(SemanticAnalysisError::SymbolRedefinition {
                name: type_def.name.clone(),
                kind: format!("struct type (error: {})", err_msg),
                location: type_def.location.clone(),
            }),
        }
    }

    fn visit_expression_statement(&mut self, expr: &Expression) -> SemanticResult {
        self.visit_expression(expr)
    }

    fn visit_let_statement(&mut self, let_stmt: &LetStatement) -> SemanticResult {
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
                                location: let_stmt.location.clone(),
                            });
                        }
                    }
                }
            }
        }

        if let Err(e) = self.check_variable_redefinition(&let_stmt.name, &let_stmt.location) {
            return Err(e);
        }

        if let Some(symbol) = self.context.lookup_symbol(&let_stmt.name) {
            if symbol.kind == SymbolKind::Type {
                return Err(SemanticAnalysisError::SymbolRedefinition {
                    name: let_stmt.name.clone(),
                    kind: "variable (conflicts with type)".to_string(),
                    location: let_stmt.location.clone(),
                });
            } else if symbol.kind == SymbolKind::Function {
                return Err(SemanticAnalysisError::SymbolRedefinition {
                    name: let_stmt.name.clone(),
                    kind: "variable (conflicts with function)".to_string(),
                    location: let_stmt.location.clone(),
                });
            } else {
                return Err(SemanticAnalysisError::SymbolRedefinition {
                    name: let_stmt.name.clone(),
                    kind: "variable".to_string(),
                    location: let_stmt.location.clone(),
                });
            }
        }

        let expr_type = self.visit_expression(&let_stmt.value)?;
        let final_type = self.determine_let_statement_type(let_stmt, expr_type)?;
        let final_type = self.finalize_inferred_type(final_type);

        self.define_variable(let_stmt.name.clone(), final_type.clone(), let_stmt.is_mutable);
        Ok(final_type)
    }

    fn visit_assignment_statement(&mut self, assign_stmt: &slang_ir::ast::AssignmentStatement) -> SemanticResult {
        if let Some(var_info) = self.resolve_variable(&assign_stmt.name) {
            if !var_info.is_mutable {
                return Err(SemanticAnalysisError::AssignmentToImmutableVariable {
                    name: assign_stmt.name.clone(),
                    location: assign_stmt.location.clone(),
                });
            }

            let expr_type = self.visit_expression(&assign_stmt.value)?;
            
            if var_info.type_id == expr_type {
                Ok(var_info.type_id)
            } else {
                if expr_type == TypeId(slang_types::types::PrimitiveType::UnspecifiedInt as usize) {
                    Ok(var_info.type_id)
                } else if expr_type == TypeId(slang_types::types::PrimitiveType::UnspecifiedFloat as usize) {
                    Ok(var_info.type_id)
                } else {
                    Err(SemanticAnalysisError::TypeMismatch {
                        expected: var_info.type_id,
                        actual: expr_type,
                        context: Some(format!("assignment to variable '{}'", assign_stmt.name)),
                        location: assign_stmt.location.clone(),
                    })
                }
            }
        } else {
            Err(SemanticAnalysisError::UndefinedVariable {
                name: assign_stmt.name.clone(),
                location: assign_stmt.location.clone(),
            })
        }
    }

    fn visit_variable_expression(
        &mut self,
        name: &str,
        location: &SourceLocation,
    ) -> SemanticResult {
        if let Some(var_info) = self.resolve_variable(name) {
            Ok(var_info.type_id)
        } else if self.context.lookup_symbol(name).is_some() {
            Err(SemanticAnalysisError::UndefinedVariable {
                name: name.to_string(),
                location: location.clone(),
            })
        } else {
            Err(SemanticAnalysisError::UndefinedVariable {
                name: name.to_string(),
                location: location.clone(),
            })
        }
    }

    fn visit_literal_expression(&mut self, literal_expr: &LiteralExpr) -> SemanticResult {
        Ok(literal_expr.expr_type.clone())
    }

    fn visit_binary_expression(&mut self, bin_expr: &BinaryExpr) -> SemanticResult {
        let left_type = self.visit_expression(&bin_expr.left)?;
        let right_type = self.visit_expression(&bin_expr.right)?;

        if bin_expr.operator == BinaryOperator::And || bin_expr.operator == BinaryOperator::Or {
            let result = self.check_logical_operation(
                &left_type,
                &right_type,
                &bin_expr.operator,
                &bin_expr.location,
            );
            return result;
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
            return self.check_relational_operation(
                &left_type,
                &right_type,
                &bin_expr.operator,
                &bin_expr.location,
            );
        }

        if matches!(
            bin_expr.operator,
            BinaryOperator::Add
                | BinaryOperator::Subtract
                | BinaryOperator::Multiply
                | BinaryOperator::Divide
        ) {
            if left_type == right_type {
                return self.check_same_type_arithmetic(
                    &left_type,
                    &bin_expr.operator,
                    &bin_expr.location,
                );
            }

            return self.check_mixed_arithmetic_operation(&left_type, &right_type, bin_expr);
        }

        Err(SemanticAnalysisError::OperationTypeMismatch {
            operator: bin_expr.operator.to_string(),
            left_type: left_type.clone(),
            right_type: right_type.clone(),
            location: bin_expr.location.clone(),
        })
    }

    fn visit_unary_expression(&mut self, unary_expr: &UnaryExpr) -> SemanticResult {
        let operand_type = self.visit_expression(&unary_expr.right)?;

        match unary_expr.operator {
            UnaryOperator::Negate => {
                if operand_type == TypeId(PrimitiveType::UnspecifiedInt as usize) {
                    if let Expression::Literal(lit) = &*unary_expr.right {
                        if let slang_ir::ast::LiteralValue::UnspecifiedInteger(_value) = &lit.value
                        {
                            return Ok(TypeId(PrimitiveType::UnspecifiedInt as usize));
                        }
                        return Ok(TypeId(PrimitiveType::UnspecifiedFloat as usize));
                    }
                }

                if operand_type == TypeId(PrimitiveType::UnspecifiedFloat as usize) {
                    if let Expression::Literal(_) = &*unary_expr.right {
                        return Ok(TypeId(PrimitiveType::UnspecifiedFloat as usize));
                    }
                }

                let is_numeric =
                    self.is_integer_type(&operand_type) || self.is_float_type(&operand_type);

                if is_numeric {
                    if operand_type == TypeId(PrimitiveType::I32 as usize)
                        || operand_type == TypeId(PrimitiveType::I64 as usize)
                        || operand_type == TypeId(PrimitiveType::F32 as usize)
                        || operand_type == TypeId(PrimitiveType::F64 as usize)
                    {
                        return Ok(operand_type);
                    }

                    // For unsigned types, we need to reject negation entirely
                    if operand_type == TypeId(PrimitiveType::U32 as usize)
                        || operand_type == TypeId(PrimitiveType::U64 as usize)
                    {
                        // Attempting to negate an unsigned type
                        return Err(SemanticAnalysisError::InvalidUnaryOperation {
                            operator: "-".to_string(),
                            operand_type: operand_type.clone(),
                            location: unary_expr.location.clone(),
                        });
                    }
                }
                Err(SemanticAnalysisError::InvalidUnaryOperation {
                    operator: "-".to_string(),
                    operand_type: operand_type.clone(),
                    location: unary_expr.location.clone(),
                })
            }
            UnaryOperator::Not => {
                if operand_type == TypeId(PrimitiveType::Bool as usize) {
                    return Ok(TypeId(PrimitiveType::Bool as usize));
                }

                Err(SemanticAnalysisError::InvalidUnaryOperation {
                    operator: "!".to_string(),
                    operand_type: operand_type.clone(),
                    location: unary_expr.location.clone(),
                })
            }
        }
    }

    fn visit_expression(&mut self, expr: &Expression) -> SemanticResult {
        match expr {
            Expression::Literal(lit) => self.visit_literal_expression(lit),
            Expression::Variable(name, location) => self.visit_variable_expression(name, location),
            Expression::Binary(bin_expr) => self.visit_binary_expression(bin_expr),
            Expression::Unary(unary_expr) => self.visit_unary_expression(unary_expr),
            Expression::Call(call_expr) => self.visit_call_expression(call_expr),
            Expression::Conditional(cond_expr) => self.visit_conditional_expression(cond_expr),
        }
    }

    fn visit_conditional_expression(&mut self, cond_expr: &ConditionalExpr) -> SemanticResult {
        // Type check the condition - it must be a boolean
        let condition_type = self.visit_expression(&cond_expr.condition)?;
        if condition_type != TypeId(PrimitiveType::Bool as usize) {
            return Err(SemanticAnalysisError::TypeMismatch {
                expected: TypeId(PrimitiveType::Bool as usize),
                actual: condition_type,
                context: Some("if condition".to_string()),
                location: cond_expr.condition.location(),
            });
        }

        // Type check both branches
        let then_type = self.visit_expression(&cond_expr.then_branch)?;
        let else_type = self.visit_expression(&cond_expr.else_branch)?;

        // For expressions, both branches must have the same type or one must be unknown
        if then_type == TypeId(PrimitiveType::Unknown as usize) {
            Ok(else_type)
        } else if else_type == TypeId(PrimitiveType::Unknown as usize) {
            Ok(then_type)
        } else if then_type == else_type {
            Ok(then_type)
        } else {
            Err(SemanticAnalysisError::TypeMismatch {
                expected: then_type,
                actual: else_type,
                context: Some("conditional expression branches must have the same type".to_string()),
                location: cond_expr.location.clone(),
            })
        }
    }

    fn visit_if_statement(&mut self, if_stmt: &IfStatement) -> SemanticResult {
        // Type check the condition - it must be a boolean
        let condition_type = self.visit_expression(&if_stmt.condition)?;
        if condition_type != TypeId(PrimitiveType::Bool as usize) {
            return Err(SemanticAnalysisError::TypeMismatch {
                expected: TypeId(PrimitiveType::Bool as usize),
                actual: condition_type,
                context: Some("if condition".to_string()),
                location: if_stmt.condition.location(),
            });
        }

        // Visit the then branch
        self.begin_scope();
        self.visit_block_statement(&if_stmt.then_branch)?;
        self.end_scope();

        // Visit the optional else branch
        if let Some(else_branch) = &if_stmt.else_branch {
            self.begin_scope();
            self.visit_block_statement(else_branch)?;
            self.end_scope();
        }

        // If statements don't return a value
        Ok(TypeId(PrimitiveType::Unknown as usize))
    }
}
