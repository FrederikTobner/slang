/// Regression tests for basic call expression behaviour.
///
/// These cover the patterns that must continue working after the parser
/// refactor that moves call-expression parsing into a new `postfix` rule.
/// Any breakage here signals a regression in the refactor.
use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn simple_named_function_call() {
    // The most basic form: a bare identifier followed by argument list.
    let program = r#"
        fn get_value() -> i32 {
            return 42;
        }

        print_value(get_value());
    "#;

    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn calling_variable_with_function_type() {
    // A variable that holds a function value must still be callable.
    // This exercises the SymbolKind::Variable path in semantic analysis.
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }

        let f: fn(i32, i32) -> i32 = add;
        print_value(f(20, 22));
    "#;

    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn call_result_used_in_arithmetic() {
    // Call expressions must remain valid operands at every precedence level.
    let program = r#"
        fn twenty() -> i32 {
            return 20;
        }

        fn twenty_two() -> i32 {
            return 22;
        }

        print_value(twenty() + twenty_two());
    "#;

    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn call_result_with_unary_negation() {
    // The new `postfix` rule sits between `unary` and `primary`.
    // Unary operators must bind correctly around it.
    let program = r#"
        fn get_value() -> i32 {
            return 42;
        }

        print_value(-get_value());
    "#;

    ProgramAssertion::new(program).succeeds().stdout("-42");
}

#[test]
fn call_result_with_unary_not() {
    // Boolean negation applied to a call result.
    let program = r#"
        fn get_true() -> bool {
            return true;
        }

        print_value(!get_true());
    "#;

    ProgramAssertion::new(program).succeeds().stdout("false");
}

#[test]
fn call_as_if_condition() {
    // A call expression must be accepted wherever a boolean expression is.
    let program = r#"
        fn is_positive(x: i32) -> bool {
            return x > 0;
        }

        if is_positive(42) {
            print_value(1);
        }
    "#;

    ProgramAssertion::new(program).succeeds().stdout("1");
}

#[test]
fn call_result_assigned_to_variable() {
    let program = r#"
        fn compute() -> i32 {
            return 42;
        }

        let result = compute();
        print_value(result);
    "#;

    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn call_as_argument_to_another_call() {
    // Passing a call result as an argument (nested, not chained).
    let program = r#"
        fn double(x: i32) -> i32 {
            return x * 2;
        }

        fn increment(x: i32) -> i32 {
            return x + 1;
        }

        print_value(double(increment(20)));
    "#;

    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn call_result_in_multiplication() {
    // Ensures calls work at every binary operator level.
    let program = r#"
        fn six() -> i32 {
            return 6;
        }

        fn seven() -> i32 {
            return 7;
        }

        print_value(six() * seven());
    "#;

    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn call_result_in_comparison() {
    let program = r#"
        fn get_value() -> i32 {
            return 42;
        }

        print_value(get_value() == 42);
    "#;

    ProgramAssertion::new(program).succeeds().stdout("true");
}


#[test]
fn undefined_function_error() {
    let program = r#"
        let result = does_not_exist(1, 2);
    "#;

    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::UndefinedFunction)
        .stderr("Undefined function: does_not_exist");
}

