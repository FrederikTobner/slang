pub mod coercion;
pub mod coordinator;
pub mod inference;
pub mod type_checker;
pub mod type_validation;

pub use coercion::TypeCoercion;
pub use coordinator::TypeCheckingCoordinator;
pub use inference::TypeInference;
pub use type_checker::TypeChecker;
pub use type_validation::TypeValidation;
