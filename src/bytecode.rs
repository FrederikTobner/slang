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
    
    // New function-related opcodes
    DefineFunction = 11, // Define a function
    Call = 12,          // Call a function
    JumpIfFalse = 13,   // Conditional jump
    Jump = 14,          // Unconditional jump
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
            11 => Some(OpCode::DefineFunction),
            12 => Some(OpCode::Call),
            13 => Some(OpCode::JumpIfFalse),
            14 => Some(OpCode::Jump),
            _ => None,
        }
    }
}

// Function representation in bytecode
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arity: u8,         // Number of parameters
    pub code_offset: usize, // Offset in the chunk where this function's code begins
    pub locals: Vec<String>, // Local variable names
}

// Native function type
pub type NativeFn = fn(&[Value]) -> Result<Value, String>;

// Native function representation
#[derive(Clone)]
pub struct NativeFunction {
    pub name: String,
    pub arity: u8,
    pub function: NativeFn,
}

impl std::fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn {}>", self.name)
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F64(f64),
    String(String),
    Function(Function), // Add function value type
    NativeFunction(NativeFunction), // Add native function value type
}

impl Value {
    pub fn type_tag(&self) -> u8 {
        match self {
            Value::I32(_) => 0,
            Value::I64(_) => 1,
            Value::U32(_) => 2,
            Value::U64(_) => 3,
            Value::String(_) => 4,
            Value::F64(_) => 5,
            Value::Function(_) => 6,
            Value::NativeFunction(_) => 7,
        }
    }

    pub fn deserialize_from_type_tag(type_tag: u8, reader: &mut dyn Read) -> std::io::Result<Self> {
        match type_tag {
            // I32 
            0 => {
                let mut bytes = [0u8; 4];
                reader.read_exact(&mut bytes)?;
                Ok(Value::I32(i32::from_le_bytes(bytes)))
            }
            // I64
            1 => {
                let mut bytes = [0u8; 8];
                reader.read_exact(&mut bytes)?;
                Ok(Value::I64(i64::from_le_bytes(bytes)))
            }
            // U32
            2 => {
                let mut bytes = [0u8; 4];
                reader.read_exact(&mut bytes)?;
                Ok(Value::U32(u32::from_le_bytes(bytes)))
            }
            // U64
            3 => {
                let mut bytes = [0u8; 8];
                reader.read_exact(&mut bytes)?;
                Ok(Value::U64(u64::from_le_bytes(bytes)))
            }
            // String
            4 => {
                let mut len_bytes = [0u8; 4];
                reader.read_exact(&mut len_bytes)?;
                let len = u32::from_le_bytes(len_bytes) as usize;

                let mut string_bytes = vec![0u8; len];
                reader.read_exact(&mut string_bytes)?;
                String::from_utf8(string_bytes)
                    .map(Value::String)
                    .map_err(|_| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8")
                    })
            }
            // F64
            5 => {
                let mut bytes = [0u8; 8];
                reader.read_exact(&mut bytes)?;
                Ok(Value::F64(f64::from_le_bytes(bytes)))
            }
            // Function
            6 => {
                let mut name_len_bytes = [0u8; 4];
                reader.read_exact(&mut name_len_bytes)?;
                let name_len = u32::from_le_bytes(name_len_bytes) as usize;

                let mut name_bytes = vec![0u8; name_len];
                reader.read_exact(&mut name_bytes)?;
                let name = String::from_utf8(name_bytes).map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8")
                })?;

                let mut arity_bytes = [0u8; 1];
                reader.read_exact(&mut arity_bytes)?;
                let arity = arity_bytes[0];

                let mut code_offset_bytes = [0u8; 4];
                reader.read_exact(&mut code_offset_bytes)?;
                let code_offset = u32::from_le_bytes(code_offset_bytes) as usize;

                let mut locals_len_bytes = [0u8; 4];
                reader.read_exact(&mut locals_len_bytes)?;
                let locals_len = u32::from_le_bytes(locals_len_bytes) as usize;

