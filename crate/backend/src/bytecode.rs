use crate::value::Value;
use slang_derive::NumericEnum;
pub use std::io::{Read, Write};

/// Operation codes for the bytecode interpreter
#[derive(Debug, PartialEq, NumericEnum)]
pub enum OpCode {
    /// Push a constant onto the stack
    Constant,
    /// Add the top two stack values
    Add,
    /// Subtract the top stack value from the second stack value
    Subtract,
    /// Multiply the top two stack values
    Multiply,
    /// Divide the second stack value by the top stack value
    Divide,
    /// Negate the top stack value
    Negate,
    /// Return from the current function
    Return,
    /// Print the top stack value
    Print,
    /// Push the value of a variable onto the stack
    GetVariable,
    /// Set a variable to the top stack value
    SetVariable,
    /// Remove the top stack value
    Pop,
    /// Define a function
    DefineFunction,
    /// Call a function
    Call,
    /// Jump if the top stack value is false
    JumpIfFalse,
    /// Jump unconditionally
    Jump,
    /// Negate a boolean value (logical NOT)
    BoolNot,
    /// Boolean AND operation
    BoolAnd,
    /// Boolean OR operation
    BoolOr,
    /// Greater than comparison
    Greater,
    /// Less than comparison
    Less,
    /// Greater than or equal comparison
    GreaterEqual,
    /// Less than or equal comparison
    LessEqual,
    /// Equal comparison
    Equal,
    /// Not equal comparison
    NotEqual,
    /// Begin a new scope (save variable state)
    BeginScope,
    /// End the current scope (restore variable state)
    EndScope,
}


/// A chunk of bytecode representing a compiled program
#[derive(Debug)]
pub struct Chunk {
    /// The actual bytecode instructions
    pub code: Vec<u8>,
    /// Constant values used by the program
    pub constants: Vec<Value>,
    /// Source code line numbers for debugging
    pub lines: Vec<usize>,
    /// Variable and function names used in the program
    pub identifiers: Vec<String>,
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk::new()
    }
}

