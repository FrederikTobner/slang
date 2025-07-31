//! Expression Factory - Clean construction of expression AST nodes
//!
//! This module provides the `ExprFactory` for creating all expression types
//! with automatic type inference and location management.

use crate::ast::{
    BinaryExpr, BinaryOperator, BlockExpr, ConditionalExpr, Expression, FunctionCallExpr,
    FunctionTypeExpr, LiteralExpr, Statement, UnaryExpr, UnaryOperator, VariableExpr,
};
use slang_error::location::Location;
use slang_types::types::TypeId;

use super::locations::LocationExtensions;
use super::traits::{IntoLiteralValue, LiteralTypeInference};
use super::types::TypeInference;

/// Factory for creating expression AST nodes
pub struct ExprFactory;

impl ExprFactory {
    /// Create a literal expression struct with explicit location
    /// 
    /// Returns the specific `LiteralExpr` type instead of the generic `Expression` enum.
    /// Use this when you need the specific type for further manipulation.
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::ExprFactory;
    /// use slang_error::location::Location;
    /// 
    /// let location = Location::new(0, 1, 1, 10);
    /// let literal = ExprFactory::literal_expr_with_location(42i32, location);
    /// ```
    #[inline(always)]
    pub fn literal_expr_with_location<T: IntoLiteralValue>(value: T, location: Location) -> LiteralExpr {
        let literal_value = value.into_literal_value();
        let expr_type = literal_value.infer_type();
        
        LiteralExpr {
            value: literal_value,
            expr_type,
            location,
        }
    }
    
    /// Create a variable reference expression struct with explicit location
    /// 
    /// Returns the specific `VariableExpr` type instead of the generic `Expression` enum.
    /// Use this when you need the specific type for further manipulation.
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::ExprFactory;
    /// use slang_error::location::Location;
    /// 
    /// let location = Location::new(0, 1, 1, 5);
    /// let var_ref = ExprFactory::variable_expr_with_location("my_var", location);
    /// ```
    #[inline(always)]
    pub fn variable_expr_with_location<S: Into<String>>(name: S, location: Location) -> VariableExpr {
        VariableExpr {
            name: name.into(),
            location,
        }
    }
    
    /// Create a function call expression struct with explicit location
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::{ExprFactory, ast::{Expression, LiteralExpr}};
    /// use slang_error::location::Location;
    /// use slang_types::TypeId;
    /// 
    /// let location = Location::new(0, 1, 1, 5);
    /// // Creating a function call: print(42)
    /// let arg = ExprFactory::literal_expr_with_location(42, location);
    /// let args = vec![Expression::Literal(arg)];
    /// let call = ExprFactory::call_expr_with_location("print", args, location);
    /// // Result is FunctionCallExpr that can be wrapped in Expression::Call if needed
    /// ```
    /// 
    /// Returns the specific `FunctionCallExpr` type instead of the generic `Expression` enum.
    /// Use this when you need the specific type for further manipulation.
    #[inline(always)]
    pub fn call_expr_with_location<S: Into<String>>(
        name: S, 
        arguments: Vec<Expression>, 
        location: Location
    ) -> FunctionCallExpr {
        FunctionCallExpr {
            name: name.into(),
            arguments,
            expr_type: TypeId::unknown(), // Will be resolved by semantic analysis
            location,
        }
    }
    
