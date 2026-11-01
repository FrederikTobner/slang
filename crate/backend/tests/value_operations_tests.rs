use slang_backend::value::{ArithmeticOps, ComparisonOps, LogicalOps, Value};

fn show(result: Result<Value, String>) -> String {
    match result {
        Ok(value) => value.to_string(),
        Err(message) => format!("error: {message}"),
    }
}

fn text(value: &str) -> Value {
    Value::String(Box::new(value.to_string()))
}

#[test]
fn add_numbers() {
    assert_eq!(show(Value::I32(2).add(&Value::I32(3))), "5");
    assert_eq!(show(Value::I64(2).add(&Value::I64(3))), "5");
    assert_eq!(show(Value::U32(2).add(&Value::U32(3))), "5");
    assert_eq!(show(Value::U64(2).add(&Value::U64(3))), "5");
    assert_eq!(show(Value::F32(1.5).add(&Value::F32(1.0))), "2.5");
    assert_eq!(show(Value::F64(1.5).add(&Value::F64(1.0))), "2.5");
}

#[test]
fn add_strings() {
    assert_eq!(show(text("foo").add(&text("bar"))), "foobar");
}

#[test]
fn add_overflow() {
    assert!(show(Value::I32(i32::MAX).add(&Value::I32(1))).contains("overflow"));
    assert!(show(Value::I64(i64::MAX).add(&Value::I64(1))).contains("overflow"));
    assert!(show(Value::U32(u32::MAX).add(&Value::U32(1))).contains("overflow"));
    assert!(show(Value::U64(u64::MAX).add(&Value::U64(1))).contains("overflow"));
    assert!(show(Value::F32(f32::MAX).add(&Value::F32(f32::MAX))).contains("error"));
    assert!(show(Value::F64(f64::MAX).add(&Value::F64(f64::MAX))).contains("error"));
}

#[test]
fn add_mismatched_types() {
    assert_eq!(
        show(Value::I32(1).add(&Value::I64(1))),
        "error: Cannot add these types"
    );
    assert!(Value::Boolean(true).add(&Value::Boolean(true)).is_err());
}

#[test]
fn subtract_numbers() {
    assert_eq!(show(Value::I32(5).subtract(&Value::I32(3))), "2");
    assert_eq!(show(Value::I64(5).subtract(&Value::I64(3))), "2");
    assert_eq!(show(Value::U32(5).subtract(&Value::U32(3))), "2");
    assert_eq!(show(Value::U64(5).subtract(&Value::U64(3))), "2");
    assert_eq!(show(Value::F32(5.5).subtract(&Value::F32(3.0))), "2.5");
    assert_eq!(show(Value::F64(5.5).subtract(&Value::F64(3.0))), "2.5");
}

#[test]
fn subtract_underflow() {
    assert!(show(Value::I32(i32::MIN).subtract(&Value::I32(1))).contains("underflow"));
    assert!(show(Value::I64(i64::MIN).subtract(&Value::I64(1))).contains("underflow"));
    assert!(show(Value::U32(0).subtract(&Value::U32(1))).contains("underflow"));
    assert!(show(Value::U64(0).subtract(&Value::U64(1))).contains("underflow"));
    assert!(show(Value::F32(f32::MAX).subtract(&Value::F32(-f32::MAX))).contains("error"));
    assert!(show(Value::F64(f64::MAX).subtract(&Value::F64(-f64::MAX))).contains("error"));
}

#[test]
fn subtract_mismatched_types() {
    assert_eq!(
        show(text("a").subtract(&text("b"))),
        "error: Cannot subtract these types"
    );
}

#[test]
fn multiply_numbers() {
    assert_eq!(show(Value::I32(4).multiply(&Value::I32(3))), "12");
    assert_eq!(show(Value::I64(4).multiply(&Value::I64(3))), "12");
    assert_eq!(show(Value::U32(4).multiply(&Value::U32(3))), "12");
    assert_eq!(show(Value::U64(4).multiply(&Value::U64(3))), "12");
    assert_eq!(show(Value::F32(1.5).multiply(&Value::F32(2.0))), "3");
    assert_eq!(show(Value::F64(1.5).multiply(&Value::F64(2.0))), "3");
}

#[test]
fn multiply_overflow() {
    assert!(show(Value::I32(i32::MAX).multiply(&Value::I32(2))).contains("overflow"));
    assert!(show(Value::I64(i64::MAX).multiply(&Value::I64(2))).contains("overflow"));
    assert!(show(Value::U32(u32::MAX).multiply(&Value::U32(2))).contains("overflow"));
    assert!(show(Value::U64(u64::MAX).multiply(&Value::U64(2))).contains("overflow"));
    assert!(show(Value::F32(f32::MAX).multiply(&Value::F32(2.0))).contains("overflow"));
    assert!(show(Value::F64(f64::MAX).multiply(&Value::F64(2.0))).contains("overflow"));
}

#[test]
fn multiply_mismatched_types() {
    assert_eq!(
        show(Value::Boolean(true).multiply(&Value::Boolean(true))),
        "error: Cannot multiply these types"
    );
}

#[test]
fn divide_numbers() {
    assert_eq!(show(Value::I32(7).divide(&Value::I32(2))), "3");
    assert_eq!(show(Value::I64(7).divide(&Value::I64(2))), "3");
    assert_eq!(show(Value::U32(7).divide(&Value::U32(2))), "3");
    assert_eq!(show(Value::U64(7).divide(&Value::U64(2))), "3");
    assert_eq!(show(Value::F32(7.0).divide(&Value::F32(2.0))), "3.5");
    assert_eq!(show(Value::F64(7.0).divide(&Value::F64(2.0))), "3.5");
}

