use slang_error::Location;
use slang_ir::ExprFactory;
use slang_ir::ast::{BinaryOperator, Expression, LiteralValue, Statement, UnaryOperator};
use slang_types::TypeId;

fn location(position: usize) -> Location {
    Location::new(position, 1, position + 1, 1)
}

fn unspecified_int(position: usize) -> Expression {
    literal(LiteralValue::UnspecifiedInteger(1), position)
}

fn literal<T: slang_ir::IntoLiteralValue>(value: T, position: usize) -> Expression {
    Expression::Literal(ExprFactory::literal_expr_with_location(
        value,
        location(position),
    ))
}

#[test]
fn literal_infers_type_from_value() {
    assert_eq!(literal(1i32, 0).expr_type(), TypeId::i32());
    assert_eq!(literal(1i64, 0).expr_type(), TypeId::i64());
    assert_eq!(literal(1u32, 0).expr_type(), TypeId::u32());
    assert_eq!(literal(1u64, 0).expr_type(), TypeId::u64());
    assert_eq!(literal(1.0f32, 0).expr_type(), TypeId::f32());
    assert_eq!(literal(1.0f64, 0).expr_type(), TypeId::f64());
    assert_eq!(literal(true, 0).expr_type(), TypeId::bool());
    assert_eq!(literal("text", 0).expr_type(), TypeId::string());
    assert_eq!(
        literal(String::from("text"), 0).expr_type(),
        TypeId::string()
    );
    assert_eq!(literal((), 0).expr_type(), TypeId::unit());
}

#[test]
fn literal_from_literal_value() {
    let expr =
        ExprFactory::literal_expr_with_location(LiteralValue::UnspecifiedInteger(5), location(2));

    assert_eq!(expr.expr_type, TypeId::unspecified_int());
    assert_eq!(expr.location.position, 2);
}

#[test]
fn variable_expression() {
    let expr = ExprFactory::variable_expr_with_location("x", location(3));

    assert_eq!(expr.name, "x");
    assert_eq!(expr.location.position, 3);
}

#[test]
fn call_expression_type_is_resolved_later() {
    let callee = Expression::Variable(ExprFactory::variable_expr_with_location("f", location(0)));

    let call = ExprFactory::call_expr_with_location(callee, vec![literal(1i32, 2)], location(0));

    assert_eq!(call.arguments.len(), 1);
    assert_eq!(call.expr_type, TypeId::unknown());
}

#[test]
fn binary_arithmetic_promotes_unspecified_literals() {
    let expr = ExprFactory::binary(unspecified_int(0), BinaryOperator::Add, literal(2i64, 4));

    assert_eq!(expr.expr_type(), TypeId::i64());
    let location = expr.location();
    assert_eq!(location.position, 0);
}

#[test]
fn binary_arithmetic_of_incompatible_types_is_unknown() {
    let expr = ExprFactory::binary_expr(
        literal(1i32, 0),
        BinaryOperator::Multiply,
        literal("text", 4),
    );

    assert_eq!(expr.expr_type, TypeId::unknown());
}

#[test]
fn binary_comparison_and_logic_are_boolean() {
    let operators = [
        BinaryOperator::GreaterThan,
        BinaryOperator::LessThan,
        BinaryOperator::GreaterThanOrEqual,
        BinaryOperator::LessThanOrEqual,
        BinaryOperator::Equal,
        BinaryOperator::NotEqual,
        BinaryOperator::And,
        BinaryOperator::Or,
    ];

    for operator in operators {
        let expr = ExprFactory::binary_expr(literal(1i32, 0), operator, literal(2i32, 4));
        assert_eq!(expr.expr_type, TypeId::bool());
    }
}

#[test]
fn binary_expr_with_explicit_location() {
    let expr = ExprFactory::binary_expr_with_location(
        literal(1i32, 0),
        BinaryOperator::Subtract,
        literal(2i32, 4),
        location(9),
    );

    assert_eq!(expr.location.position, 9);
    assert_eq!(expr.expr_type, TypeId::i32());
}

#[test]
fn unary_negate_keeps_operand_type() {
    let expr = ExprFactory::unary(UnaryOperator::Negate, literal(1.5f64, 1));

    assert_eq!(expr.expr_type(), TypeId::f64());
    assert_eq!(expr.location().position, 1);
}

#[test]
fn unary_not_is_boolean() {
    let expr = ExprFactory::unary_expr(UnaryOperator::Not, literal(true, 1));

    assert_eq!(expr.expr_type, TypeId::bool());
}

#[test]
fn unary_expr_with_explicit_location() {
    let expr =
        ExprFactory::unary_expr_with_location(UnaryOperator::Negate, literal(1i32, 1), location(7));

    assert_eq!(expr.expr_type, TypeId::i32());
    assert_eq!(expr.location.position, 7);
}

#[test]
fn conditional_with_same_branch_types() {
    let expr = ExprFactory::conditional_expr(literal(true, 0), literal(1i32, 5), literal(2i32, 9));

    assert_eq!(expr.expr_type, TypeId::i32());
    assert_eq!(expr.location.position, 0);
}

#[test]
fn conditional_promotes_unspecified_literal_branch() {
    let expr =
        ExprFactory::conditional_expr(literal(true, 0), unspecified_int(5), literal(2i64, 9));

    assert_eq!(expr.expr_type, TypeId::i64());
}

#[test]
fn conditional_with_incompatible_branches_is_unknown() {
    let expr = ExprFactory::conditional_expr(literal(true, 0), literal(1i32, 5), literal("x", 9));

    assert_eq!(expr.expr_type, TypeId::unknown());
}

#[test]
fn block_with_return_expression_takes_its_type() {
    let expr = ExprFactory::block(vec![], Some(literal(1i32, 2)));

    assert_eq!(expr.expr_type(), TypeId::i32());
}

#[test]
fn block_without_return_expression_is_unit() {
    let expr = ExprFactory::block_expr(vec![], None);

    assert_eq!(expr.expr_type, TypeId::unit());
    assert!(expr.return_expr.is_none());
}

#[test]
fn block_with_statements_spans_them() {
    let statement = Statement::Expression(literal(1i32, 3));

    let expr = ExprFactory::block_expr(vec![statement], Some(literal(2i32, 8)));

    assert_eq!(expr.statements.len(), 1);
    assert_eq!(expr.expr_type, TypeId::i32());
}

#[test]
fn function_type_expressions_are_resolved_later() {
    let params = vec![TypeId::i32(), TypeId::bool()];

    let wrapped =
        ExprFactory::function_type_with_location(params.clone(), TypeId::string(), location(1));
    let raw = ExprFactory::function_type_expr_with_location(params, TypeId::string(), location(1));

    assert_eq!(wrapped.expr_type(), TypeId::unknown());
    assert_eq!(raw.param_types.len(), 2);
    assert_eq!(raw.return_type, TypeId::string());
    assert_eq!(raw.expr_type, TypeId::unknown());
}
