//! Heterogeneous List (HList) implementation for type-safe pipeline execution.
//!
//! This module provides the foundation for compile-time type-safe pipeline execution
//! using heterogeneous lists. HLists allow storing different types in a single structure
//! while maintaining complete type safety and enabling zero-cost abstractions.

use crate::error::StageError;
use crate::stage::{CompilationStage, StageContext};
use slang_shared::DiagnosticEngine;

/// Base trait for heterogeneous lists.
///
/// All HList types must be `Send + Sync` for thread safety and `'static`
/// for simplified lifetime management in the compilation pipeline.
pub trait HList: Send + Sync + 'static {}

/// Empty heterogeneous list (base case).
///
/// Represents the termination of an HList chain. Used as the tail
/// for all heterogeneous list constructions.
///
/// # Example
/// ```rust
/// use slang_compilation_pipeline::hlist::HNil;
///
/// let empty_list = HNil;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HNil;

impl HList for HNil {}

/// Cons cell for heterogeneous lists (recursive case).
///
/// Contains a head element of type `H` and a tail of type `T` (which must be an HList).
/// This creates a recursive structure that can store multiple different types.
///
/// # Type Parameters
/// - `H`: The type of the head element
/// - `T`: The type of the tail (must implement `HList`)
///
/// # Example
/// ```rust
/// use slang_compilation_pipeline::hlist::{HCons, HNil};
///
/// // Create HList with two different types
/// let list = HCons {
///     head: "string",
///     tail: HCons {
///         head: 42i32,
///         tail: HNil,
///     },
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HCons<H, T: HList> {
    pub head: H,
    pub tail: T,
}

impl<H: Send + Sync + 'static, T: HList> HList for HCons<H, T> {}

/// Type aliases for common HList sizes to improve ergonomics.
pub type HList1<A> = HCons<A, HNil>;
pub type HList2<A, B> = HCons<A, HList1<B>>;
pub type HList3<A, B, C> = HCons<A, HList2<B, C>>;
pub type HList4<A, B, C, D> = HCons<A, HList3<B, C, D>>;
pub type HList5<A, B, C, D, E> = HCons<A, HList4<B, C, D, E>>;

/// Construction helpers for HList types.
impl HNil {
    /// Create a new empty HList.
    #[inline]
    pub const fn new() -> Self {
        HNil
    }
}

impl Default for HNil {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<H, T: HList> HCons<H, T> {
    /// Create a new cons cell with head and tail.
    #[inline]
    pub const fn new(head: H, tail: T) -> Self {
        Self { head, tail }
    }
}

/// Trait for executing HList of compilation stages with complete type safety.
///
/// This trait enables type-safe execution of heterogeneous lists of stages.
/// Each implementation specifies the input type it accepts and the output
/// type it produces, enabling compile-time verification of stage compatibility.
///
/// # Type Parameters
/// - `'a`: Lifetime of the diagnostic engine and related data
/// - `Input`: The input type this HList can process
pub trait Execute<'a, Input> {
    /// The type produced after executing all stages in this HList.
    type Output;

    /// Execute all stages in this HList with the given input.
    ///
    /// # Arguments
    /// * `input` - The input value to process through the stages
    /// * `context` - Mutable stage context for shared state
    /// * `diagnostics` - Diagnostic engine for error reporting
    ///
    /// # Returns
    /// * `Ok(Self::Output)` - The final output after all stages execute successfully
    /// * `Err(StageError)` - Compilation failed with specific error type
    fn execute(
        &self,
        input: Input,
        context: &mut StageContext,
        diagnostics: &mut DiagnosticEngine<'a>,
    ) -> Result<Self::Output, StageError>;
}

/// Base case: empty HList passes input through unchanged.
///
/// This implementation handles the termination of recursive execution.
/// When we reach an empty list, we simply return the input as-is.
impl<'a, T> Execute<'a, T> for HNil {
    type Output = T;

    #[inline(always)]
    fn execute(
        &self,
        input: T,
        _context: &mut StageContext,
        _diagnostics: &mut DiagnosticEngine<'a>,
    ) -> Result<T, StageError> {
        Ok(input)
    }
}

/// Recursive case: execute head stage, then execute tail with head's output.
///
/// This is the core of the type-safe pipeline execution. The compiler
/// verifies that the head stage's output type matches the tail's input type.
/// If they don't match, compilation fails with a clear error message.
impl<'a, H, T, Input> Execute<'a, Input> for HCons<H, T>
where
    H: CompilationStage<Input = Input>,
    T: HList + Execute<'a, H::Output>,
    Input: 'static,
    H::Output: 'static,
{
    type Output = T::Output;

    #[inline] // Enable inlining for better performance
    fn execute(
        &self,
        input: Input,
        context: &mut StageContext,
        diagnostics: &mut DiagnosticEngine<'a>,
    ) -> Result<Self::Output, StageError> {
        // Execute head stage - types are guaranteed to match by the compiler!
        let intermediate = self.head.execute(input, context, diagnostics)?;

        // Execute tail with head's output - no type conversion needed!
        self.tail.execute(intermediate, context, diagnostics)
    }
}

/// Convenience macro for constructing HLists with a more natural syntax.
///
/// # Examples
/// ```rust
/// use slang_compilation_pipeline::hlist;
///
/// // Create empty HList
/// let empty = hlist![];
///
/// // Create HList with single element
/// let single = hlist![42];
///
/// // Create HList with multiple elements
/// let multi = hlist![42, "hello", true];
/// ```
#[macro_export]
macro_rules! hlist {
    () => { $crate::hlist::HNil::new() };
    ($head:expr) => {
        $crate::hlist::HCons::new($head, $crate::hlist::HNil::new())
    };
    ($head:expr, $($tail:expr),+ $(,)?) => {
        $crate::hlist::HCons::new($head, hlist!($($tail),+))
    };
}
