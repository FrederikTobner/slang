use slang_derive::NumericEnum;

// Test that the NumericEnum derive macro works with both explicit and implicit values
#[derive(Debug, PartialEq, NumericEnum)]
enum TestEnum {
    Explicit1 = 10,
    Explicit2 = 20,
    Implicit1,     // 21
    Implicit2,     // 22
    Explicit3 = 30,
    Implicit3,     // 31
}

#[test]
fn test_explicit_values() {
    assert_eq!(TestEnum::from_int(10u8), Some(TestEnum::Explicit1));
    assert_eq!(TestEnum::from_int(20u16), Some(TestEnum::Explicit2));
    assert_eq!(TestEnum::from_int(30u8), Some(TestEnum::Explicit3));
}

#[test]
fn test_implicit_values() {
    assert_eq!(TestEnum::from_int(21u8), Some(TestEnum::Implicit1));
    assert_eq!(TestEnum::from_int(22u16), Some(TestEnum::Implicit2));
    assert_eq!(TestEnum::from_int(31u8), Some(TestEnum::Implicit3));
}

#[test]
fn test_invalid_values() {
    assert_eq!(TestEnum::from_int(0u8), None);
    assert_eq!(TestEnum::from_int(11u8), None);
    assert_eq!(TestEnum::from_int(255u8), None);
}
