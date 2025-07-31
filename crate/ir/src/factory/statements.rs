//! Statement Factory - Clean construction of statement AST nodes
//!
//! This module provides the `StmtFactory` for creating all statement types
//! with automatic type inference and location management.

use crate::ast::{
    AssignmentStatement, Expression, FunctionDeclarationStmt, IfStatement, LetStatement,
    Parameter, ReturnStatement, TypeDefinitionStmt, BlockExpr,
};
use slang_error::location::Location;
use slang_types::types::TypeId;

/// Factory for creating statement AST nodes
pub struct StmtFactory;

impl StmtFactory {
    /// Create a mutable variable declaration statement struct with explicit location
    /// 
    /// Returns the specific `LetStatement` type instead of the generic `Statement` enum.
    /// Use this when you need the specific type for further manipulation.
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::{StmtFactory, ExprFactory, ast::Expression};
    /// use slang_error::location::Location;
    /// 
    /// let location = Location::new(0, 1, 1, 10);
    /// let loc = Location::new(0, 1, 1, 1);
    /// let literal = ExprFactory::literal_expr_with_location(0, loc);
    /// let expr = Expression::Literal(literal);
    /// let mut_var = StmtFactory::let_mut_stmt_with_location("counter", expr, location);
    /// ```
    #[inline(always)]
    pub fn let_mut_stmt_with_location<S: Into<String>>(
        name: S,
        value: Expression,
        location: Location,
    ) -> LetStatement {
        let expr_type = value.expr_type();
        
        LetStatement {
            name: name.into(),
            is_mutable: true,
            value,
            expr_type,
            location,
        }
    }
    
    /// Create a mutable typed variable declaration statement struct with explicit location
    /// 
    /// Returns the specific `LetStatement` type instead of the generic `Statement` enum.
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::{StmtFactory, ExprFactory, ast::Expression};
    /// use slang_error::location::Location;
    /// use slang_types::TypeId;
    /// 
    /// let location = Location::new(0, 1, 1, 15);
    /// let loc = Location::new(0, 1, 1, 1);
    /// let literal = ExprFactory::literal_expr_with_location(42, loc);
    /// let expr = Expression::Literal(literal);
    /// let typed_var = StmtFactory::let_mut_typed_stmt_with_location("result", TypeId::i32(), expr, location);
    /// ```
    #[inline(always)]
    pub fn let_mut_typed_stmt_with_location<S: Into<String>>(
        name: S, 
        var_type: TypeId, 
        value: Expression, 
        location: Location
    ) -> LetStatement {
        LetStatement {
            name: name.into(),
            is_mutable: true,
            value,
            expr_type: var_type,
            location,
        }
    }
    
    /// Create a variable declaration statement struct with explicit location
    /// 
    /// Returns the specific `LetStatement` type instead of the generic `Statement` enum.
    /// Use this when you need the specific type for further manipulation.
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::{StmtFactory, ExprFactory, ast::Expression};
    /// use slang_error::location::Location;
    ///
    /// let location = Location::new(0, 1, 1, 10);
    /// let loc = Location::new(0, 1, 1, 1);
    /// let literal = ExprFactory::literal_expr_with_location(42, loc);
    /// let expr = Expression::Literal(literal);
    /// let var = StmtFactory::let_var_stmt_with_location("result", expr, location);
    /// ```
    #[inline(always)]
    pub fn let_var_stmt_with_location<S: Into<String>>(
        name: S,
        value: Expression,
        location: Location,
    ) -> LetStatement {
        let expr_type = value.expr_type();
        
        LetStatement {
            name: name.into(),
            is_mutable: false,
            value,
            expr_type,
            location,
        }
    }
    
    /// Create a typed variable declaration statement struct with explicit type and location
    /// 
    /// Returns the specific `LetStatement` type instead of the generic `Statement` enum.
    /// Use this when you need the specific type for further manipulation.
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::{StmtFactory, ExprFactory, ast::Expression};
    /// use slang_error::location::Location;
    /// use slang_types::TypeId;
    /// 
    /// let location = Location::new(0, 1, 1, 15);
    /// let loc = Location::new(0, 1, 1, 1);
    /// let literal = ExprFactory::literal_expr_with_location(42, loc);
    /// let expr = Expression::Literal(literal);
    /// let typed_var = StmtFactory::let_typed_stmt_with_location("result", TypeId::i32(), expr, location);
    /// ```
    /// 
    /// # Design Principles Applied:
    /// - **Type Safety**: Explicit type specification with proper location tracking
    /// - **Location Accuracy**: Uses provided location for the entire statement
    #[inline(always)]
    pub fn let_typed_stmt_with_location<S: Into<String>>(
        name: S, 
        var_type: TypeId, 
        value: Expression,
        location: Location
    ) -> LetStatement {
        LetStatement {
            name: name.into(),
            is_mutable: false,
            value,
            expr_type: var_type,
            location,
        }
    }

    /// Create a variable assignment statement struct with explicit location
    /// 
    /// Returns the specific `AssignmentStatement` type instead of the generic `Statement` enum.
    /// Use this when you need the specific type for further manipulation.
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::{StmtFactory, ExprFactory, ast::Expression};
    /// use slang_error::location::Location;
    /// 
    /// let location = Location::new(0, 1, 1, 15);
    /// let loc = Location::new(0, 1, 1, 1);
    /// let literal = ExprFactory::literal_expr_with_location(100, loc);
    /// let expr = Expression::Literal(literal);
    /// let assignment = StmtFactory::assign_stmt_with_location("result", expr, location);
    /// ```
    #[inline(always)]
    pub fn assign_stmt_with_location<S: Into<String>>(
        name: S,
        value: Expression,
        location: Location,
    ) -> AssignmentStatement {
        AssignmentStatement {
            name: name.into(),
            value,
            location,
        }
    }
    
