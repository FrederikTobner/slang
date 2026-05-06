/// Tests for chained call expressions: `expr(args)(more_args)`.
///
/// These tests define the expected behaviour of the new `postfix` parsing
/// rule.  They will fail until the refactor described in
/// `docs/call-expression-chaining.md` is implemented.
use crate::ErrorCode;
use crate::assertions::ProgramAssertion;

#[test]
fn call_result_called_immediately_no_args() {
    // get_getter()() — both the outer and inner calls take no arguments.
    let program = r#"
        fn get_value() -> i32 {
            return 42;
        }

        fn get_getter() -> fn() -> i32 {
            return get_value;
        }

        print_value(get_getter()());
    "#;

    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn call_result_called_with_argument() {
    // get_adder()(32) — the outer call returns a function, which is then
    // applied to an argument.
    let program = r#"
        fn add_ten(x: i32) -> i32 {
            return x + 10;
        }

        fn get_adder() -> fn(i32) -> i32 {
            return add_ten;
        }

        print_value(get_adder()(32));
    "#;

    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn chained_call_result_assigned_to_variable() {
    let program = r#"
        fn increment(x: i32) -> i32 {
            return x + 1;
        }

        fn get_incrementer() -> fn(i32) -> i32 {
            return increment;
        }

        let result = get_incrementer()(41);
        print_value(result);
    "#;

    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn chained_call_in_arithmetic_expression() {
    // Two independent chained calls combined with +.
    let program = r#"
        fn identity(x: i32) -> i32 {
            return x;
        }

        fn get_identity() -> fn(i32) -> i32 {
            return identity;
        }

        print_value(get_identity()(20) + get_identity()(22));
    "#;

    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn chained_call_as_if_condition() {
    let program = r#"
        fn always_true() -> bool {
            return true;
        }

        fn get_predicate() -> fn() -> bool {
            return always_true;
        }

        if get_predicate()() {
            print_value(42);
        }
    "#;

    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn chained_call_passed_as_argument() {
    // The chained call result is itself an argument to another function.
    let program = r#"
        fn identity(x: i32) -> i32 {
            return x;
        }

        fn get_identity() -> fn(i32) -> i32 {
            return identity;
        }

        fn double(x: i32) -> i32 {
            return x * 2;
        }

        print_value(double(get_identity()(21)));
    "#;

    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn outer_call_receives_argument_and_returns_function() {
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }

        fn make_pipeline(offset: i32) -> fn(i32, i32) -> i32 {
            return add;
        }

        print_value(make_pipeline(0)(20, 22));
    "#;

    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn chained_call_wrong_argument_count() {
    // The inner call receives too many arguments.
    let program = r#"
        fn identity(x: i32) -> i32 {
            return x;
        }

        fn get_identity() -> fn(i32) -> i32 {
            return identity;
        }

        get_identity()(1, 2);
    "#;

    ProgramAssertion::new(program)
        .fails()
        .stderr("Called expression expects 1 arguments, but got 2")
        .diagnostic_snippet(
            10,
            r#"
            |        get_identity()(1, 2);
            |        ^^^^^^^^^^^^^^^^^^^^
            "#,
        )
        .error_code(ErrorCode::ArgumentCountMismatch);
}

#[test]
fn chained_call_wrong_argument_type() {
    // The inner call receives an argument of the wrong type.
    let program = r#"
        fn identity(x: i32) -> i32 {
            return x;
        }

        fn get_identity() -> fn(i32) -> i32 {
            return identity;
        }

        get_identity()("not an int");
    "#;

    ProgramAssertion::new(program)
        .fails()
        .stderr("Type mismatch: Called expression expects argument 1 to be i32, but got string")
        .diagnostic_snippet(
            10,
            r#"
            |        get_identity()("not an int");
            |                       ^^^^^^^^^^^^
            "#,
        )
        .error_code(ErrorCode::ArgumentTypeMismatch);
}

#[test]
fn calling_non_callable_expression_result_errors() {
    // get_number() returns i32, so get_number()() must fail at the second
    // call site.  A new or existing "not callable" error code should fire.
    let program = r#"
        fn get_number() -> i32 {
            return 42;
        }

        get_number()();
    "#;

    ProgramAssertion::new(program)
        .fails()
        .stderr("Expression result is not callable")
        .diagnostic_snippet(
            6,
            r#"
            |        get_number()();
            |        ^^^^^^^^^^^^^^
            "#,
        )
        .error_code(ErrorCode::InvalidExpression);
}
