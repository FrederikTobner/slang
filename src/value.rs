use std::fmt;
use crate::bytecode::Function;
use crate::bytecode::NativeFunction;
use std::io::Read;

pub trait ValueOperation 
where Self : Sized
{
    fn add(&self, other: &Self) -> Result<Self, String>;
    fn subtract(&self, other: &Self) -> Result<Self, String>;
    fn multiply(&self, other: &Self) -> Result<Self, String>;
    fn divide(&self, other: &Self) -> Result<Self, String>;
    fn negate(&self) -> Result<Self, String>;
    fn not(&self) -> Result<Self, String>;
    fn and(&self, other: &Self) -> Result<Self, String>;
    fn or(&self, other: &Self) -> Result<Self, String>;
    fn equal(&self, other: &Self) -> Result<Self, String>;
    fn not_equal(&self, other: &Self) -> Result<Self, String>;
    fn less_than(&self, other: &Self) -> Result<Self, String>;
    fn less_than_equal(&self, other: &Self) -> Result<Self, String>;
    fn greater_than(&self, other: &Self) -> Result<Self, String>;
    fn greater_than_equal(&self, other: &Self) -> Result<Self, String>;
}

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
    String(String),
    /// Boolean value
    Boolean(bool),
    /// Function value
    Function(Function),
    /// Native function value
    NativeFunction(NativeFunction),
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
        }
    }

    /// Deserialize a value from a reader based on its type tag
    /// 
    /// # Arguments
    /// 
    /// * `type_tag` - The type tag of the value
    /// * `reader` - The reader to read the value data from
    /// 
    /// # Returns
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
                    .map(Value::String)
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

                Ok(Value::Function(Function {
                    name,
                    arity,
                    code_offset,
                    locals,
                }))
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
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid value type tag",
            )),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::I32(i) => write!(f, "{}", i),
            Value::I64(i) => write!(f, "{}", i),
            Value::U32(i) => write!(f, "{}", i),
            Value::U64(i) => write!(f, "{}", i),
            Value::F32(flo) => write!(f, "{}", flo),
            Value::F64(flo) => write!(f, "{}", flo),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Function(func) => write!(f, "<fn {}>", func.name),
            Value::NativeFunction(func) => write!(f, "<native fn {}>", func.name),
        }
    }
}

impl ValueOperation for Value {
    fn add(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::I32(a), Value::I32(b)) => Ok(Value::I32(*a + *b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64(*a + *b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::U32(*a + *b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::U64(*a + *b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::F32(*a + *b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::F64(*a + *b)),
            (Value::String(a), Value::String(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            _ => Err("Cannot add these types".to_string()),
        }
    }
    fn subtract(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::I32(a), Value::I32(b)) => Ok(Value::I32(*a - *b)),
                    (Value::I64(a), Value::I64(b)) => Ok(Value::I64(*a - *b)),
                    (Value::U32(a), Value::U32(b)) => {
                        if *a >= *b {
                            Ok(Value::U32(*a - *b))
                        } else {
                            Err("Unsigned underflow".to_string())
                        }
                    }
                    (Value::U64(a), Value::U64(b)) => {
                        if *a >= *b {
                            Ok(Value::U64(*a - *b))
                        } else {
                            Err("Unsigned underflow".to_string())
                        }
                    }
                    (Value::F32(a), Value::F32(b)) => Ok(Value::F32(*a - *b)),
                    (Value::F64(a), Value::F64(b)) => Ok(Value::F64(*a - *b)),
                    _ => Err("Cannot subtract these types".to_string()),
        }
    }
    fn multiply(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::I32(a), Value::I32(b)) => match (*a).checked_mul(*b) {
                Some(result) => Ok(Value::I32(result)),
                None => Err("Integer overflow in I32 multiplication".to_string()),
            },
            (Value::I64(a), Value::I64(b)) => match (*a).checked_mul(*b) {
                Some(result) => Ok(Value::I64(result)),
                None => Err("Integer overflow in I64 multiplication".to_string()),
            },
            (Value::U32(a), Value::U32(b)) => match (*a).checked_mul(*b) {
                Some(result) => Ok(Value::U32(result)),
                None => Err("Integer overflow in U32 multiplication".to_string()),
            },
            (Value::U64(a), Value::U64(b)) => match (*a).checked_mul(*b) {
                Some(result) => Ok(Value::U64(result)),
                None => Err("Integer overflow in U64 multiplication".to_string()),
            },
            (Value::F32(a), Value::F32(b)) => Ok(Value::F32(*a * *b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::F64(*a * *b)),
            _ => Err("Cannot multiply these types".to_string()),
        }
    }
    fn divide(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::I32(a), Value::I32(b)) => {
                if *b == 0 {
                    return Err("Division by zero".to_string());
                }
                Ok(Value::I32(*a / *b))
            }
            (Value::I64(a), Value::I64(b)) => {
                if *b == 0 {
                    return Err("Division by zero".to_string());
                }
                Ok(Value::I64(*a / *b))
            }
            (Value::U32(a), Value::U32(b)) => {
                if *b == 0 {
                    return Err("Division by zero".to_string());
                }
                Ok(Value::U32(*a / *b))
            }
            (Value::U64(a), Value::U64(b)) => {
                if *b == 0 {
                    return Err("Division by zero".to_string());
                }
                Ok(Value::U64(*a / *b))
            }
            (Value::F32(a), Value::F32(b)) => {
                if *b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                Ok(Value::F32(*a / *b))
            }
            (Value::F64(a), Value::F64(b)) => {
                if *b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                Ok(Value::F64(*a / *b))
            }
            _ => Err("Cannot divide these types".to_string()),
        }
    }   

