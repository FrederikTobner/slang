use slang_derive::NamedEnum;

// Test enum with a mix of explicit and implicit names
#[derive(Debug, PartialEq, Clone, Copy, NamedEnum)]
enum TestEnum {
    #[name = "custom_first"]
    First,
    #[name = "UPPERCASE_SECOND"]
    Second,
    Third, // Should implicitly use "third" as the name
    #[name = "custom-fourth-with-hyphens"]
    Fourth,
    #[name = ""]
    EmptyName,
}

// Test enum with complex names and special characters
#[derive(Debug, PartialEq, Clone, Copy, NamedEnum)]
enum SpecialNameEnum {
    #[name = "special.name"]
    WithDot,
    #[name = "snake_case_name"]
    SnakeCase,
    #[name = "camelCaseName"]
    CamelCase,
    #[name = "name with spaces"]
    WithSpaces,
    #[name = "name-with-hyphens"]
    WithHyphens,
    #[name = "123_numeric_prefix"]
    NumericPrefix,
    #[name = "unicode_☺_char"]
    WithUnicode,
}

// Test enum to verify case-sensitivity in from_str
#[derive(Debug, PartialEq, Clone, Copy, NamedEnum)]
enum CaseSensitivityEnum {
    #[name = "lowercase"]
    Lowercase,
    #[name = "UPPERCASE"]
    Uppercase,
    #[name = "MixedCase"]
    MixedCase,
}

#[test]
fn test_explicit_names() {
    // Test explicitly named variants
    assert_eq!(TestEnum::First.name(), "custom_first");
    assert_eq!(TestEnum::Second.name(), "UPPERCASE_SECOND");
    assert_eq!(TestEnum::Fourth.name(), "custom-fourth-with-hyphens");
    assert_eq!(TestEnum::EmptyName.name(), "");
}

#[test]
fn test_implicit_names() {
    // Test implicitly named variant
    assert_eq!(TestEnum::Third.name(), "third");
}

#[test]
fn test_from_str_with_explicit_names() {
    // Test from_str for explicitly named variants
    assert_eq!(TestEnum::from_str("custom_first"), Some(TestEnum::First));
    assert_eq!(TestEnum::from_str("UPPERCASE_SECOND"), Some(TestEnum::Second));
    assert_eq!(TestEnum::from_str("custom-fourth-with-hyphens"), Some(TestEnum::Fourth));
    assert_eq!(TestEnum::from_str(""), Some(TestEnum::EmptyName));
}

#[test]
fn test_from_str_with_implicit_names() {
    // Test from_str for implicitly named variant
    assert_eq!(TestEnum::from_str("third"), Some(TestEnum::Third));
}

#[test]
fn test_from_str_with_invalid_names() {
    // Test from_str with invalid names
    assert_eq!(TestEnum::from_str("not_a_valid_name"), None);
    assert_eq!(TestEnum::from_str("First"), None); // Original variant name, not lowercase
    assert_eq!(TestEnum::from_str("first"), None); // Not the explicit name
}

#[test]
fn test_case_sensitivity() {
    // Verify that case sensitivity is preserved in names
    assert_eq!(CaseSensitivityEnum::Lowercase.name(), "lowercase");
    assert_eq!(CaseSensitivityEnum::Uppercase.name(), "UPPERCASE");
    assert_eq!(CaseSensitivityEnum::MixedCase.name(), "MixedCase");
    
    // Verify that from_str is case-sensitive
    assert_eq!(CaseSensitivityEnum::from_str("lowercase"), Some(CaseSensitivityEnum::Lowercase));
    assert_eq!(CaseSensitivityEnum::from_str("UPPERCASE"), Some(CaseSensitivityEnum::Uppercase));
    assert_eq!(CaseSensitivityEnum::from_str("MixedCase"), Some(CaseSensitivityEnum::MixedCase));
    
    // Verify that incorrect case returns None
    assert_eq!(CaseSensitivityEnum::from_str("LOWERCASE"), None);
    assert_eq!(CaseSensitivityEnum::from_str("uppercase"), None);
    assert_eq!(CaseSensitivityEnum::from_str("mixedcase"), None);
}

