# Slang Derive

This crate provides procedural macros for the Slang language compiler, offering derive macros and custom attributes to simplify common code patterns.

## Available Macros

### `TypeName` Derive Macro

The `TypeName` derive macro automatically generates `type_name()` and `from_str()` methods for enums based on attributes.

```rust
use slang_derive::TypeName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TypeName)]
pub enum PrimitiveType {
    #[type_name = "i32"]
    I32,
    #[type_name = "i64"]
    I64,
    #[type_name = "bool"]
    Bool,
    // Variants without attributes use their lowercase name
    String, // type_name = "string"
}
```

#### Generated Methods

1. `type_name(&self) -> &'static str`: Returns the string representation of the enum variant
2. `from_str(s: &str) -> Option<Self>`: Converts a string into the corresponding enum variant

## Usage

Add the derive crate to your dependencies:

```toml
[dependencies]
slang-derive = { path = "../derive" }
```

Then import and use the macros in your Rust code:

```rust
use slang_derive::TypeName;

#[derive(TypeName)]
pub enum DataType {
    #[type_name = "int"]
    Integer,
    #[type_name = "float"]
    Float,
    #[type_name = "string"]
    Text,
}

fn main() {
    assert_eq!(DataType::Integer.type_name(), "int");
    assert_eq!(DataType::from_str("float"), Some(DataType::Float));
    assert_eq!(DataType::from_str("unknown"), None);
}
```

## Benefits

- **Reduces Boilerplate**: Eliminates tedious match statements and manual string conversions
- **Single Source of Truth**: Type names are defined once, directly on the variants
- **Type Safety**: The macro ensures consistency between string representations and parsing logic
- **Maintainability**: Adding new variants automatically updates all related code

## Implementation Details

The `TypeName` derive macro:

- Works only on enums
- Uses the `#[type_name = "..."]` attribute to define custom string representations
- Falls back to lowercase variant name if no attribute is provided
- Generates both forward (enum → string) and reverse (string → enum) conversions

## Requirements

- Rust 2021 edition or later
- Dependencies: `syn`, `quote`, and `proc-macro2`