#[test]
fn divide_by_zero() {
    let zero_divisions = [
        Value::I32(1).divide(&Value::I32(0)),
        Value::I64(1).divide(&Value::I64(0)),
        Value::U32(1).divide(&Value::U32(0)),
        Value::U64(1).divide(&Value::U64(0)),
        Value::F32(1.0).divide(&Value::F32(0.0)),
        Value::F64(1.0).divide(&Value::F64(0.0)),
    ];

    for result in zero_divisions {
        assert_eq!(show(result), "error: Division by zero");
    }
}

#[test]
fn divide_overflow() {
    assert!(show(Value::I32(i32::MIN).divide(&Value::I32(-1))).contains("overflow"));
    assert!(show(Value::I64(i64::MIN).divide(&Value::I64(-1))).contains("overflow"));
    assert!(show(Value::F32(f32::MAX).divide(&Value::F32(0.5))).contains("overflow"));
    assert!(show(Value::F64(f64::MAX).divide(&Value::F64(0.5))).contains("overflow"));
}

#[test]
fn divide_mismatched_types() {
    assert_eq!(
        show(text("a").divide(&text("b"))),
        "error: Cannot divide these types"
    );
}

#[test]
fn negate_numbers() {
    assert_eq!(show(Value::I32(5).negate()), "-5");
    assert_eq!(show(Value::I64(5).negate()), "-5");
    assert_eq!(show(Value::F32(1.5).negate()), "-1.5");
    assert_eq!(show(Value::F64(1.5).negate()), "-1.5");
}

#[test]
fn negate_errors() {
    assert!(show(Value::I32(i32::MIN).negate()).contains("overflow"));
    assert!(show(Value::I64(i64::MIN).negate()).contains("overflow"));
    assert!(show(Value::U32(1).negate()).contains("unsigned"));
    assert!(show(Value::U64(1).negate()).contains("unsigned"));
    assert_eq!(
        show(Value::Boolean(true).negate()),
        "error: Can only negate numbers"
    );
}

fn comparison_results(left: Value, right: Value) -> Vec<String> {
    vec![
        show(left.equal(&right)),
        show(left.not_equal(&right)),
        show(left.less_than(&right)),
        show(left.less_than_equal(&right)),
        show(left.greater_than(&right)),
        show(left.greater_than_equal(&right)),
    ]
}

#[test]
fn compare_numbers() {
    let expected = ["false", "true", "true", "true", "false", "false"];

    for (left, right) in [
        (Value::I32(1), Value::I32(2)),
        (Value::I64(1), Value::I64(2)),
        (Value::U32(1), Value::U32(2)),
        (Value::U64(1), Value::U64(2)),
        (Value::F32(1.0), Value::F32(2.0)),
        (Value::F64(1.0), Value::F64(2.0)),
    ] {
        assert_eq!(comparison_results(left, right), expected);
    }
}

#[test]
fn compare_equal_numbers() {
    let expected = ["true", "false", "false", "true", "false", "true"];

    assert_eq!(comparison_results(Value::I32(2), Value::I32(2)), expected);
}

#[test]
fn equality_of_booleans_and_strings() {
    assert_eq!(
        show(Value::Boolean(true).equal(&Value::Boolean(true))),
        "true"
    );
    assert_eq!(
        show(Value::Boolean(true).not_equal(&Value::Boolean(true))),
        "false"
    );
    assert_eq!(show(text("a").equal(&text("a"))), "true");
    assert_eq!(show(text("a").not_equal(&text("b"))), "true");
}

#[test]
fn equality_of_functions() {
    use slang_backend::value::{Function, NativeFunction};

    let function = |name: &str, code_offset| {
        Value::Function(Box::new(Function {
            name: name.to_string(),
            arity: 0,
            code_offset,
            locals: vec![],
        }))
    };
    let native = |name: &str| {
        Value::NativeFunction(Box::new(NativeFunction {
            name: name.to_string(),
            arity: 0,
            function: |_| Ok(Value::Unit(())),
        }))
    };

    assert_eq!(show(function("f", 1).equal(&function("f", 1))), "true");
    assert_eq!(show(function("f", 1).equal(&function("f", 2))), "false");
    assert_eq!(show(function("f", 1).not_equal(&function("g", 1))), "true");
    assert_eq!(show(native("p").equal(&native("p"))), "true");
    assert_eq!(show(native("p").not_equal(&native("q"))), "true");
}

#[test]
fn compare_mismatched_types() {
    assert_eq!(
        show(Value::I32(1).equal(&Value::I64(1))),
        "error: Cannot compare these types with =="
    );
    assert_eq!(
        show(Value::I32(1).not_equal(&Value::I64(1))),
        "error: Cannot compare these types with !="
    );
    assert!(
        Value::Boolean(true)
            .less_than(&Value::Boolean(false))
            .is_err()
    );
    assert!(text("a").less_than_equal(&text("b")).is_err());
    assert!(text("a").greater_than(&text("b")).is_err());
    assert!(text("a").greater_than_equal(&text("b")).is_err());
}

#[test]
fn logical_operations() {
    let t = Value::Boolean(true);
    let f = Value::Boolean(false);

    assert_eq!(show(t.not()), "false");
    assert_eq!(show(t.and(&f)), "false");
    assert_eq!(show(t.and(&t)), "true");
    assert_eq!(show(f.or(&t)), "true");
    assert_eq!(show(f.or(&f)), "false");
}

#[test]
fn logical_operations_require_booleans() {
    assert!(Value::I32(1).not().is_err());
    assert!(Value::I32(1).and(&Value::Boolean(true)).is_err());
    assert!(Value::Boolean(true).or(&Value::I32(1)).is_err());
}
