use crate::value::Value;

/// Built-in function to print a value
///
/// ### Arguments
///
/// * `args` - Arguments to the function (should be exactly 1)
///
/// ### Returns
///
/// Success with i32(0) if successful, or an error message
pub fn print_value(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("print_value expects exactly 1 argument".to_string());
    }

    println!("{}", args[0]);

    // Return 0 to indicate success
    Ok(Value::I32(0))
}
