use slang_derive::IterableEnum;

#[derive(Debug, PartialEq, Eq, IterableEnum)]
enum TestPrimitiveType {
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
    Bool,
    String,
    UnspecifiedInt,
    UnspecifiedFloat,
    Unknown,
}

#[test]
fn test_iterable_enum_functionality() {
    let all_types: Vec<TestPrimitiveType> = TestPrimitiveType::iter().collect();

    let expected_count = 11;
    assert_eq!(all_types.len(), expected_count);

    assert!(all_types.contains(&TestPrimitiveType::I32));
    assert!(all_types.contains(&TestPrimitiveType::I64));
    assert!(all_types.contains(&TestPrimitiveType::U32));
    assert!(all_types.contains(&TestPrimitiveType::U64));
    assert!(all_types.contains(&TestPrimitiveType::F32));
    assert!(all_types.contains(&TestPrimitiveType::F64));
    assert!(all_types.contains(&TestPrimitiveType::Bool));
    assert!(all_types.contains(&TestPrimitiveType::String));
    assert!(all_types.contains(&TestPrimitiveType::UnspecifiedInt));
    assert!(all_types.contains(&TestPrimitiveType::UnspecifiedFloat));
    assert!(all_types.contains(&TestPrimitiveType::Unknown));

    println!(
        "IterableEnum test passed! Found {} primitive types.",
        all_types.len()
    );
}

#[test]
fn test_iterator_is_repeatable() {
    let first_iteration: Vec<TestPrimitiveType> = TestPrimitiveType::iter().collect();
    let second_iteration: Vec<TestPrimitiveType> = TestPrimitiveType::iter().collect();

    assert_eq!(first_iteration, second_iteration);
    assert_eq!(first_iteration.len(), 11);
}

#[test]
fn test_iterator_order_is_consistent() {
    let types: Vec<TestPrimitiveType> = TestPrimitiveType::iter().collect();

    assert_eq!(types[0], TestPrimitiveType::I32);
    assert_eq!(types[1], TestPrimitiveType::I64);
    assert_eq!(types[2], TestPrimitiveType::U32);
    assert_eq!(types[3], TestPrimitiveType::U64);
    assert_eq!(types[4], TestPrimitiveType::F32);
    assert_eq!(types[5], TestPrimitiveType::F64);
    assert_eq!(types[6], TestPrimitiveType::Bool);
    assert_eq!(types[7], TestPrimitiveType::String);
    assert_eq!(types[8], TestPrimitiveType::UnspecifiedInt);
    assert_eq!(types[9], TestPrimitiveType::UnspecifiedFloat);
    assert_eq!(types[10], TestPrimitiveType::Unknown);
}

#[test]
fn test_iterable_enum_with_single_variant() {
    #[derive(Debug, Copy, Clone, PartialEq, IterableEnum)]
    enum SingleVariant {
        Only,
    }

    let variants: Vec<SingleVariant> = SingleVariant::iter().collect();
    assert_eq!(variants.len(), 1);
    assert_eq!(variants[0], SingleVariant::Only);
}

#[test]
fn test_iterable_enum_with_two_variants() {
    #[derive(Debug, Copy, Clone, PartialEq, IterableEnum)]
    enum TwoVariants {
        First,
        Second,
    }

    let variants: Vec<TwoVariants> = TwoVariants::iter().collect();
    assert_eq!(variants.len(), 2);
    assert_eq!(variants[0], TwoVariants::First);
    assert_eq!(variants[1], TwoVariants::Second);
}

#[test]
fn test_iterator_is_cloneable() {
    let iter1 = TestPrimitiveType::iter();
    let iter2 = iter1.clone();

    let vec1: Vec<TestPrimitiveType> = iter1.collect();
    let vec2: Vec<TestPrimitiveType> = iter2.collect();

    assert_eq!(vec1, vec2);
    assert_eq!(vec1.len(), 11);
}
