use bytecode_printer_lib::format::BytecodeFormat;
use slang_backend::bytecode::{Chunk, OpCode};
use slang_backend::value::Value;
use slang_compilation_pipeline::ChainPipeline;
use slang_compilation_pipeline::SlangSourceFile;
use slang_compilation_pipeline::result::CompilationResult;

const PROGRAM: &str = r#"
fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

let mut x: i32 = 5;
let name = "slang";
let ratio = 1.5;
let ok = true && !false || x > 3;
x = x + factorial(3) - 2 / 1;
{
    let inner = -x;
    print_value(inner);
}
print_value(name);
print_value(ratio);
print_value(ok);
"#;

fn compile(source: &str) -> Chunk {
    colored::control::set_override(false);
    let source_file = SlangSourceFile::for_tooling("test.sl", source.to_string());
    match ChainPipeline::full_compilation().execute(source_file) {
        CompilationResult::Success { output, .. } => output,
        CompilationResult::Failed { .. } => panic!("Test program should compile"),
    }
}

fn format_with(format: BytecodeFormat, chunk: &Chunk) -> String {
    colored::control::set_override(false);
    format
        .create_formatter()
        .format(chunk, "test")
        .expect("Formatting should succeed")
}

fn chunk_with_invalid_operands() -> Chunk {
    let mut chunk = Chunk::new();
    chunk.add_constant(Value::I32(1));
    chunk.add_identifier("known".to_string());
    chunk.write_op(OpCode::Constant, 1);
    chunk.write_byte(9, 1); // constant index out of range
    chunk.write_op(OpCode::GetVariable, 2);
    chunk.write_byte(9, 2); // identifier index out of range
    chunk.write_op(OpCode::Jump, 3);
    chunk.write_byte(0, 3);
    chunk.write_byte(0, 3);
    chunk.write_byte(250, 4); // not a valid opcode
    chunk.write_op(OpCode::Return, 5);
    chunk
}

mod pretty {
    use super::*;

    #[test]
    fn compiled_program() {
        // Arrange
        let chunk = compile(PROGRAM);

        // Act
        let output = format_with(BytecodeFormat::Pretty, &chunk);

        // Assert
        assert!(output.contains("test"));
        assert!(output.contains("Constants"));
        assert!(output.contains("Identifiers"));
        assert!(output.contains("factorial"));
        assert!(output.contains("Jump Target"));
    }

    #[test]
    fn empty_chunk() {
        // Arrange
        let chunk = Chunk::new();

        // Act
        let output = format_with(BytecodeFormat::Pretty, &chunk);

        // Assert
        assert!(output.contains("test"));
        assert!(!output.contains("Constants"));
        assert!(!output.contains("Identifiers"));
    }

    #[test]
    fn invalid_operands_and_unknown_opcode() {
        // Arrange
        let chunk = chunk_with_invalid_operands();

        // Act
        let output = format_with(BytecodeFormat::Pretty, &chunk);

        // Assert
        assert!(output.contains("UNKNOWN"));
        assert!(output.contains("<?"));
    }
}

mod debug {
    use super::*;

    #[test]
    fn compiled_program() {
        // Arrange
        let chunk = compile(PROGRAM);

        // Act
        let output = format_with(BytecodeFormat::Debug, &chunk);

        // Assert
        assert!(output.contains("BYTECODE CHUNK: test"));
        assert!(output.contains(&format!("Code size: {} bytes", chunk.code.len())));
        assert!(output.contains("RAW BYTECODE (HEX)"));
    }

    #[test]
    fn invalid_operands_and_unknown_opcode() {
        // Arrange
        let chunk = chunk_with_invalid_operands();

        // Act
        let output = format_with(BytecodeFormat::Debug, &chunk);

        // Assert
        assert!(output.contains("BYTECODE CHUNK: test"));
        assert!(output.contains("fa"));
    }
}

mod json {
    use super::*;

    fn parse(chunk: &Chunk) -> serde_json::Value {
        let output = format_with(BytecodeFormat::Json, chunk);
        serde_json::from_str(&output).expect("Output should be valid JSON")
    }

    #[test]
    fn compiled_program() {
        // Arrange
        let chunk = compile(PROGRAM);

        // Act
        let json = parse(&chunk);

        // Assert
        assert_eq!(json["name"], "test");
        assert_eq!(json["statistics"]["code_size"], chunk.code.len());
        assert_eq!(json["statistics"]["constants_count"], chunk.constants.len());
        assert!(!json["instructions"].as_array().unwrap().is_empty());
        let types: Vec<&str> = json["constants"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["type"].as_str().unwrap())
            .collect();
        assert!(types.contains(&"i64"));
        assert!(types.contains(&"string"));
        assert!(types.contains(&"f64"));
        assert!(types.contains(&"boolean"));
        assert!(types.contains(&"function"));
        assert!(
            json["identifiers"]
                .as_array()
                .unwrap()
                .iter()
                .any(|i| i["name"] == "factorial")
        );
    }

    #[test]
    fn invalid_operands_and_unknown_opcode() {
        // Arrange
        let chunk = chunk_with_invalid_operands();

        // Act
        let json = parse(&chunk);

        // Assert
        let instructions = json["instructions"].as_array().unwrap();
        assert_eq!(instructions[0]["value"], "<??>");
        assert_eq!(instructions[1]["identifier"], "<?>");
        assert_eq!(instructions[2]["opcode"], "JUMP");
        assert_eq!(instructions[3]["opcode"], "UNKNOWN");
        assert_eq!(instructions[3]["raw_byte"], 250);
        assert_eq!(instructions[4]["opcode"], "RETURN");
    }

    #[test]
    fn empty_chunk() {
        // Arrange
        let chunk = Chunk::new();

        // Act
        let json = parse(&chunk);

        // Assert
        assert!(json["instructions"].as_array().unwrap().is_empty());
        assert_eq!(json["statistics"]["code_size"], 0);
    }
}
