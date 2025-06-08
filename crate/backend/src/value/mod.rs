pub mod operations;

use std::fmt;
use std::io::Read;

use crate::bytecode::Function;
use crate::bytecode::NativeFunction;

// Re-export the traits and combined trait for convenience
pub use operations::{ArithmeticOps, LogicalOps, ComparisonOps, ValueOperation};

/// Values that can be stored in the bytecode and manipulated by the VM
#[derive(Debug, Clone)]
pub enum Value {
    /// 32-bit signed integer
    I32(i32),
    /// 64-bit signed integer
    I64(i64),
    /// 32-bit unsigned integer
    U32(u32),
    /// 64-bit unsigned integer
    U64(u64),
    /// 32-bit floating point
    F32(f32),
    /// 64-bit floating point
    F64(f64),
    /// String value
    String(Box<String>),
    /// Boolean value
    Boolean(bool),
    /// Unit value (similar to Rust's ())
    Unit,
    /// Function value
    Function(Box<Function>),
    /// Native function value
    NativeFunction(Box<NativeFunction>),
}

impl Value {
    /// Returns a tag byte identifying this value's type
    pub fn type_tag(&self) -> u8 {
        match self {
            Value::I32(_) => 0,
            Value::I64(_) => 1,
            Value::U32(_) => 2,
            Value::U64(_) => 3,
            Value::String(_) => 4,
            Value::F64(_) => 5,
            Value::Function(_) => 6,
            Value::NativeFunction(_) => 7,
            Value::F32(_) => 8,
            Value::Boolean(_) => 9,
            Value::Unit => 10,
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
            // I32
            0 => {
                let mut bytes = [0u8; 4];
                reader.read_exact(&mut bytes)?;
                Ok(Value::I32(i32::from_le_bytes(bytes)))
            }
            // I64
            1 => {
                let mut bytes = [0u8; 8];
                reader.read_exact(&mut bytes)?;
                Ok(Value::I64(i64::from_le_bytes(bytes)))
            }
            // U32
            2 => {
                let mut bytes = [0u8; 4];
                reader.read_exact(&mut bytes)?;
                Ok(Value::U32(u32::from_le_bytes(bytes)))
            }
            // U64
            3 => {
                let mut bytes = [0u8; 8];
                reader.read_exact(&mut bytes)?;
                Ok(Value::U64(u64::from_le_bytes(bytes)))
            }
            // String
            4 => {
                let mut len_bytes = [0u8; 4];
                reader.read_exact(&mut len_bytes)?;
                let len = u32::from_le_bytes(len_bytes) as usize;

                let mut string_bytes = vec![0u8; len];
                reader.read_exact(&mut string_bytes)?;
                String::from_utf8(string_bytes)
                    .map(|s| Value::String(Box::new(s)))
                    .map_err(|_| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8")
                    })
            }
            // F64
            5 => {
                let mut bytes = [0u8; 8];
                reader.read_exact(&mut bytes)?;
                Ok(Value::F64(f64::from_le_bytes(bytes)))
            }
            // Function
            6 => {
                let mut name_len_bytes = [0u8; 4];
                reader.read_exact(&mut name_len_bytes)?;
                let name_len = u32::from_le_bytes(name_len_bytes) as usize;

                let mut name_bytes = vec![0u8; name_len];
                reader.read_exact(&mut name_bytes)?;
                let name = String::from_utf8(name_bytes).map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8")
                })?;

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
                    let mut local_len_bytes = [0u8; 4];
                    reader.read_exact(&mut local_len_bytes)?;
                    let local_len = u32::from_le_bytes(local_len_bytes) as usize;

                    let mut local_bytes = vec![0u8; local_len];
                    reader.read_exact(&mut local_bytes)?;
                    let local = String::from_utf8(local_bytes).map_err(|_| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8")
                    })?;
                    locals.push(local);
                }

                Ok(Value::Function(Box::new(Function {
                    name,
                    arity,
                    code_offset,
                    locals,
                })))
            }
            // F32
            8 => {
                let mut bytes = [0u8; 4];
                reader.read_exact(&mut bytes)?;
                Ok(Value::F32(f32::from_le_bytes(bytes)))
            }
            // Boolean
            9 => {
                let mut byte = [0u8; 1];
                reader.read_exact(&mut byte)?;
                Ok(Value::Boolean(byte[0] != 0))
            }
            // Unit
            10 => Ok(Value::Unit),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid value type tag",
            )),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::I32(value) => write!(f, "{}", value),
            Value::I64(value) => write!(f, "{}", value),
            Value::U32(value) => write!(f, "{}", value),
            Value::U64(value) => write!(f, "{}", value),
            Value::F32(value) => write!(f, "{}", value),
            Value::F64(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value.as_ref()),
            Value::Boolean(value) => write!(f, "{}", value),
            Value::Unit => write!(f, "()"),
            Value::Function(function) => write!(f, "<fn {}>", function.name),
            Value::NativeFunction(function) => write!(f, "<native fn {}>", function.name),
        }
    }
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