    fn negate(&self) -> Result<Value, String> {
        match self {
            Value::I32(i) => Ok(Value::I32(-i)),
            Value::I64(i) => Ok(Value::I64(-i)),
            Value::U32(_) => Err("Cannot negate unsigned integer U32".to_string()),
            Value::U64(_) => Err("Cannot negate unsigned integer U64".to_string()),
            Value::F32(f) => Ok(Value::F32(-f)),
            Value::F64(f) => Ok(Value::F64(-f)),
            _ => Err("Can only negate numbers".to_string()),
        }
    }

    fn not(&self) -> Result<Value, String> {
        match self {
            Value::Boolean(b) => Ok(Value::Boolean(!b)),
            _ => return Err("Can only negate boolean values".to_string()),
        }
    }

    fn and(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a && *b)),
            _ => Err("Logical AND operator requires boolean operands".to_string()),
        }
    }

    fn or(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a || *b)),
            _ => Err("Logical OR operator requires boolean operands".to_string()),
        }
    }

    fn equal(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::I32(a), Value::I32(b)) => Ok(Value::Boolean(a == b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::Boolean(a == b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::Boolean(a == b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::Boolean(a == b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::Boolean(a == b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::Boolean(a == b)),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a == b)),
            _ => Err("Cannot compare these types with ==".to_string()),
        }
    }

    fn not_equal(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::I32(a), Value::I32(b)) => Ok(Value::Boolean(a != b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::Boolean(a != b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::Boolean(a != b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::Boolean(a != b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::Boolean(a != b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::Boolean(a != b)),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a != b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a != b)),
            _ => Err("Cannot compare these types with !=".to_string()),
        }
    }

    fn less_than(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::I32(a), Value::I32(b)) => Ok(Value::Boolean(a < b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::Boolean(a < b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::Boolean(a < b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::Boolean(a < b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::Boolean(a < b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::Boolean(a < b)),
            _ => Err("Cannot compare these types with <".to_string()),
        }
    }

    fn less_than_equal(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::I32(a), Value::I32(b)) => Ok(Value::Boolean(a <= b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::Boolean(a <= b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::Boolean(a <= b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::Boolean(a <= b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::Boolean(a <= b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::Boolean(a <= b)),
            _ => Err("Cannot compare these types with <=".to_string()),
        }
    }

    fn greater_than(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::I32(a), Value::I32(b)) => Ok(Value::Boolean(a > b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::Boolean(a > b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::Boolean(a > b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::Boolean(a > b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::Boolean(a > b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::Boolean(a > b)),
            _ => Err("Cannot compare these types with >".to_string()),
        }
    }

    fn greater_than_equal(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::I32(a), Value::I32(b)) => Ok(Value::Boolean(a >= b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::Boolean(a >= b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::Boolean(a >= b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::Boolean(a >= b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::Boolean(a >= b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::Boolean(a >= b)),
            _ => Err("Cannot compare these types with >=".to_string()),
        }
    }

}