    /// Create a binary expression with automatic type inference
    /// 
    /// Returns the generic `Expression` enum.
    #[inline(always)]
    pub fn binary(left: Expression, operator: BinaryOperator, right: Expression) -> Expression {
        let location = left.location().span_to(&right.location());
        let expr_type = TypeInference::infer_binary_result(&left, &operator, &right)
            .unwrap_or_else(TypeId::unknown);
        
        Expression::Binary(BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            expr_type,
            location,
        })
    }

    /// Create a binary expression struct with automatic type inference
    /// 
    /// Returns the specific `BinaryExpr` type instead of the generic `Expression` enum.
    /// Use this when you need the specific type for further manipulation.
    #[inline(always)]
    pub fn binary_expr(left: Expression, operator: BinaryOperator, right: Expression) -> BinaryExpr {
        let location = left.location().span_to(&right.location());
        let expr_type = TypeInference::infer_binary_result(&left, &operator, &right)
            .unwrap_or_else(TypeId::unknown);
        
        BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            expr_type,
            location,
        }
    }

    /// Create a binary expression struct with explicit location
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::{ExprFactory, ast::{Expression, BinaryOperator}};
    /// use slang_error::location::Location;
    /// 
    /// let location = Location::new(0, 1, 1, 5);
    /// // Creating a binary expression: a + 5
    /// let left = ExprFactory::variable_expr_with_location("a", location);
    /// let right = ExprFactory::literal_expr_with_location(5, location);
    /// let binary = ExprFactory::binary_expr_with_location(
    ///     Expression::Variable(left), 
    ///     BinaryOperator::Add, 
    ///     Expression::Literal(right), 
    ///     location
    /// );
    /// // Result is BinaryExpr that can be wrapped in Expression::Binary if needed
    /// ```
    /// Create a binary expression struct with explicit location
    /// 
    /// Returns the specific `BinaryExpr` type instead of the generic `Expression` enum.
    /// Use this when you need the specific type for further manipulation.
    #[inline(always)]
    pub fn binary_expr_with_location(
        left: Expression,
        operator: BinaryOperator,
        right: Expression,
        location: Location,
    ) -> BinaryExpr {
        let expr_type = TypeInference::infer_binary_result(&left, &operator, &right)
            .unwrap_or_else(TypeId::unknown);
        
        BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            expr_type,
            location,
        }
    }
    
    /// Create a unary expression with automatic type inference
    /// 
    /// Returns the generic `Expression` enum.
    #[inline(always)]
    pub fn unary(operator: UnaryOperator, operand: Expression) -> Expression {
        let location = operand.location();
        let expr_type = TypeInference::infer_unary_result(&operator, &operand)
            .unwrap_or_else(|| operand.expr_type());
        
        Expression::Unary(UnaryExpr {
            operator,
            right: Box::new(operand),
            expr_type,
            location,
        })
    }
    
    /// Create a unary expression struct with automatic type inference
    /// 
    /// Returns the specific `UnaryExpr` type instead of the generic `Expression` enum.
    /// Use this when you need the specific type for further manipulation.
    #[inline(always)]
    pub fn unary_expr(operator: UnaryOperator, operand: Expression) -> UnaryExpr {
        let location = operand.location();
        let expr_type = TypeInference::infer_unary_result(&operator, &operand)
            .unwrap_or_else(|| operand.expr_type());
        
        UnaryExpr {
            operator,
            right: Box::new(operand),
            expr_type,
            location,
        }
    }

    /// Create a unary expression struct with explicit location
    /// 
    /// # Examples  
    /// ```rust
    /// use slang_ir::{ExprFactory, ast::{Expression, UnaryOperator}};
    /// use slang_error::location::Location;
    /// 
    /// let location = Location::new(0, 1, 1, 5);
    /// // Creating a unary expression: -x
    /// let operand_var = ExprFactory::variable_expr_with_location("x", location);
    /// let operand = Expression::Variable(operand_var);
    /// let unary = ExprFactory::unary_expr_with_location(UnaryOperator::Negate, operand, location);
    /// // Result is UnaryExpr that can be wrapped in Expression::Unary if needed
    /// ```
    /// Create a unary expression struct with explicit location
    /// 
    /// Returns the specific `UnaryExpr` type instead of the generic `Expression` enum.
    /// Use this when you need the specific type for further manipulation.
    #[inline(always)]
    pub fn unary_expr_with_location(
        operator: UnaryOperator,
        operand: Expression,
        location: Location,
    ) -> UnaryExpr {
        let expr_type = TypeInference::infer_unary_result(&operator, &operand)
            .unwrap_or_else(|| operand.expr_type());
        
        UnaryExpr {
            operator,
            right: Box::new(operand),
            expr_type,
            location,
        }
    }
    
    /// Create a conditional expression struct (ternary-like)
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::{ExprFactory, ast::Expression};
    /// use slang_error::location::Location;
    /// 
    /// let location = Location::new(0, 1, 1, 5);
    /// // Creating a conditional expression: flag ? 10 : 20
    /// let condition_var = ExprFactory::variable_expr_with_location("flag", location);
    /// let condition = Expression::Variable(condition_var);
    /// let then_lit = ExprFactory::literal_expr_with_location(10, location);
    /// let then_expr = Expression::Literal(then_lit);
    /// let else_lit = ExprFactory::literal_expr_with_location(20, location);
    /// let else_expr = Expression::Literal(else_lit);
    /// let conditional = ExprFactory::conditional_expr(condition, then_expr, else_expr);
    /// // Result is ConditionalExpr that can be wrapped in Expression::Conditional if needed
    /// ```
    /// Create a conditional expression struct (ternary-like)
    /// 
    /// Returns the specific `ConditionalExpr` type instead of the generic `Expression` enum.
    /// Use this when you need the specific type for further manipulation.
    #[inline(always)]
    pub fn conditional_expr(
        condition: Expression,
        then_branch: Expression,
        else_branch: Expression,
    ) -> ConditionalExpr {
        let location = condition.location()
            .span_to(&then_branch.location())
            .span_to(&else_branch.location());
            
        let expr_type = TypeInference::infer_common_type(&then_branch, &else_branch)
            .unwrap_or_else(TypeId::unknown);
        
        ConditionalExpr {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
            expr_type,
            location,
        }
    }

    /// Create a block expression struct with statements and optional return expression
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::{ExprFactory, ast::{Statement, Expression}};
    /// use slang_types::TypeId;
    /// use slang_error::location::Location;
    /// 
    /// let location = Location::new(0, 1, 1, 5);
    /// // Creating a block expression: { let x = 5; x }
    /// let let_stmt = ExprFactory::literal_expr_with_location(5, location);
    /// let statements = vec![]; // Simplified for doc test
    /// let return_var = ExprFactory::variable_expr_with_location("x", location);
    /// let return_expr = Some(Expression::Variable(return_var));
    /// let block = ExprFactory::block_expr(statements, return_expr);
    /// // Result is BlockExpr that can be wrapped in Expression::Block if needed
    /// ```
    #[inline(always)]
    pub fn block(statements: Vec<Statement>, return_expr: Option<Expression>) -> Expression {
        let location = Location::span_from_statements_and_expr(&statements, return_expr.as_ref());
        let expr_type = return_expr.as_ref()
            .map(|expr| expr.expr_type())
            .unwrap_or_else(TypeId::unit);
        
        Expression::Block(BlockExpr {
            statements,
            return_expr: return_expr.map(Box::new),
            expr_type,
            location,
        })
    }

    /// Create a block expression struct with statements and optional return expression
    /// 
    /// Returns the specific `BlockExpr` type instead of the generic `Expression` enum.
    /// Use this when you need the specific type for further manipulation.
    #[inline(always)]
    pub fn block_expr(statements: Vec<Statement>, return_expr: Option<Expression>) -> BlockExpr {
        let location = Location::span_from_statements_and_expr(&statements, return_expr.as_ref());
        let expr_type = return_expr.as_ref()
            .map(|expr| expr.expr_type())
            .unwrap_or_else(TypeId::unit);
        
        BlockExpr {
            statements,
            return_expr: return_expr.map(Box::new),
            expr_type,
            location,
        }
    }
    
    /// Create a function type expression with explicit location
    #[inline(always)]
    pub fn function_type_with_location(
        param_types: Vec<TypeId>, 
        return_type: TypeId, 
        location: Location
    ) -> Expression {
        let expr_type = TypeId::unknown(); // Function type construction happens in semantic analysis
        
        Expression::FunctionType(FunctionTypeExpr {
            param_types,
            return_type,
            expr_type,
            location,
        })
    }

    /// Create a function type expression struct with explicit location
    /// 
    /// # Examples
    /// ```rust
    /// use slang_ir::{ExprFactory, ast::Expression};
    /// use slang_error::location::Location;
    /// use slang_types::TypeId;
    /// 
    /// let location = Location::new(0, 1, 1, 5);
    /// // Creating a function type expression: (i32, str) -> i32
    /// let param_types = vec![TypeId::i32(), TypeId::string()];
    /// let return_type = TypeId::i32();
    /// let func_type = ExprFactory::function_type_expr_with_location(param_types, return_type, location);
    /// // Result is FunctionTypeExpr that can be wrapped in Expression::FunctionType if needed
    /// ```
    #[inline(always)]
    pub fn function_type_expr_with_location(
        param_types: Vec<TypeId>, 
        return_type: TypeId, 
        location: Location
    ) -> FunctionTypeExpr {
        let expr_type = TypeId::unknown(); // Function type construction happens in semantic analysis
        
        FunctionTypeExpr {
            param_types,
            return_type,
            expr_type,
            location,
        }
    }
}
