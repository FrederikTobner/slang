pub mod operations;

use std::fmt;
use std::io::Read;

use crate::bytecode::Function;
use crate::bytecode::NativeFunction;

// Re-export the traits and combined trait for convenience
pub use operations::{ArithmeticOps, LogicalOps, ComparisonOps, ValueOperation};

/// Trait for types that can deserialize themselves from a reader
pub trait DeserializeFromReader: Sized {
    fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self>;
}

/// Trait for types that can display themselves as values
pub trait DisplayValue {
    fn display_value(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

// Implementations for basic types
impl DeserializeFromReader for i32 {
    fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes)?;
        Ok(i32::from_le_bytes(bytes))
    }
}

impl DeserializeFromReader for i64 {
    fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes)?;
        Ok(i64::from_le_bytes(bytes))
    }
}

impl DeserializeFromReader for u32 {
    fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }
}

impl DeserializeFromReader for u64 {
    fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }
}

impl DeserializeFromReader for f32 {
    fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes)?;
        Ok(f32::from_le_bytes(bytes))
    }
}

impl DeserializeFromReader for f64 {
    fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes)?;
        Ok(f64::from_le_bytes(bytes))
    }
}

impl DeserializeFromReader for bool {
    fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)?;
        Ok(byte[0] != 0)
    }
}

impl DeserializeFromReader for () {
    fn deserialize(_reader: &mut dyn Read) -> std::io::Result<Self> {
        Ok(())
    }
}

impl DeserializeFromReader for Box<String> {
    fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut len_bytes = [0u8; 4];
        reader.read_exact(&mut len_bytes)?;
        let len = u32::from_le_bytes(len_bytes) as usize;

        let mut string_bytes = vec![0u8; len];
        reader.read_exact(&mut string_bytes)?;
        String::from_utf8(string_bytes)
            .map(Box::new)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))
    }
}

impl DeserializeFromReader for Box<Function> {
    fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut name_len_bytes = [0u8; 4];
        reader.read_exact(&mut name_len_bytes)?;
        let name_len = u32::from_le_bytes(name_len_bytes) as usize;

        let mut name_bytes = vec![0u8; name_len];
        reader.read_exact(&mut name_bytes)?;
        let name = String::from_utf8(name_bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        let mut arity_bytes = [0u8; 1];
        reader.read_exact(&mut arity_bytes)?;
        let arity = arity_bytes[0];

        let mut code_offset_bytes = [0u8; 4];
        reader.read_exact(&mut code_offset_bytes)?;
        let code_offset = u32::from_le_bytes(code_offset_bytes) as usize;

        let mut locals_len_bytes = [0u8; 4];
        reader.read_exact(&mut locals_len_bytes)?;
        let locals_len = u32::from_le_bytes(locals_len_bytes) as usize;

        let mut locals = Vec::new();
        for _ in 0..locals_len {
            let local_string = Box::<String>::deserialize(reader)?;
            locals.push(*local_string);
        }

        Ok(Box::new(Function {
            name,
            arity,
            code_offset,
            locals,
        }))
    }
}

impl DeserializeFromReader for Box<NativeFunction> {
    fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self> {
        // For now, we'll just read the name since NativeFunction might have 
        // more complex deserialization requirements
        let name_string = Box::<String>::deserialize(reader)?;
        
        // Read arity
        let mut arity_bytes = [0u8; 1];
        reader.read_exact(&mut arity_bytes)?;
        let arity = arity_bytes[0];
        
        // For the function pointer, we can't really deserialize it from bytes,
        // so we'll use a placeholder function. In a real implementation, you might
        // have a registry of native functions that you look up by name.
        let placeholder_fn: fn(&[crate::value::Value]) -> Result<crate::value::Value, String> = 
            |_args| Err("Placeholder native function".to_string());
            
        Ok(Box::new(NativeFunction {
            name: *name_string,
            arity,
            function: placeholder_fn,
        }))
    }
}