#[test]
fn test_special_characters() {
    // Test names with special characters
    assert_eq!(SpecialNameEnum::WithDot.name(), "special.name");
    assert_eq!(SpecialNameEnum::SnakeCase.name(), "snake_case_name");
    assert_eq!(SpecialNameEnum::CamelCase.name(), "camelCaseName");
    assert_eq!(SpecialNameEnum::WithSpaces.name(), "name with spaces");
    assert_eq!(SpecialNameEnum::WithHyphens.name(), "name-with-hyphens");
    assert_eq!(SpecialNameEnum::NumericPrefix.name(), "123_numeric_prefix");
    assert_eq!(SpecialNameEnum::WithUnicode.name(), "unicode_☺_char");
    
    // Test from_str with special characters
    assert_eq!(SpecialNameEnum::from_str("special.name"), Some(SpecialNameEnum::WithDot));
    assert_eq!(SpecialNameEnum::from_str("snake_case_name"), Some(SpecialNameEnum::SnakeCase));
    assert_eq!(SpecialNameEnum::from_str("camelCaseName"), Some(SpecialNameEnum::CamelCase));
    assert_eq!(SpecialNameEnum::from_str("name with spaces"), Some(SpecialNameEnum::WithSpaces));
    assert_eq!(SpecialNameEnum::from_str("name-with-hyphens"), Some(SpecialNameEnum::WithHyphens));
    assert_eq!(SpecialNameEnum::from_str("123_numeric_prefix"), Some(SpecialNameEnum::NumericPrefix));
    assert_eq!(SpecialNameEnum::from_str("unicode_☺_char"), Some(SpecialNameEnum::WithUnicode));
}

#[test]
fn test_roundtrip_conversion() {
    // Test roundtrip conversion from enum -> name -> enum
    let variants = [
        TestEnum::First,
        TestEnum::Second,
        TestEnum::Third,
        TestEnum::Fourth,
        TestEnum::EmptyName,
    ];
    
    for variant in variants.iter() {
        let name = variant.name();
        let roundtrip = TestEnum::from_str(name);
        assert_eq!(roundtrip, Some(*variant));
    }
    
    // Same test for special names
    let special_variants = [
        SpecialNameEnum::WithDot,
        SpecialNameEnum::SnakeCase,
        SpecialNameEnum::CamelCase,
        SpecialNameEnum::WithSpaces,
        SpecialNameEnum::WithHyphens,
        SpecialNameEnum::NumericPrefix,
        SpecialNameEnum::WithUnicode,
    ];
    
    for variant in special_variants.iter() {
        let name = variant.name();
        let roundtrip = SpecialNameEnum::from_str(name);
        assert_eq!(roundtrip, Some(*variant));
    }
}

// Test enum mixing unit variants and variants with data
#[derive(Debug, PartialEq, Clone, NamedEnum)]
enum MixedEnum {
    #[name = "first_unit"]
    FirstUnit,
    #[name = "second_unit"]
    SecondUnit,
    #[name = "third_unit"]
    ThirdUnit,
}

#[test]
fn test_unit_variants_only() {
    // Test name() method
    assert_eq!(MixedEnum::FirstUnit.name(), "first_unit");
    assert_eq!(MixedEnum::SecondUnit.name(), "second_unit");
    assert_eq!(MixedEnum::ThirdUnit.name(), "third_unit");
    
    // Test from_str method
    assert_eq!(MixedEnum::from_str("first_unit"), Some(MixedEnum::FirstUnit));
    assert_eq!(MixedEnum::from_str("second_unit"), Some(MixedEnum::SecondUnit));
    assert_eq!(MixedEnum::from_str("third_unit"), Some(MixedEnum::ThirdUnit));
    
    // Test invalid string
    assert_eq!(MixedEnum::from_str("invalid"), None);
}