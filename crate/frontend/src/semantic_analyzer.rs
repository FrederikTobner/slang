use crate::error::{CompileResult, CompilerError};
use crate::semantic_error::SemanticAnalysisError;

use slang_ir::SourceLocation;
use slang_ir::Visitor;
use slang_ir::ast::{
    BinaryExpr, BinaryOperator, BlockExpr, ConditionalExpr, Expression, FunctionCallExpr,
    FunctionDeclarationStmt, FunctionTypeExpr, IfStatement, LetStatement, LiteralExpr, LiteralValue, Statement,
    TypeDefinitionStmt, UnaryExpr, UnaryOperator,
};
use slang_shared::{CompilationContext, Symbol, SymbolKind};
use slang_types::{PrimitiveType, TYPE_NAME_U32, TYPE_NAME_U64, TypeId};

/// Type alias for result of semantic analysis operations
/// Contains either a valid TypeId or a SemanticAnalysisError
pub type SemanticResult = Result<TypeId, SemanticAnalysisError>;

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
    /// Current function's return type for validating return statements
    current_return_type: Option<TypeId>,
    /// Collected semantic errors
    errors: Vec<CompilerError>,
    /// Compilation context for type information and symbol table
    context: &'a mut CompilationContext,
}

impl<'a> SemanticAnalyzer<'a> {
    /// Creates a new semantic analyzer with built-in functions registered
    pub fn new(context: &'a mut CompilationContext) -> Self {
        let mut analyzer = SemanticAnalyzer {
            current_return_type: None,
            errors: Vec::new(),
            context,
        };
        analyzer.register_native_functions();
        analyzer
    }

    /// Begins a new scope by calling the compilation context
    /// Used when entering a block or function body.
    fn begin_scope(&mut self) {
        self.context.begin_scope();
    }

    /// Ends the current scope by calling the compilation context
    /// Used when exiting a block or function body.
    fn end_scope(&mut self) {
        self.context.end_scope();
    }

    /// Defines a variable in the current scope using the symbol table
    ///
    /// ### Arguments
    /// name - The name of the variable
    /// type_id - The type ID of the variable
    /// is_mutable - Whether the variable is mutable
    fn define_variable(&mut self, name: String, type_id: TypeId, is_mutable: bool) -> Result<(), String> {
        self.context.define_symbol(name, SymbolKind::Variable, type_id, is_mutable)
    }

    /// Looks up a variable in all scopes using the symbol table
    ///
    /// ### Arguments
    /// name - The name of the variable to look up
    ///
    /// ### Returns
    /// The symbol if found, or None if not found
    fn resolve_variable(&self, name: &str) -> Option<&Symbol> {
        self.context.lookup_symbol(name).filter(|symbol| symbol.kind() == SymbolKind::Variable)
    }