                let mut locals = Vec::new();
                for _ in 0..locals_len {
                    let mut local_len_bytes = [0u8; 4];
                    reader.read_exact(&mut local_len_bytes)?;
                    let local_len = u32::from_le_bytes(local_len_bytes) as usize;

                    let mut local_bytes = vec![0u8; local_len];
                    reader.read_exact(&mut local_bytes)?;
                    let local = String::from_utf8(local_bytes).map_err(|_| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8")
                    })?;
                    locals.push(local);
                }

                Ok(Value::Function(Function {
                    name,
                    arity,
                    code_offset,
                    locals,
                }))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid value type tag",
            )),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::I32(i) => write!(f, "{}", i),
            Value::I64(i) => write!(f, "{}", i),
            Value::U32(i) => write!(f, "{}", i),
            Value::U64(i) => write!(f, "{}", i),
            Value::F64(flo) => write!(f, "{}", flo),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Function(func) => write!(f, "<fn {}>", func.name),
            Value::NativeFunction(func) => write!(f, "<native fn {}>", func.name),
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

    #[allow(dead_code)]
    pub fn deserialize(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut chunk = Chunk::new();

        let mut code_len_bytes = [0u8; 4];
        reader.read_exact(&mut code_len_bytes)?;
        let code_len = u32::from_le_bytes(code_len_bytes) as usize;

        let mut code = vec![0u8; code_len];
        reader.read_exact(&mut code)?;
        chunk.code = code;

        // Initialize lines (will be filled with dummy values)
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

    // Add methods for disassembling and printing bytecode
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }
    
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        
        // Add line info
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }
        
        let instruction = self.code[offset];
        match OpCode::from_u8(instruction) {
            Some(OpCode::Constant) => {
                self.simple_instruction_with_operand("CONSTANT", offset)
            },
            Some(OpCode::Add) => {
                self.simple_instruction("ADD", offset)
            },
            Some(OpCode::Subtract) => {
                self.simple_instruction("SUBTRACT", offset)
            },
            Some(OpCode::Multiply) => {
                self.simple_instruction("MULTIPLY", offset)
            },
            Some(OpCode::Divide) => {
                self.simple_instruction("DIVIDE", offset)
            },
            Some(OpCode::Negate) => {
                self.simple_instruction("NEGATE", offset)
            },
            Some(OpCode::Return) => {
                self.simple_instruction("RETURN", offset)
            },
            Some(OpCode::Print) => {
                self.simple_instruction("PRINT", offset)
            },
            Some(OpCode::GetVariable) => {
                self.variable_instruction("GET_VARIABLE", offset)
            },
            Some(OpCode::SetVariable) => {
                self.variable_instruction("SET_VARIABLE", offset)
            },
            Some(OpCode::Pop) => {
                self.simple_instruction("POP", offset)
            },
            Some(OpCode::DefineFunction) => {
                self.variable_instruction("DEFINE_FUNCTION", offset)
            },
            Some(OpCode::Call) => {
                let arg_count = self.code[offset + 1];
                println!("{:<16} {:4} args", "CALL", arg_count);
                offset + 2 // Instruction plus 1-byte operand
            },
            Some(OpCode::JumpIfFalse) => {
                let jump_offset = ((self.code[offset + 1] as usize) << 8) | (self.code[offset + 2] as usize);
                println!("{:<16} {:4} -> {}", "JUMP_IF_FALSE", offset, offset + 3 + jump_offset);
                offset + 3 // Instruction plus 2-byte operand
            },
            Some(OpCode::Jump) => {
                let jump_offset = ((self.code[offset + 1] as usize) << 8) | (self.code[offset + 2] as usize);
                println!("{:<16} {:4} -> {}", "JUMP", offset, offset + 3 + jump_offset);
                offset + 3 // Instruction plus 2-byte operand
            },
            None => {
                println!("Unknown opcode: {}", instruction);
                offset + 1
            },
        }
    }
    
    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }
    
    fn simple_instruction_with_operand(&self, name: &str, offset: usize) -> usize {
        let constant_index = self.code[offset + 1];
        println!("{:<16} {:4} '{}'", name, constant_index, self.constants[constant_index as usize]);
        offset + 2 // Instruction plus 1-byte operand
    }
    
    fn variable_instruction(&self, name: &str, offset: usize) -> usize {
        let var_index = self.code[offset + 1];
        println!("{:<16} {:4} '{}'", name, var_index, self.identifiers[var_index as usize]);
        offset + 2 // Instruction plus 1-byte operand
    }
}
