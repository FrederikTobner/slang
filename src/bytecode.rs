pub use std::fmt;
pub use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    Constant = 0,    // Push constant from constant pool
    Add = 1,         // Add top two values on stack
    Subtract = 2,    // Subtract top two values on stack
    Multiply = 3,    // Multiply top two values on stack
    Divide = 4,      // Divide top two values on stack
    Negate = 5,      // Negate top value on stack
    Return = 6,      // Return from current function
    Print = 7,       // Print top value on stack
    GetVariable = 8, // Get variable value
    SetVariable = 9, // Set variable value
    Pop = 10,        // Pop top value from stack
}

impl OpCode {
    pub fn from_u8(value: u8) -> Option<OpCode> {
        match value {
            0 => Some(OpCode::Constant),
            1 => Some(OpCode::Add),
            2 => Some(OpCode::Subtract),
            3 => Some(OpCode::Multiply),
            4 => Some(OpCode::Divide),
            5 => Some(OpCode::Negate),
            6 => Some(OpCode::Return),
            7 => Some(OpCode::Print),
            8 => Some(OpCode::GetVariable),
            9 => Some(OpCode::SetVariable),
            10 => Some(OpCode::Pop),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::I32(i) => write!(f, "{}", i),
            Value::I64(i) => write!(f, "{}", i),
            Value::U32(i) => write!(f, "{}", i),
            Value::U64(i) => write!(f, "{}", i),
            Value::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
    pub identifiers: Vec<String>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
            identifiers: Vec::new(),
        }
    }

    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_op(&mut self, op: OpCode, line: usize) {
        self.write_byte(op as u8, line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn add_identifier(&mut self, name: String) -> usize {
        // Check if identifier already exists
        for (i, id) in self.identifiers.iter().enumerate() {
            if id == &name {
                return i;
            }
        }

        // If not, add it
        self.identifiers.push(name);
        self.identifiers.len() - 1
    }

    // Serialization methods
   #[allow(dead_code)]
    pub fn serialize(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        // Write code size and bytes
        let code_len = self.code.len() as u32;
        writer.write_all(&code_len.to_le_bytes())?;
        writer.write_all(&self.code)?;

        // Write constants count
        let constants_len = self.constants.len() as u32;
        writer.write_all(&constants_len.to_le_bytes())?;

        // Write each constant
        for value in &self.constants {
            match value {
                Value::I32(i) => {
                    writer.write_all(&[0])?; // Type tag: 0 for integer
                    writer.write_all(&i.to_le_bytes())?;
                }
                Value::I64(i) => {
                    writer.write_all(&[1])?; // Type tag: 0 for integer
                    writer.write_all(&i.to_le_bytes())?;
                }
                Value::U32(i) => {
                    writer.write_all(&[2])?; // Type tag: 0 for integer
                    writer.write_all(&i.to_le_bytes())?;
                }
                Value::U64(i) => {
                    writer.write_all(&[3])?; // Type tag: 0 for integer
                    writer.write_all(&i.to_le_bytes())?;
                }
                Value::String(s) => {
                    writer.write_all(&[4])?; // Type tag: 4 for string
                    let bytes = s.as_bytes();
                    let len = bytes.len() as u32;
                    writer.write_all(&len.to_le_bytes())?;
                    writer.write_all(bytes)?;
                }
            }
        }

        // Write identifiers count
        let identifiers_len = self.identifiers.len() as u32;
        writer.write_all(&identifiers_len.to_le_bytes())?;

        // Write each identifier
        for id in &self.identifiers {
            let bytes = id.as_bytes();
            let len = bytes.len() as u32;
            writer.write_all(&len.to_le_bytes())?;
            writer.write_all(bytes)?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut chunk = Chunk::new();

        // Read code size and bytes
        let mut code_len_bytes = [0u8; 4];
        reader.read_exact(&mut code_len_bytes)?;
        let code_len = u32::from_le_bytes(code_len_bytes) as usize;

        // Read code bytes
        let mut code = vec![0u8; code_len];
        reader.read_exact(&mut code)?;
        chunk.code = code;

        // Initialize lines (will be filled with dummy values)
        chunk.lines = vec![0; code_len];

        // Read constants count
        let mut constants_len_bytes = [0u8; 4];
        reader.read_exact(&mut constants_len_bytes)?;
        let constants_len = u32::from_le_bytes(constants_len_bytes) as usize;

        // Read each constant
        for _ in 0..constants_len {
            let mut type_tag = [0u8; 1];
            reader.read_exact(&mut type_tag)?;

            match type_tag[0] {
                0 => {
                    // Integer 64 bit
                    let mut int_bytes = [0u8; 8];
                    reader.read_exact(&mut int_bytes)?;
                    let value = i64::from_le_bytes(int_bytes);
                    chunk.constants.push(Value::I64(value));
                }
                1 => {
                    // Integer 32 bit
                    let mut int_bytes = [0u8; 4];
                    reader.read_exact(&mut int_bytes)?;
                    let value = i32::from_le_bytes(int_bytes);
                    chunk.constants.push(Value::I32(value));
                }
                2 => {
                    // Unsigned Integer 64 bit
                    let mut int_bytes = [0u8; 8];
                    reader.read_exact(&mut int_bytes)?;
                    let value = u64::from_le_bytes(int_bytes);
                    chunk.constants.push(Value::U64(value));
                }
                3 => {
                    // Unsigned Integer 32 bit
                    let mut int_bytes = [0u8; 4];
                    reader.read_exact(&mut int_bytes)?;
                    let value = u32::from_le_bytes(int_bytes);
                    chunk.constants.push(Value::U32(value));
                }
                4 => {
                    // String
                    let mut len_bytes = [0u8; 4];
                    reader.read_exact(&mut len_bytes)?;
                    let len = u32::from_le_bytes(len_bytes) as usize;

                    let mut string_bytes = vec![0u8; len];
                    reader.read_exact(&mut string_bytes)?;
                    let string = String::from_utf8(string_bytes).map_err(|_| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8")
                    })?;

                    chunk.constants.push(Value::String(string));
                }
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid value type",
                    ));
                }
            }
        }

        // Read identifiers count
        let mut identifiers_len_bytes = [0u8; 4];
        reader.read_exact(&mut identifiers_len_bytes)?;
        let identifiers_len = u32::from_le_bytes(identifiers_len_bytes) as usize;

        // Read each identifier
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
