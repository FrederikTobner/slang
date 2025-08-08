//! Macros for defining execution chain types and constructors
//!
//! This module contains declarative macros that eliminate boilerplate code
//! when defining ExecutionChain type aliases and their convenience constructors.

/// Macro to define execution chain type aliases with their stage combinations.
///
/// This macro eliminates the repetitive boilerplate of defining ExecutionChain type aliases
/// by automatically generating the appropriate HCons chain for the specified stages.
///
/// # Usage
/// ```
/// # use slang_compilation_pipeline::define_chain_types;
/// # use slang_compilation_pipeline::{ExecutionChain, HCons, HNil};
/// # struct MyStage1;
/// # struct MyStage2;
/// # struct Input;
/// # struct Output;
/// define_chain_types! {
///     /// Documentation for the chain type
///     ChainName(Input, Output): [MyStage1, MyStage2];
/// }
/// ```
///
/// # Example
/// ```
/// # use slang_compilation_pipeline::define_chain_types;
/// # use slang_compilation_pipeline::*;
/// # use slang_backend::bytecode::Chunk;
/// define_chain_types! {
///     /// Full compilation chain: SlangSourceFile → Chunk
///     FullCompilationChain(SlangSourceFile, Chunk): [CodeGenerationStage, SemanticAnalysisStage, ParsingStage, TokenizationStage];
/// }
/// ```
///
/// # Generated Code
/// The macro generates type aliases like:
/// ```rust
/// # use slang_compilation_pipeline::*;
/// # use slang_backend::bytecode::Chunk;
/// pub type FullCompilationChain = ExecutionChain<
///     SlangSourceFile,
///     Chunk,
///     HCons<CodeGenerationStage, HCons<SemanticAnalysisStage, HCons<ParsingStage, HCons<TokenizationStage, HNil>>>>
/// >;
/// ```
#[macro_export]
macro_rules! define_chain_types {
    (
        $(
            $(#[$meta:meta])*
            $name:ident($input:ty, $output:ty): [$($stage:ty),+ $(,)?];
        )*
    ) => {
        $(
            $(#[$meta])*
            pub type $name = $crate::execution_chain::ExecutionChain<
                $input,
                $output,
                $crate::define_chain_types!(@build_hcons [$($stage),+])
            >;
        )*
    };

    // Helper to build nested HCons types from a list of stages
    (@build_hcons [$stage:ty]) => {
        $crate::hlist::HCons<$stage, $crate::hlist::HNil>
    };
    (@build_hcons [$first:ty, $($rest:ty),+]) => {
        $crate::hlist::HCons<$first, $crate::define_chain_types!(@build_hcons [$($rest),+])>
    };
}

/// Macro to define convenience constructors for chain types.
///
/// This macro eliminates the repetitive impl blocks for chain constructors
/// by automatically generating the appropriate constructor methods.
///
/// # Usage
/// ```ignore
/// use slang_compilation_pipeline::define_chain_constructors;
/// use slang_compilation_pipeline::{ExecutionChain, define_chain_types};
///
/// define_chain_constructors! {
///     ChainName => method_name: [MyStage1, MyStage2];
/// }
/// ```
///
/// # Example
/// This macro is typically used in the execution_chain module:
/// ```ignore
/// define_chain_constructors! {
///     FullCompilationChain => full_compilation: [TokenizationStage, ParsingStage, SemanticAnalysisStage, CodeGenerationStage];
/// }
/// ```
///
/// # Generated Code
/// The macro generates impl blocks that provide convenient constructor methods:
/// ```rust
/// # use slang_compilation_pipeline::*;
/// # use slang_backend::bytecode::Chunk;
/// // For a type like this:
/// // pub type FullCompilationChain = ExecutionChain<SlangSourceFile, Chunk, ...>;
/// //
/// // The macro would generate:
/// // impl FullCompilationChain {
/// //     pub fn full_compilation() -> Self {
/// //         ExecutionChain::starting_with(TokenizationStage)
/// //             .then(ParsingStage)
/// //             .then(SemanticAnalysisStage)
/// //             .then(CodeGenerationStage)
/// //     }
/// // }
/// ```
#[macro_export]
macro_rules! define_chain_constructors {
    (
        $(
            $chain:ident => $method:ident: [$($stage:ident),+ $(,)?];
        )*
    ) => {
        $(
            impl $chain {
                #[doc = concat!("Create a ", stringify!($method), " chain.")]
                pub fn $method() -> Self {
                    $crate::define_chain_constructors!(@build_chain [$($stage),+])
                }
            }
        )*
    };

    // Helper to build the execution chain from stages (in logical order)
    (@build_chain [$stage:ident]) => {
        $crate::execution_chain::ExecutionChain::starting_with($stage)
    };
    (@build_chain [$first:ident, $($rest:ident),+]) => {
        $crate::execution_chain::ExecutionChain::starting_with($first)$(
            .then($rest)
        )+
    };
}
