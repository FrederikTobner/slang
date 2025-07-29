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

/// Parser utilities for location creation
/// 
/// These utilities help with common location creation patterns in the parser
/// without requiring direct access to LineInfo structures.
pub trait ParserLocationUtils {
    /// Creates a Location from position and length data
    /// 
    /// ### Arguments
    /// 
    /// * `pos` - The starting position
    /// * `len` - The length of the location
    /// * `line_col_fn` - Function that converts position to (line, column)
    /// 
    /// ### Returns
    /// 
    /// A Location struct covering the specified range
    fn from_pos_with_line_info<F>(pos: usize, len: usize, line_col_fn: F) -> Self
    where
        F: Fn(usize) -> (usize, usize);

    /// Creates a Location spanning from one position to another
    /// 
    /// ### Arguments
    /// 
    /// * `start_pos` - The starting position
    /// * `end_pos` - The ending position (exclusive)
    /// * `line_col_fn` - Function that converts position to (line, column)
    /// 
    /// ### Returns
    /// 
    /// A Location struct covering the range from start_pos to end_pos
    fn from_range_with_line_info<F>(start_pos: usize, end_pos: usize, line_col_fn: F) -> Self
    where
        F: Fn(usize) -> (usize, usize);
}

impl ParserLocationUtils for Location {
    fn from_pos_with_line_info<F>(pos: usize, len: usize, line_col_fn: F) -> Self
    where
        F: Fn(usize) -> (usize, usize),
    {
        let (line, column) = line_col_fn(pos);
        Location::new(pos, line, column, len)
    }

    fn from_range_with_line_info<F>(start_pos: usize, end_pos: usize, line_col_fn: F) -> Self
    where
        F: Fn(usize) -> (usize, usize),
    {
        let (start_line, start_column) = line_col_fn(start_pos);
        let length = end_pos - start_pos;
        Location::new(start_pos, start_line, start_column, length)
    }
}
