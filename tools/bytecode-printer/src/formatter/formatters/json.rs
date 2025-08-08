use slang_backend::bytecode::{Chunk, OpCode};
use serde_json::{json, Value as JsonValue};

/// JSON formatter for structured data exchange
pub struct JsonFormatter;

impl super::super::BytecodeFormatter for JsonFormatter {
    fn format(&self, chunk: &Chunk, name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut instructions = Vec::new();
        let mut offset = 0;

        while offset < chunk.code.len() {
            let instruction_byte = chunk.code[offset];
            let line = if offset < chunk.lines.len() { chunk.lines[offset] } else { 0 };
            
            match OpCode::from_int(instruction_byte) {
                Some(op) => {
                    let (instruction, new_offset) = self.instruction_to_json(chunk, offset, &op, line);
                    instructions.push(instruction);
                    offset = new_offset;
                }
                None => {
                    instructions.push(json!({
                        "offset": offset,
                        "line": line,
                        "opcode": "UNKNOWN",
                        "raw_byte": instruction_byte,
                    }));
                    offset += 1;
                }
            }
        }

        let constants: Vec<JsonValue> = chunk.constants.iter().enumerate()
            .map(|(i, c)| json!({
                "index": i,
                "value": format!("{c}"),
                "type": match c {
                    slang_backend::value::Value::I32(_) => "i32",
                    slang_backend::value::Value::I64(_) => "i64",
                    slang_backend::value::Value::U32(_) => "u32",
                    slang_backend::value::Value::U64(_) => "u64",
                    slang_backend::value::Value::F32(_) => "f32",
                    slang_backend::value::Value::F64(_) => "f64",
                    slang_backend::value::Value::String(_) => "string",
                    slang_backend::value::Value::Boolean(_) => "boolean",
                    slang_backend::value::Value::Function(_) => "function",
                    slang_backend::value::Value::NativeFunction(_) => "native_function",
                    slang_backend::value::Value::Unit(_) => "unit",
                }
            }))
            .collect();

        let identifiers: Vec<JsonValue> = chunk.identifiers.iter().enumerate()
            .map(|(i, id)| json!({
                "index": i,
                "name": id
            }))
            .collect();

        let result = json!({
            "name": name,
            "statistics": {
                "code_size": chunk.code.len(),
                "constants_count": chunk.constants.len(),
                "identifiers_count": chunk.identifiers.len(),
                "lines_count": chunk.lines.len()
            },
            "instructions": instructions,
            "constants": constants,
            "identifiers": identifiers
        });

        Ok(serde_json::to_string_pretty(&result)?)
    }
}

impl JsonFormatter {
    fn instruction_to_json(&self, chunk: &Chunk, offset: usize, op: &OpCode, line: usize) -> (JsonValue, usize) {
        let base = json!({
            "offset": offset,
            "line": line,
            "opcode": format!("{op:?}").to_uppercase(),
        });

        match op {
            OpCode::Constant => {
                let constant_index = chunk.code[offset + 1] as usize;
                let value = if constant_index < chunk.constants.len() {
                    format!("{constant}", constant = chunk.constants[constant_index])
                } else {
                    "<??>".to_string()
                };
                (json!({
                    "offset": offset,
                    "line": line,
                    "opcode": "CONSTANT",
                    "operand": constant_index,
                    "value": value
                }), offset + 2)
            }
            OpCode::GetVariable | OpCode::SetVariable => {
                let var_index = chunk.code[offset + 1] as usize;
                let name = if var_index < chunk.identifiers.len() {
                    &chunk.identifiers[var_index]
                } else {
                    "<?>"
                };
                (json!({
                    "offset": offset,
                    "line": line,
                    "opcode": format!("{op:?}").to_uppercase(),
                    "operand": var_index,
                    "identifier": name
                }), offset + 2)
            }
            OpCode::DefineFunction => {
                let var_index = chunk.code[offset + 1] as usize;
                let fn_constant_index = chunk.code[offset + 2] as usize;
                let name = if var_index < chunk.identifiers.len() {
                    &chunk.identifiers[var_index]
                } else {
                    "<?>"
                };
                let fn_info = if fn_constant_index < chunk.constants.len() {
                    format!("{fn_constant}", fn_constant = chunk.constants[fn_constant_index])
                } else {
                    "<??>".to_string()
                };
                (json!({
                    "offset": offset,
                    "line": line,
                    "opcode": "DEFINEFUNCTION",
                    "name_index": var_index,
                    "constant_index": fn_constant_index,
                    "identifier": name,
                    "function": fn_info
                }), offset + 3)
            }
            OpCode::Call => {
                let arg_count = chunk.code[offset + 1];
                (json!({
                    "offset": offset,
                    "line": line,
                    "opcode": "CALL",
                    "operand": arg_count,
                    "description": format!("{arg_count} arguments")
                }), offset + 2)
            }
            OpCode::JumpIfFalse | OpCode::Jump => {
                let jump_offset = ((chunk.code[offset + 1] as usize) << 8) | (chunk.code[offset + 2] as usize);
                let target = offset + 3 + jump_offset;
                (json!({
                    "offset": offset,
                    "line": line,
                    "opcode": format!("{op:?}").to_uppercase(),
                    "operand": jump_offset,
                    "target": target
                }), offset + 3)
            }
            _ => (base, offset + 1)
        }
    }
}
