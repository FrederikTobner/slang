use super::super::Value;

/// Logical operations on values
pub trait LogicalOps {
    /// Performs logical NOT on a boolean value.
    ///
    /// ### Returns
    /// * The negated boolean value
    /// * An error message if the type is incompatible
    fn not(&self) -> Result<Self, String>
    where
        Self: Sized;

    /// Performs logical AND between two boolean values.
    ///
    /// ### Arguments
    /// * `other` - The other value
    ///
    /// ### Returns
    /// * The result of the AND operation
    /// * An error message if either value is not a boolean
    fn and(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;

    /// Performs logical OR between two boolean values.
    ///
    /// ### Arguments
    /// * `other` - The other value
    ///
    /// ### Returns
    /// * The result of the OR operation
    /// * An error message if either value is not a boolean
    fn or(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;
}

impl LogicalOps for Value {
    fn not(&self) -> Result<Value, String> {
        match self {
            Value::Boolean(b) => Ok(Value::Boolean(!b)),
            _ => Err("Logical NOT operator requires a boolean operand".to_string()),
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
}