// Display implementations
impl DisplayValue for i32 {
    fn display_value(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl DisplayValue for i64 {
    fn display_value(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl DisplayValue for u32 {
    fn display_value(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl DisplayValue for u64 {
    fn display_value(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl DisplayValue for f32 {
    fn display_value(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl DisplayValue for f64 {
    fn display_value(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl DisplayValue for bool {
    fn display_value(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl DisplayValue for () {
    fn display_value(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "()")
    }
}

impl DisplayValue for Box<String> {
    fn display_value(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl DisplayValue for Box<Function> {
    fn display_value(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}

impl DisplayValue for Box<NativeFunction> {
    fn display_value(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn {}>", self.name)
    }
}

// Macro to define the Value enum with automatic type tag management
macro_rules! define_value_enum {
    (
        $(
            $(#[doc = $doc:expr])*
            $variant:ident($type:ty) => $tag:expr,
        )*
    ) => {
        /// Values that can be stored in the bytecode and manipulated by the VM
        #[derive(Debug, Clone)]
        pub enum Value {
            $(
                $(#[doc = $doc])*
                $variant($type),
            )*
        }

        impl Value {
            /// Returns a tag byte identifying this value's type
            pub fn type_tag(&self) -> u8 {
                match self {
                    $(
                        Value::$variant(_) => $tag,
                    )*
                }
            }

            /// Deserialize a value from a reader based on its type tag
            ///
            /// ### Arguments
            ///
            /// * `type_tag` - The type tag of the value
            /// * `reader` - The reader to read the value data from
            ///
            /// ### Returns
            ///
            /// The deserialized value or an IO error
            pub fn deserialize_from_type_tag(type_tag: u8, reader: &mut dyn Read) -> std::io::Result<Self> {
                match type_tag {
                    $(
                        $tag => Ok(Value::$variant(<$type>::deserialize(reader)?)),
                    )*
                    _ => Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid value type tag: {}", type_tag),
                    )),
                }
            }
        }

        impl fmt::Display for Value {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        Value::$variant(value) => value.display_value(f),
                    )*
                }
            }
        }
    };
}

// Use the macro to define the Value enum with automatic type tag management
define_value_enum! {
    /// 32-bit signed integer
    I32(i32) => 0,
    /// 64-bit signed integer
    I64(i64) => 1,
    /// 32-bit unsigned integer
    U32(u32) => 2,
    /// 64-bit unsigned integer
    U64(u64) => 3,
    /// String value
    String(Box<String>) => 4,
    /// 64-bit floating point
    F64(f64) => 5,
    /// Function value
    Function(Box<Function>) => 6,
    /// Native function value
    NativeFunction(Box<NativeFunction>) => 7,
    /// 32-bit floating point
    F32(f32) => 8,
    /// Boolean value
    Boolean(bool) => 9,
    /// Unit value (similar to Rust's ())
    Unit(()) => 10,
}

impl Value {
    /// Check if the value is numeric (integer or float)
    pub fn is_numeric(&self) -> bool {
        matches!(self, Value::I32(_) | Value::I64(_) | Value::U32(_) | Value::U64(_) | Value::F32(_) | Value::F64(_))
    }

    /// Check if the value is an integer type
    pub fn is_integer(&self) -> bool {
        matches!(self, Value::I32(_) | Value::I64(_) | Value::U32(_) | Value::U64(_))
    }

    /// Check if the value is a float type
    pub fn is_float(&self) -> bool {
        matches!(self, Value::F32(_) | Value::F64(_))
    }

    /// Check if the value is a signed integer
    pub fn is_signed_integer(&self) -> bool {
        matches!(self, Value::I32(_) | Value::I64(_))
    }

    /// Check if the value is an unsigned integer
    pub fn is_unsigned_integer(&self) -> bool {
        matches!(self, Value::U32(_) | Value::U64(_))
    }

    /// Check if the value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Check if the value is a boolean
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }
}
