pub mod type_checker;
pub mod coercion;
pub mod inference;
pub mod type_validation;
pub mod coordinator;

pub use type_checker::TypeChecker;
pub use coercion::TypeCoercion;
pub use inference::TypeInference;
pub use type_validation::TypeValidation;
pub use coordinator::TypeCheckingCoordinator;
