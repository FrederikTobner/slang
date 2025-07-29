//! AST Factory System - Clean Design Implementation
//!
//! This module provides separate factories for expressions and statements,
//! following the Factory Method pattern with smart constructors.

pub mod expressions;
pub mod statements;
pub mod types;
pub mod locations;
pub mod traits;

pub use expressions::ExprFactory;
pub use statements::StmtFactory;
pub use traits::IntoLiteralValue;
pub use types::TypeInference;
pub use locations::LocationExtensions;
