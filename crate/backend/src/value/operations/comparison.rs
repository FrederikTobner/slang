use super::super::Value;

/// Comparison operations on values
pub trait ComparisonOps {
    /// Tests if two values are equal.
    ///
    /// ### Arguments
    /// * `other` - The other value to compare
    ///
    /// ### Returns
    /// * The result of the equality comparison
    /// * An error message if the types cannot be compared
    fn equal(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;

    /// Tests if two values are not equal.
    ///
    /// ### Arguments
    /// * `other` - The other value to compare
    ///
    /// ### Returns
    /// * The result of the inequality comparison
    /// * An error message if the types cannot be compared
    fn not_equal(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;

    /// Tests if one value is less than another.
    ///
    /// ### Arguments
    /// * `other` - The other value to compare
    ///
    /// ### Returns
    /// * The result of the less-than comparison
    /// * An error message if the types cannot be compared
    fn less_than(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;

    /// Tests if one value is less than or equal to another.
    ///
    /// ### Arguments
    /// * `other` - The other value to compare
    ///
    /// ### Returns
    /// * The result of the less-than-or-equal comparison
    /// * An error message if the types cannot be compared
    fn less_than_equal(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;

    /// Tests if one value is greater than another.
    ///
    /// ### Arguments
    /// * `other` - The other value to compare
    ///
    /// ### Returns
    /// * The result of the greater-than comparison
    /// * An error message if the types cannot be compared
    fn greater_than(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;

    /// Tests if one value is greater than or equal to another.
    ///
    /// ### Arguments
    /// * `other` - The other value to compare
    ///
    /// ### Returns
    /// * The result of the greater-than-or-equal comparison
    /// * An error message if the types cannot be compared
    fn greater_than_equal(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;
}

impl ComparisonOps for Value {
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
