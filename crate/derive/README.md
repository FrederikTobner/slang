# Slang Derive

This crate provides procedural macros for the Slang language compiler, offering derive macros and custom attributes to simplify common code patterns.

## Available Macros

### `NamedEnum` Derive Macro

The `NamedEnum` derive macro automatically generates `name()` and `from_str()` methods for enums based on attributes.

```rust
use slang_derive::NamedEnum;

#[derive(NamedEnum)]
pub enum PrimitiveType {
    #[name = "i32"]
    I32,
    #[name = "i64"]
    I64,
    #[name = "bool"]
    Bool,
    // Variants without attributes use their lowercase name
    String, // name = "string"
}
```

#### Generated Methods

1. `name(&self) -> &'static str`: Returns the string representation of the enum variant
2. `from_str(s: &str) -> Option<Self>`: Converts a string into the corresponding enum variant

### `NumericEnum` Derive Macro

The `NumericEnum` derive macro automatically generates bidirectional conversion methods between enum variants and their numeric values.

```rust
use slang_derive::NumericEnum;

#[derive(NumericEnum)]
pub enum OpCode {
    Constant = 0,     // Explicit value
    Add = 1,          // Explicit value
    Subtract = 2,     // Explicit value
    Multiply,         // Implicit value: 3
    Divide,           // Implicit value: 4
    // No need to manually implement numeric conversion!
}
```

#### Generated Methods

1. `from_int<T: Into<usize>>(value: T) -> Option<Self>`: Converts any numeric value into the corresponding enum variant

#### Key Advantages

- Bidirectional conversion between numeric values and enum variants
- No need to manually update conversion code when adding new enum variants
- Automatically handles both explicit and implicit discriminant values
- Ensures the conversion implementation always stays in sync with the enum definition
- Eliminates a common source of bugs when forgetting to update conversion code
- Works with any numeric type that can be converted to `usize` (u8, u16, i32, etc.)

## Usage

Add the derive crate to your dependencies:

```toml
[dependencies]
slang-derive = { path = "../derive" }
```

Then import and use the macros in your Rust code:

```rust
use slang_derive::NamedEnum;

#[derive(NamedEnum)]
pub enum DataType {
    #[name = "int"]
    Integer,
    #[name = "float"]
    Float,
    #[name = "string"]
    Text,
}

fn main() {
    assert_eq!(DataType::Integer.name(), "int");
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
- Uses the `#[name = "..."]` attribute to define custom string representations
- Falls back to lowercase variant name if no attribute is provided
- Generates both forward (enum → string) and reverse (string → enum) conversions

## Requirements

- Rust 2024 edition or later
- Dependencies: `syn`, `quote`, and `proc-macro2`