    /// Create a return statement struct with a value and explicit location
    /// 
    /// Returns the specific `ReturnStatement` type instead of the generic `Statement` enum.
    /// Use this when you need the specific type for further manipulation.
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::{StmtFactory, ExprFactory, ast::{Expression, BlockExpr}};
    /// use slang_error::location::Location;
    ///
    /// let location = Location::new(0, 1, 1, 10);
    /// let loc = Location::new(0, 1, 1, 1);
    /// let variable = ExprFactory::variable_expr_with_location("result", loc);
    /// let expr = Expression::Variable(variable);
    /// let return_stmt = StmtFactory::return_value_stmt_with_location(expr, location);
    /// ```
    #[inline(always)]
    pub fn return_value_stmt_with_location(value: Expression, location: Location) -> ReturnStatement {
        ReturnStatement {
            value: Some(value),
            location,
        }
    }
    
    /// Create a return statement struct without a value and explicit location
    /// 
    /// Returns the specific `ReturnStatement` type instead of the generic `Statement` enum.
    /// Use this when you need the specific type for further manipulation.
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::StmtFactory;
    /// use slang_error::location::Location;
    /// 
    /// let location = Location::new(0, 1, 1, 6);
    /// let return_stmt = StmtFactory::return_void_stmt_with_location(location);
    /// ```
    #[inline(always)]
    pub fn return_void_stmt_with_location(location: Location) -> ReturnStatement {
        ReturnStatement {
            value: None,
            location,
        }
    }
    
    /// Create a function declaration statement struct with explicit location
    /// 
    /// Returns the specific `FunctionDeclarationStmt` type instead of the generic `Statement` enum.
    /// Use this when you need the specific type for further manipulation.
    /// 
    /// # Type Safety
    /// This method accepts a `BlockExpr` directly instead of an `Expression`,
    /// eliminating the need for runtime panics and ensuring compile-time type safety.
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::{StmtFactory, ExprFactory, ast::{BlockExpr, Parameter}};
    /// use slang_types::TypeId;
    /// use slang_error::location::Location;
    /// 
    /// let location = Location::new(0, 1, 1, 20);
    /// let body = BlockExpr {
    ///     statements: vec![],
    ///     return_expr: None,
    ///     expr_type: TypeId::unit(),
    ///     location: Location::new(0, 1, 1, 1),
    /// };
    /// let func = StmtFactory::function_stmt_with_location("test", vec![], TypeId::unit(), body, location);
    /// ```
    #[inline(always)]
    pub fn function_stmt_with_location<S: Into<String>>(
        name: S,
        parameters: Vec<Parameter>,
        return_type: TypeId,
        body: BlockExpr,
        location: Location,
    ) -> FunctionDeclarationStmt {
        FunctionDeclarationStmt {
            name: name.into(),
            parameters,
            return_type,
            body,
            location,
        }
    }
    
    /// Create an if statement struct with explicit location
    /// 
    /// Returns the specific `IfStatement` type instead of the generic `Statement` enum.
    /// Use this when you need the specific type for further manipulation.
    /// 
    /// # Type Safety
    /// This method accepts `BlockExpr` directly for branches instead of `Expression`,
    /// eliminating the need for runtime panics and ensuring compile-time type safety.
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::{StmtFactory, ExprFactory, ast::{Expression, BlockExpr}};
    /// use slang_error::location::Location;
    /// use slang_types::TypeId;
    /// 
    /// let location = Location::new(0, 1, 1, 20);
    /// let literal = ExprFactory::literal_expr_with_location(true, Location::new(0, 1, 1, 1));
    /// let condition = Expression::Literal(literal);
    /// let then_block = BlockExpr {
    ///     statements: vec![],
    ///     return_expr: None,
    ///     expr_type: TypeId::unit(),
    ///     location: Location::new(0, 1, 1, 1),
    /// };
    /// let if_stmt = StmtFactory::if_stmt_with_location(condition, then_block, None, location);
    /// ```
    #[inline(always)]
    pub fn if_stmt_with_location(
        condition: Expression,
        then_branch: BlockExpr,
        else_branch: Option<BlockExpr>,
        location: Location,
    ) -> IfStatement {
        IfStatement {
            condition,
            then_branch,
            else_branch,
            location,
        }
    }
    
    /// Create a type definition statement struct with explicit location
    /// 
    /// Returns the specific `TypeDefinitionStmt` type instead of the generic `Statement` enum.
    /// Use this when you need the specific type for further manipulation.
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::StmtFactory;
    /// use slang_error::location::Location;
    /// use slang_types::TypeId;
    /// 
    /// let location = Location::new(0, 1, 1, 10);
    /// let type_def = StmtFactory::type_definition_stmt_with_location("Person", vec![("name".to_string(), TypeId::string())], location);
    /// ```
    #[inline(always)]
    pub fn type_definition_stmt_with_location<S: Into<String>>(
        name: S,
        fields: Vec<(String, TypeId)>,
        location: Location,
    ) -> TypeDefinitionStmt {
        TypeDefinitionStmt {
            name: name.into(),
            fields,
            location,
        }
    }
}