impl Chunk {
    /// Creates a new, empty bytecode chunk
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
            identifiers: Vec::new(),
        }
    }

    /// Writes a byte to the chunk
    ///
    /// ### Arguments
    ///
    /// * `byte` - The byte to write
    /// * `line` - The source code line number
    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    /// Writes an opcode to the chunk
    ///
    /// ### Arguments
    ///
    /// * `op` - The opcode to write
    /// * `line` - The source code line number
    pub fn write_op(&mut self, op: OpCode, line: usize) {
        self.write_byte(op as u8, line);
    }

    /// Adds a constant to the chunk's constant pool
    ///
    /// ### Arguments
    ///
    /// * `value` - The constant value to add
    ///
    /// ### Returns
    ///
    /// The index of the constant in the constant pool
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    /// Adds an identifier to the chunk's identifier pool
    ///
    /// ### Arguments
    ///
    /// * `name` - The identifier name to add
    ///
    /// ### Returns
    ///
    /// The index of the identifier in the identifier pool
    pub fn add_identifier(&mut self, name: String) -> usize {
        for (i, id) in self.identifiers.iter().enumerate() {
            if id == &name {
                return i;
            }
        }
        self.identifiers.push(name);
        self.identifiers.len() - 1
    }

    /// Serializes the chunk to binary data
    ///
    /// ### Arguments
    ///
    /// * `writer` - The writer to write the binary data to
    ///
    /// ### Returns
    ///
    /// IO result indicating success or failure
    pub fn serialize(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        let code_len = self.code.len() as u32;
        writer.write_all(&code_len.to_le_bytes())?;
        writer.write_all(&self.code)?;

        let constants_len = self.constants.len() as u32;
        writer.write_all(&constants_len.to_le_bytes())?;

        for value in &self.constants {
            writer.write_all(&[value.type_tag()])?;
            match value {
                Value::I32(i) => {
                    writer.write_all(&i.to_le_bytes())?;
                }
                Value::I64(i) => {
                    writer.write_all(&i.to_le_bytes())?;
                }
                Value::U32(i) => {
                    writer.write_all(&i.to_le_bytes())?;
                }
                Value::U64(i) => {
                    writer.write_all(&i.to_le_bytes())?;
                }
                Value::String(s) => {
                    let bytes = s.as_bytes();
                    let len = bytes.len() as u32;
                    writer.write_all(&len.to_le_bytes())?;
                    writer.write_all(bytes)?;
                }
                Value::F32(f) => {
                    writer.write_all(&f.to_le_bytes())?;
                }
                Value::F64(f) => {
                    writer.write_all(&f.to_le_bytes())?;
                }
                Value::Function(func) => {
                    let name_bytes = func.name.as_bytes();
                    let name_len = name_bytes.len() as u32;
                    writer.write_all(&name_len.to_le_bytes())?;
                    writer.write_all(name_bytes)?;

                    writer.write_all(&[func.arity])?;
                    writer.write_all(&(func.code_offset as u32).to_le_bytes())?;

                    let locals_len = func.locals.len() as u32;
                    writer.write_all(&locals_len.to_le_bytes())?;
                    for local in &func.locals {
                        let local_bytes = local.as_bytes();
                        let local_len = local_bytes.len() as u32;
                        writer.write_all(&local_len.to_le_bytes())?;
                        writer.write_all(local_bytes)?;
                    }
                }
                Value::NativeFunction(func) => {
                    let name_bytes = func.name.as_bytes();
                    let name_len = name_bytes.len() as u32;
                    writer.write_all(&name_len.to_le_bytes())?;
                    writer.write_all(name_bytes)?;

                    writer.write_all(&[func.arity])?;
                }
                Value::Boolean(b) => {
                    writer.write_all(&[*b as u8])?;
                }
                Value::Unit(_) => {}
            }
        }

        let identifiers_len = self.identifiers.len() as u32;
        writer.write_all(&identifiers_len.to_le_bytes())?;

        for id in &self.identifiers {
            let bytes = id.as_bytes();
            let len = bytes.len() as u32;
            writer.write_all(&len.to_le_bytes())?;
            writer.write_all(bytes)?;
        }

        Ok(())
    }

    /// Deserializes a chunk from binary data
    ///
    /// ### Arguments
    ///
    /// * `reader` - The reader to read the binary data from
    ///
    /// ### Returns
    ///
    /// The deserialized chunk or an IO error
    pub fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut chunk = Chunk::new();

        let mut code_len_bytes = [0u8; 4];
        reader.read_exact(&mut code_len_bytes)?;
        let code_len = u32::from_le_bytes(code_len_bytes) as usize;

        let mut code = vec![0u8; code_len];
        reader.read_exact(&mut code)?;
        chunk.code = code;

        chunk.lines = vec![0; code_len];

        let mut constants_len_bytes = [0u8; 4];
        reader.read_exact(&mut constants_len_bytes)?;
        let constants_len = u32::from_le_bytes(constants_len_bytes) as usize;

        for _ in 0..constants_len {
            let mut type_tag = [0u8; 1];
            reader.read_exact(&mut type_tag)?;

            let value = Value::deserialize_from_type_tag(type_tag[0], reader)?;
            chunk.constants.push(value);
        }

        let mut identifiers_len_bytes = [0u8; 4];
        reader.read_exact(&mut identifiers_len_bytes)?;
        let identifiers_len = u32::from_le_bytes(identifiers_len_bytes) as usize;

        for _ in 0..identifiers_len {
            let mut len_bytes = [0u8; 4];
            reader.read_exact(&mut len_bytes)?;
            let len = u32::from_le_bytes(len_bytes) as usize;

            let mut string_bytes = vec![0u8; len];
            reader.read_exact(&mut string_bytes)?;
            let string = String::from_utf8(string_bytes).map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8")
            })?;

            chunk.identifiers.push(string);
        }

        Ok(chunk)
    }
}