    /// Resolves a symbol that can be used as a value (variables and functions)
    /// This allows functions to be accessed as first-class values
    ///
    /// ### Arguments
    /// name - The name of the symbol to look up
    ///
    /// ### Returns
    /// The symbol if found, or None if not found
    fn resolve_value(&self, name: &str) -> Option<&Symbol> {
        self.context.lookup_symbol(name).filter(|symbol| {
            matches!(symbol.kind(), SymbolKind::Variable | SymbolKind::Function)
        })
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
        // Register print_value function
        let param_types = vec![TypeId(PrimitiveType::Unknown as usize)];
        let return_type = TypeId(PrimitiveType::I32 as usize);

        // Register as a function symbol in the symbol table
        let function_type_id = self.context.register_function_type(param_types, return_type);
        let _ = self.context.define_symbol("print_value".to_string(), SymbolKind::Function, function_type_id, false);
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
                location: *location,
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
        let is_relational = matches!(
            operator,
            BinaryOperator::GreaterThan
                | BinaryOperator::LessThan
                | BinaryOperator::GreaterThanOrEqual
                | BinaryOperator::LessThanOrEqual
        );

        if is_relational
            && (!PrimitiveType::from_int(left_type.0).is_some_and(|f| f.is_numeric())
                || !PrimitiveType::from_int(right_type.0).is_some_and(|f| f.is_numeric()))
        {
            return Err(SemanticAnalysisError::OperationTypeMismatch {
                operator: operator.to_string(),
                left_type: left_type.clone(),
                right_type: right_type.clone(),
                location: *location,
            });
        }

        if (left_type == right_type && *left_type != TypeId(PrimitiveType::Unit as usize))
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
                location: *location,
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
            || *type_id == TypeId(PrimitiveType::Unit as usize)
            || (operator != &BinaryOperator::Add
                && *type_id == TypeId(PrimitiveType::String as usize))
            || self.context.is_function_type(type_id)
        {
            Err(SemanticAnalysisError::OperationTypeMismatch {
                operator: operator.to_string(),
                left_type: type_id.clone(),
                right_type: type_id.clone(),
                location: *location,
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
            location: bin_expr.location,
        })
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

        if self.context.get_function_type(&let_stmt.expr_type).is_some() && 
           self.context.get_function_type(&expr_type).is_some() {
            if let_stmt.expr_type == expr_type {
                return Ok(let_stmt.expr_type.clone());
            } else {
                return Err(SemanticAnalysisError::TypeMismatch {
                    expected: let_stmt.expr_type.clone(),
                    actual: expr_type,
                    context: Some(let_stmt.name.clone()),
                    location: let_stmt.location,
                });
            }
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
            location: let_stmt.location,
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
                location: let_stmt.location,
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
                location: let_stmt.location,
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
            location: *location,
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
                let compiler_error = error.to_compiler_error(self.context);
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

impl Visitor<SemanticResult> for SemanticAnalyzer<'_> {
    fn visit_statement(&mut self, stmt: &Statement) -> SemanticResult {
        match stmt {
            Statement::Let(let_stmt) => self.visit_let_statement(let_stmt),
            Statement::Assignment(assign_stmt) => self.visit_assignment_statement(assign_stmt),
            Statement::Expression(expr) => self.visit_expression_statement(expr),
            Statement::TypeDefinition(type_def) => self.visit_type_definition_statement(type_def),
            Statement::FunctionDeclaration(fn_decl) => {
                self.visit_function_declaration_statement(fn_decl)
            }
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
        
        // Register function type and define function in symbol table
        let function_type_id = self.context.register_function_type(param_types.clone(), fn_decl.return_type.clone());
        
        // Define function symbol in the symbol table
        if let Err(_) = self.context.define_symbol(
            fn_decl.name.clone(), 
            SymbolKind::Function, 
            function_type_id, 
            false,
        ) {
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
            if let Err(_) = self.context.define_symbol(
                param.name.clone(), 
                SymbolKind::Variable, 
                param.param_type.clone(),
                true,
            ) {
                return Err(SemanticAnalysisError::SymbolRedefinition {
                    name: param.name.clone(),
                    kind: "parameter".to_string(),
                    location: fn_decl.location,
                });
            }
        }

        let result = self.visit_block_expression(&fn_decl.body);

        self.current_return_type = previous_return_type;
        self.context.end_scope();
        result.and(Ok(fn_decl.return_type.clone()))
    }

    fn visit_return_statement(&mut self, return_stmt: &slang_ir::ast::ReturnStatement) -> SemanticResult {
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

    fn visit_call_expression(&mut self, call_expr: &FunctionCallExpr) -> SemanticResult {
        // Look up the function in the symbol table and clone the function type to avoid borrowing conflicts
        let function_type = if let Some(symbol) = self.context.lookup_symbol(&call_expr.name) {
            // Check if it's a function symbol or a variable with function type
            match symbol.kind() {
                SymbolKind::Function => {
                    // Direct function symbol - get its function type
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
                    // Variable with function type
                    if self.context.is_function_type(&symbol.type_id) {
                        self.context.get_function_type(&symbol.type_id).cloned()
                    } else {
                        return Err(SemanticAnalysisError::UndefinedFunction {
                            name: call_expr.name.clone(),
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
            // Function not found in symbol table
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

                // Skip type checking for Unknown parameter types (like in print_value)
                if param_type == TypeId(PrimitiveType::Unknown as usize) {
                    continue;
                }

                if arg_type != param_type {
                    if arg_type == TypeId(PrimitiveType::UnspecifiedInt as usize) {
                        if self
                            .check_unspecified_int_for_type(arg, &param_type)
                            .is_err()
                        {
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
                        if self
                            .check_unspecified_float_for_type(arg, &param_type)
                            .is_err()
                        {
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
                        expected: param_type,
                        actual: arg_type,
                        location: arg.location(),
                    });
                }
            }

            return Ok(func_type.return_type.clone());
        }

        // This should not happen since we already checked above
        Err(SemanticAnalysisError::UndefinedFunction {
            name: call_expr.name.clone(),
            location: call_expr.location,
        })
    }

    fn visit_type_definition_statement(&mut self, type_def: &TypeDefinitionStmt) -> SemanticResult {
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

        match self
            .context
            .register_struct_type(type_def.name.clone(), field_types_for_registration)
        {
            Ok(type_id) => Ok(type_id),
            Err(_) => Err(SemanticAnalysisError::SymbolRedefinition {
                name: type_def.name.clone(),
                kind: "type".to_string(),
                location: type_def.location,
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
                                location: let_stmt.location,
                            });
                        }
                    }
                }
            }
        }

        // Check for conflicts with types and functions across all scopes
        // Variables are allowed to shadow other variables in outer scopes
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
            // Variable shadowing is allowed - don't return an error for variables in outer scopes
        }

        let expr_type = self.visit_expression(&let_stmt.value)?;
        let final_type = self.determine_let_statement_type(let_stmt, expr_type)?;
        let final_type = self.finalize_inferred_type(final_type);

        if let Err(_) = self.define_variable(
            let_stmt.name.clone(),
            final_type.clone(),
            let_stmt.is_mutable,
        ) {
            // The symbol table's define method only fails for same-scope redefinition
            // This should be a VariableRedefinition error (E2002)
            return Err(SemanticAnalysisError::VariableRedefinition {
                name: let_stmt.name.clone(),
                location: let_stmt.location,
            });
        }
        Ok(final_type)
    }

    fn visit_assignment_statement(
        &mut self,
        assign_stmt: &slang_ir::ast::AssignmentStatement,
    ) -> SemanticResult {
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

        // Now we can visit the expression since we no longer hold a reference to var_info
        let expr_type = self.visit_expression(&assign_stmt.value)?;

        if var_type_id == expr_type
            || expr_type == TypeId(slang_types::types::PrimitiveType::UnspecifiedInt as usize)
            || expr_type == TypeId(slang_types::types::PrimitiveType::UnspecifiedFloat as usize)
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

    fn visit_variable_expression(&mut self, var_expr: &slang_ir::ast::VariableExpr) -> SemanticResult {
        if let Some(var_info) = self.resolve_value(&var_expr.name) {
            Ok(var_info.type_id.clone())
        } else {
            Err(SemanticAnalysisError::UndefinedVariable {
                name: var_expr.name.clone(),
                location: var_expr.location,
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
            location: bin_expr.location,
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

                    if operand_type == TypeId(PrimitiveType::U32 as usize)
                        || operand_type == TypeId(PrimitiveType::U64 as usize)
                    {
                        return Err(SemanticAnalysisError::InvalidUnaryOperation {
                            operator: "-".to_string(),
                            operand_type: operand_type.clone(),
                            location: unary_expr.location,
                        });
                    }
                }
                Err(SemanticAnalysisError::InvalidUnaryOperation {
                    operator: "-".to_string(),
                    operand_type: operand_type.clone(),
                    location: unary_expr.location,
                })
            }
            UnaryOperator::Not => {
                if operand_type == TypeId(PrimitiveType::Bool as usize) {
                    return Ok(TypeId(PrimitiveType::Bool as usize));
                }

                Err(SemanticAnalysisError::InvalidUnaryOperation {
                    operator: "!".to_string(),
                    operand_type: operand_type.clone(),
                    location: unary_expr.location,
                })
            }
        }
    }

    fn visit_expression(&mut self, expr: &Expression) -> SemanticResult {
        match expr {
            Expression::Literal(lit) => self.visit_literal_expression(lit),
            Expression::Variable(var) => self.visit_variable_expression(var),
            Expression::Binary(bin_expr) => self.visit_binary_expression(bin_expr),
            Expression::Unary(unary_expr) => self.visit_unary_expression(unary_expr),
            Expression::Call(call_expr) => self.visit_call_expression(call_expr),
            Expression::Conditional(cond_expr) => self.visit_conditional_expression(cond_expr),
            Expression::Block(block_expr) => self.visit_block_expression(block_expr),
            Expression::FunctionType(func_type_expr) => self.visit_function_type_expression(func_type_expr),
        }
    }

    fn visit_conditional_expression(&mut self, cond_expr: &ConditionalExpr) -> SemanticResult {
        let condition_type = self.visit_expression(&cond_expr.condition)?;
        if condition_type != TypeId(PrimitiveType::Bool as usize) {
            return Err(SemanticAnalysisError::TypeMismatch {
                expected: TypeId(PrimitiveType::Bool as usize),
                actual: condition_type,
                context: Some("if condition".to_string()),
                location: cond_expr.condition.location(),
            });
        }

        let then_type = self.visit_expression(&cond_expr.then_branch)?;
        let else_type = self.visit_expression(&cond_expr.else_branch)?;

        if then_type == TypeId(PrimitiveType::Unknown as usize) {
            Ok(else_type)
        } else if else_type == TypeId(PrimitiveType::Unknown as usize) || then_type == else_type {
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

    fn visit_block_expression(&mut self, block_expr: &BlockExpr) -> SemanticResult {
        self.begin_scope();

        for stmt in &block_expr.statements {
            self.visit_statement(stmt)?;
        }

        let block_type = if let Some(return_expr) = &block_expr.return_expr {
            self.visit_expression(return_expr)?
        } else {
            TypeId(PrimitiveType::Unit as usize)
        };

        self.end_scope();

        Ok(block_type)
    }

    fn visit_function_type_expression(&mut self, func_type_expr: &FunctionTypeExpr) -> SemanticResult {
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

        if self.context.get_type_info(&func_type_expr.return_type).is_none() {
            return Err(SemanticAnalysisError::InvalidFieldType {
                struct_name: "function type".to_string(),
                field_name: "return type".to_string(),
                type_id: func_type_expr.return_type.clone(),
                location: func_type_expr.location,
            });
        }

        // Function type expressions evaluate to their own type
        Ok(func_type_expr.expr_type.clone())
    }

    fn visit_if_statement(&mut self, if_stmt: &IfStatement) -> SemanticResult {
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
}
