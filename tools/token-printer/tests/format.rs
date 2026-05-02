use token_printer_lib::format::TokenFormat;

#[test]
fn from_str() {
    assert!(matches!(
        "pretty".parse::<TokenFormat>().unwrap(),
        TokenFormat::Pretty
    ));
    assert!(matches!(
        "debug".parse::<TokenFormat>().unwrap(),
        TokenFormat::Debug
    ));
    assert!("invalid".parse::<TokenFormat>().is_err());
}

#[test]
fn to_string() {
    assert_eq!(TokenFormat::Pretty.to_string(), "pretty");
    assert_eq!(TokenFormat::Debug.to_string(), "debug");
}
