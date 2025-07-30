//! Enhanced location management system
//!
//! This module provides utilities for automatic location calculation
//! and span management in AST construction.

use crate::ast::{Expression, Statement};
use crate::location::Location;

/// Extension trait to provide location access for AST nodes
pub trait AstLocation {
    fn location(&self) -> Location;
}

impl AstLocation for Statement {
    #[inline(always)]
    fn location(&self) -> Location {
        match self {
            Statement::Let(stmt) => stmt.location,
            Statement::Assignment(stmt) => stmt.location,
            Statement::Expression(expr) => expr.location(),
            Statement::TypeDefinition(stmt) => stmt.location,
            Statement::FunctionDeclaration(stmt) => stmt.location,
            Statement::Return(stmt) => stmt.location,
            Statement::If(stmt) => stmt.location,
        }
    }
}

impl AstLocation for Expression {
    #[inline(always)]
    fn location(&self) -> Location {
        self.location()
    }
}

/// Enhanced location management following DRY principles
/// 
/// # Design Principles Applied:
/// - **DRY**: Centralized location calculation logic
/// - **Single Responsibility**: Only handles location management
/// - **Type Safety**: Consistent location handling across all AST nodes
pub trait LocationExtensions {
    /// Create a span from this location to another location
    fn span_to(&self, other: &Self) -> Self;
    
    /// Create a location that spans multiple expressions
    fn span_from_expressions(expressions: &[Expression]) -> Self;
    
    /// Create a location that spans statements and optional expression
    fn span_from_statements_and_expr(statements: &[Statement], expr: Option<&Expression>) -> Self;
}

impl LocationExtensions for Location {
    #[inline(always)]
    fn span_to(&self, other: &Self) -> Self {
        let start_pos = self.position.min(other.position);
        let end_pos = self.end_position().max(other.end_position());

        Location {
            position: start_pos,
            line: self.line.min(other.line),
            column: if self.line == other.line {
                self.column.min(other.column)
            } else {
                self.column
            },
            length: end_pos - start_pos,
        }
    }
    
    #[inline(always)]
    fn span_from_expressions(expressions: &[Expression]) -> Self {
        if expressions.is_empty() {
            panic!("Cannot create span from empty expressions - this indicates improper usage");
        }
        
        let first = expressions.first().unwrap().location();
        let last = expressions.last().unwrap().location();
        first.span_to(&last)
    }
    
    #[inline(always)]
    fn span_from_statements_and_expr(statements: &[Statement], expr: Option<&Expression>) -> Self {
        match (statements.first(), statements.last(), expr) {
            (Some(first_stmt), _last_stmt, Some(expr)) => {
                let start = first_stmt.location();
                let end = expr.location();
                start.span_to(&end)
            }
            (Some(first_stmt), Some(last_stmt), None) => {
                first_stmt.location().span_to(&last_stmt.location())
            }
            (None, None, Some(expr)) => expr.location(),
            _ => {
                // For truly empty blocks, create a minimal valid location
                // This should ideally be provided by the caller, but for compatibility
                // we create a default location that won't cause issues
                Location::new_simple(0, 1, 1)
            },
        }
    }
}
