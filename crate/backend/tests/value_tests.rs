use slang_backend::bytecode::Chunk;
use slang_backend::value::{Function, NativeFunction, Value};
use std::io::Cursor;

fn function(name: &str, code_offset: usize) -> Value {
    Value::Function(Box::new(Function {
        name: name.to_string(),
        arity: 2,
        code_offset,
        locals: vec!["a".to_string(), "b".to_string()],
    }))
}

fn native_function(name: &str) -> Value {
    Value::NativeFunction(Box::new(NativeFunction {
        name: name.to_string(),
        arity: 1,
        function: |_| Ok(Value::Unit(())),
    }))
}

fn all_values() -> Vec<Value> {
    vec![
        Value::I32(-5),
        Value::I64(-50),
        Value::U32(5),
        Value::U64(50),
        Value::F32(1.5),
        Value::F64(2.5),
        Value::Boolean(true),
        Value::String(Box::new("hello".to_string())),
        function("add", 7),
        native_function("print_value"),
        Value::Unit(()),
    ]
}

fn round_trip(chunk: &Chunk) -> Chunk {
    let mut buffer = Vec::new();
    chunk.serialize(&mut buffer).unwrap();
    Chunk::deserialize(&mut Cursor::new(buffer)).unwrap()
}

#[test]
fn display() {
    let displayed: Vec<String> = all_values().iter().map(|v| v.to_string()).collect();

    assert_eq!(
        displayed,
        vec![
            "-5",
            "-50",
            "5",
            "50",
            "1.5",
            "2.5",
            "true",
            "hello",
            "<fn add>",
            "<native fn print_value>",
            "()"
        ]
    );
}

#[test]
fn type_tags_are_unique() {
    let mut tags: Vec<u8> = all_values().iter().map(Value::type_tag).collect();
    tags.sort_unstable();
    tags.dedup();

    assert_eq!(tags.len(), all_values().len());
}

#[test]
fn type_predicates() {
    assert!(Value::I32(1).is_numeric());
    assert!(Value::F64(1.0).is_numeric());
    assert!(!Value::Boolean(true).is_numeric());
    assert!(!Value::String(Box::default()).is_numeric());

    assert!(Value::U64(1).is_integer());
    assert!(!Value::F32(1.0).is_integer());

    assert!(Value::F32(1.0).is_float());
    assert!(Value::F64(1.0).is_float());
    assert!(!Value::I32(1).is_float());

    assert!(Value::I64(1).is_signed_integer());
    assert!(!Value::U32(1).is_signed_integer());

    assert!(Value::U32(1).is_unsigned_integer());
    assert!(!Value::I32(1).is_unsigned_integer());

    assert!(Value::String(Box::default()).is_string());
    assert!(!Value::Unit(()).is_string());

    assert!(Value::Boolean(false).is_boolean());
    assert!(!Value::I32(0).is_boolean());
}

#[test]
fn debug_of_native_function() {
    let debug = format!("{:?}", native_function("print_value"));

    assert!(debug.contains("print_value"));
}

#[test]
fn chunk_round_trip_preserves_all_value_types() {
    // Arrange
    let mut chunk = Chunk::new();
    for value in all_values() {
        chunk.add_constant(value);
    }
    chunk.add_identifier("x".to_string());
    chunk.write_byte(1, 1);
    chunk.write_byte(2, 1);

    // Act
    let restored = round_trip(&chunk);

    // Assert
    assert_eq!(restored.code, chunk.code);
    assert_eq!(restored.identifiers, chunk.identifiers);
    let original: Vec<String> = chunk.constants.iter().map(|c| c.to_string()).collect();
    let loaded: Vec<String> = restored.constants.iter().map(|c| c.to_string()).collect();
    assert_eq!(loaded, original);
    let tags: Vec<u8> = restored.constants.iter().map(Value::type_tag).collect();
    let expected: Vec<u8> = chunk.constants.iter().map(Value::type_tag).collect();
    assert_eq!(tags, expected);
}

#[test]
fn function_round_trip_preserves_fields() {
    // Arrange
    let mut chunk = Chunk::new();
    chunk.add_constant(function("add", 7));

    // Act
    let restored = round_trip(&chunk);

    // Assert
    match &restored.constants[0] {
        Value::Function(f) => {
            assert_eq!(f.name, "add");
            assert_eq!(f.arity, 2);
            assert_eq!(f.code_offset, 7);
            assert_eq!(f.locals, vec!["a".to_string(), "b".to_string()]);
        }
        other => panic!("Expected a function, got {other}"),
    }
}

#[test]
fn deserialized_native_function_is_a_placeholder() {
    // Arrange
    let mut chunk = Chunk::new();
    chunk.add_constant(native_function("print_value"));

    // Act
    let restored = round_trip(&chunk);

    // Assert
    match &restored.constants[0] {
        Value::NativeFunction(f) => {
            assert_eq!(f.name, "print_value");
            assert_eq!(f.arity, 1);
            assert!((f.function)(&[]).is_err());
        }
        other => panic!("Expected a native function, got {other}"),
    }
}

#[test]
fn deserialize_from_type_tag_reads_primitives() {
    let value = Value::deserialize_from_type_tag(1, &mut Cursor::new(42i64.to_le_bytes()));

    assert_eq!(value.unwrap().to_string(), "42");
}

#[test]
fn deserialize_from_type_tag_unit_reads_nothing() {
    let value = Value::deserialize_from_type_tag(10, &mut Cursor::new(Vec::<u8>::new()));

    assert_eq!(value.unwrap().to_string(), "()");
}

#[test]
fn deserialize_from_invalid_type_tag() {
    let error = Value::deserialize_from_type_tag(200, &mut Cursor::new(Vec::<u8>::new()))
        .expect_err("Tag 200 is not a value type");

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert!(error.to_string().contains("200"));
}

#[test]
fn deserialize_string_with_invalid_utf8() {
    let mut bytes = 2u32.to_le_bytes().to_vec();
    bytes.extend_from_slice(&[0xff, 0xfe]);

    let error = Value::deserialize_from_type_tag(4, &mut Cursor::new(bytes))
        .expect_err("Invalid UTF-8 should be rejected");

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
}

#[test]
fn deserialize_function_with_invalid_utf8_name() {
    let mut bytes = 1u32.to_le_bytes().to_vec();
    bytes.push(0xff);

    let error = Value::deserialize_from_type_tag(6, &mut Cursor::new(bytes))
        .expect_err("Invalid UTF-8 should be rejected");

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
}

#[test]
fn deserialize_truncated_data() {
    let error = Value::deserialize_from_type_tag(0, &mut Cursor::new(vec![1u8, 2]))
        .expect_err("Truncated data should be rejected");

    assert_eq!(error.kind(), std::io::ErrorKind::UnexpectedEof);
}

#[test]
fn chunk_deserialize_truncated_data() {
    let mut chunk = Chunk::new();
    chunk.add_constant(Value::I64(1));
    let mut buffer = Vec::new();
    chunk.serialize(&mut buffer).unwrap();
    buffer.truncate(buffer.len() - 3);

    assert!(Chunk::deserialize(&mut Cursor::new(buffer)).is_err());
}
