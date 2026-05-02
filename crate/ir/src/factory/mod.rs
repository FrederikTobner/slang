//! AST Factory System - Clean Design Implementation
//!
//! This module provides separate factories for expressions and statements,
//! following the Factory Method pattern with smart constructors.

pub mod expressions;
pub mod locations;
pub mod statements;
pub mod traits;
pub mod types;

pub use expressions::ExprFactory;
pub use locations::LocationExtensions;
pub use statements::StmtFactory;
pub use traits::IntoLiteralValue;
pub use types::TypeInference;
