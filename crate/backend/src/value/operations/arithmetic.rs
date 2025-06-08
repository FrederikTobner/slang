use super::super::Value;

/// Arithmetic operations on values
pub trait ArithmeticOps {
    /// Adds two values and returns the result.
    ///
    /// ### Arguments
    /// * `other` - The other value to add
    ///
    /// ### Returns
    /// * The result of the addition
    /// * An error message if the types are incompatible
    fn add(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;

    /// Subtracts one value from another and returns the result.
    ///
    /// ### Arguments
    /// * `other` - The value to subtract
    ///
    /// ### Returns
    /// * The result of the subtraction
    /// * An error message if the types are incompatible or if an underflow occurs
    fn subtract(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;

    /// Multiplies two values and returns the result.
    ///
    /// ### Arguments
    /// * `other` - The other value to multiply
    ///
    /// ### Returns
    /// * The result of the multiplication
    /// * An error message if the types are incompatible or if an overflow occurs
    fn multiply(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;

    /// Divides one value by another and returns the result.
    ///
    /// ### Arguments
    /// * `other` - The value to divide by
    ///
    /// ### Returns
    /// * The result of the division
    /// * An error message if the types are incompatible or if division by zero occurs
    fn divide(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;

    /// Negates a value and returns the result.
    ///
    /// ### Returns
    /// * The negated value
    /// * An error message if the type is incompatible, or an overflow occurs
    fn negate(&self) -> Result<Self, String>
    where
        Self: Sized;
}

impl ArithmeticOps for Value {
    fn add(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            // Integer addition with overflow checking
            (Value::I32(a), Value::I32(b)) => match a.checked_add(*b) {
                Some(result) => Ok(Value::I32(result)),
                None => Err("Integer overflow in I32 addition".to_string()),
            },
            (Value::I64(a), Value::I64(b)) => match a.checked_add(*b) {
                Some(result) => Ok(Value::I64(result)),
                None => Err("Integer overflow in I64 addition".to_string()),
            },
            (Value::U32(a), Value::U32(b)) => match a.checked_add(*b) {
                Some(result) => Ok(Value::U32(result)),
                None => Err("Integer overflow in U32 addition".to_string()),
            },
            (Value::U64(a), Value::U64(b)) => match a.checked_add(*b) {
                Some(result) => Ok(Value::U64(result)),
                None => Err("Integer overflow in U64 addition".to_string()),
            },
            // Float addition with overflow checking
            (Value::F32(a), Value::F32(b)) => {
                let result = *a + *b;
                if result.is_infinite() && !a.is_infinite() && !b.is_infinite() {
                    Err("Floating point overflow in F32 addition".to_string())
                } else {
                    Ok(Value::F32(result))
                }
            }
            (Value::F64(a), Value::F64(b)) => {
                let result = *a + *b;
                if result.is_infinite() && !a.is_infinite() && !b.is_infinite() {
                    Err("Floating point overflow in F64 addition".to_string())
                } else {
                    Ok(Value::F64(result))
                }
            }
            // String concatenation
            (Value::String(a), Value::String(b)) => {
                Ok(Value::String(Box::new(format!("{}{}", a, b))))
            }
            _ => Err("Cannot add these types".to_string()),
        }
    }

    fn subtract(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::I32(a), Value::I32(b)) => match a.checked_sub(*b) {
                Some(result) => Ok(Value::I32(result)),
                None => Err("Integer underflow in I32 subtraction".to_string()),
            },
            (Value::I64(a), Value::I64(b)) => match a.checked_sub(*b) {
                Some(result) => Ok(Value::I64(result)),
                None => Err("Integer underflow in I64 subtraction".to_string()),
            },
            (Value::U32(a), Value::U32(b)) => match a.checked_sub(*b) {
                Some(result) => Ok(Value::U32(result)),
                None => Err("Integer underflow in U32 subtraction".to_string()),
            },
            (Value::U64(a), Value::U64(b)) => match a.checked_sub(*b) {
                Some(result) => Ok(Value::U64(result)),
                None => Err("Integer underflow in U64 subtraction".to_string()),
            },
            (Value::F32(a), Value::F32(b)) => {
                let result = *a - *b;
                if result.is_infinite() && !a.is_infinite() && !b.is_infinite() {
                    Err("Floating point overflow/underflow in F32 subtraction".to_string())
                } else {
                    Ok(Value::F32(result))
                }
            }
            (Value::F64(a), Value::F64(b)) => {
                let result = *a - *b;
                if result.is_infinite() && !a.is_infinite() && !b.is_infinite() {
                    Err("Floating point overflow/underflow in F64 subtraction".to_string())
                } else {
                    Ok(Value::F64(result))
                }
            }
            _ => Err("Cannot subtract these types".to_string()),
        }
    }

    fn multiply(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::I32(a), Value::I32(b)) => match a.checked_mul(*b) {
                Some(result) => Ok(Value::I32(result)),
                None => Err("Integer overflow in I32 multiplication".to_string()),
            },
            (Value::I64(a), Value::I64(b)) => match a.checked_mul(*b) {
                Some(result) => Ok(Value::I64(result)),
                None => Err("Integer overflow in I64 multiplication".to_string()),
            },
            (Value::U32(a), Value::U32(b)) => match a.checked_mul(*b) {
                Some(result) => Ok(Value::U32(result)),
                None => Err("Integer overflow in U32 multiplication".to_string()),
            },
            (Value::U64(a), Value::U64(b)) => match a.checked_mul(*b) {
                Some(result) => Ok(Value::U64(result)),
                None => Err("Integer overflow in U64 multiplication".to_string()),
            },
            (Value::F32(a), Value::F32(b)) => {
                let result = *a * *b;
                if result.is_infinite() && !a.is_infinite() && !b.is_infinite() {
                    Err("Floating point overflow in F32 multiplication".to_string())
                } else {
                    Ok(Value::F32(result))
                }
            }
            (Value::F64(a), Value::F64(b)) => {
                let result = *a * *b;
                if result.is_infinite() && !a.is_infinite() && !b.is_infinite() {
                    Err("Floating point overflow in F64 multiplication".to_string())
                } else {
                    Ok(Value::F64(result))
                }
            }
            _ => Err("Cannot multiply these types".to_string()),
        }
    }

    fn divide(&self, other: &Self) -> Result<Value, String> {
        match (self, other) {
            (Value::I32(a), Value::I32(b)) => {
                if *b == 0 {
                    return Err("Division by zero".to_string());
                }
                if *a == i32::MIN && *b == -1 {
                    return Err("Integer overflow in I32 division".to_string());
                }
                match a.checked_div(*b) {
                    Some(result) => Ok(Value::I32(result)),
                    None => Err("Integer division error".to_string()),
                }
            }
            (Value::I64(a), Value::I64(b)) => {
                if *b == 0 {
                    return Err("Division by zero".to_string());
                }
                if *a == i64::MIN && *b == -1 {
                    return Err("Integer overflow in I64 division".to_string());
                }
                match a.checked_div(*b) {
                    Some(result) => Ok(Value::I64(result)),
                    None => Err("Integer division error".to_string()),
                }
            }
            (Value::U32(a), Value::U32(b)) => {
                if *b == 0 {
                    return Err("Division by zero".to_string());
                }
                match a.checked_div(*b) {
                    Some(result) => Ok(Value::U32(result)),
                    None => Err("Integer division error".to_string()),
                }
            }
            (Value::U64(a), Value::U64(b)) => {
                if *b == 0 {
                    return Err("Division by zero".to_string());
                }
                match a.checked_div(*b) {
                    Some(result) => Ok(Value::U64(result)),
                    None => Err("Integer division error".to_string()),
                }
            }
            (Value::F32(a), Value::F32(b)) => {
                if *b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                let result = *a / *b;
                if result.is_infinite() && !a.is_infinite() {
                    Err("Floating point overflow in F32 division".to_string())
                } else {
                    Ok(Value::F32(result))
                }
            }
            (Value::F64(a), Value::F64(b)) => {
                if *b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                let result = *a / *b;
                if result.is_infinite() && !a.is_infinite() {
                    Err("Floating point overflow in F64 division".to_string())
                } else {
                    Ok(Value::F64(result))
                }
            }
            _ => Err("Cannot divide these types".to_string()),
        }
    }

    fn negate(&self) -> Result<Value, String> {
        match self {
            Value::I32(i) => {
                if *i == i32::MIN {
                    return Err("Integer overflow in I32 negation".to_string());
                }
                Ok(Value::I32(-i))
            }
            Value::I64(i) => {
                if *i == i64::MIN {
                    return Err("Integer overflow in I64 negation".to_string());
                }
                Ok(Value::I64(-i))
            }
            Value::U32(_) => Err("Cannot negate unsigned integer U32".to_string()),
            Value::U64(_) => Err("Cannot negate unsigned integer U64".to_string()),
            Value::F32(f) => Ok(Value::F32(-f)),
            Value::F64(f) => Ok(Value::F64(-f)),
            _ => Err("Can only negate numbers".to_string()),
        }
    }
}
