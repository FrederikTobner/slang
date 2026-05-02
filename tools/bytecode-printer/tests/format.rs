use bytecode_printer_lib::format::BytecodeFormat;

#[test]
fn from_str() {
    assert!(matches!(
        "pretty".parse::<BytecodeFormat>().unwrap(),
        BytecodeFormat::Pretty
    ));
    assert!(matches!(
        "debug".parse::<BytecodeFormat>().unwrap(),
        BytecodeFormat::Debug
    ));
    assert!(matches!(
        "json".parse::<BytecodeFormat>().unwrap(),
        BytecodeFormat::Json
    ));

    assert!("invalid".parse::<BytecodeFormat>().is_err());
}

#[test]
fn to_string() {
    assert_eq!(BytecodeFormat::Pretty.to_string(), "pretty");
    assert_eq!(BytecodeFormat::Debug.to_string(), "debug");
    assert_eq!(BytecodeFormat::Json.to_string(), "json");
}
