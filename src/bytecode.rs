pub use std::fmt;
pub use std::io::{Read, Write};

/// Operation codes for the bytecode interpreter
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    /// Push a constant onto the stack
    Constant = 0,
    /// Add the top two stack values
    Add = 1,
    /// Subtract the top stack value from the second stack value
    Subtract = 2,
    /// Multiply the top two stack values
    Multiply = 3,
    /// Divide the second stack value by the top stack value
    Divide = 4,
    /// Negate the top stack value
    Negate = 5,
    /// Return from the current function
    Return = 6,
    /// Print the top stack value
    Print = 7,
    /// Push the value of a variable onto the stack
    GetVariable = 8,
    /// Set a variable to the top stack value
    SetVariable = 9,
    /// Remove the top stack value
    Pop = 10,
    /// Define a function
    DefineFunction = 11,
    /// Call a function
    Call = 12,
    /// Jump if the top stack value is false
    JumpIfFalse = 13,
    /// Jump unconditionally
    Jump = 14,
}

impl OpCode {
    /// Convert a byte to an OpCode
    /// 
    /// # Arguments
    /// 
    /// * `value` - The byte to convert
    /// 
    /// # Returns
    /// 
    /// Some(OpCode) if the byte represents a valid OpCode, None otherwise
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

/// Function representation in bytecode
#[derive(Debug, Clone)]
pub struct Function {
    /// Name of the function
    pub name: String,
    /// Number of parameters
    pub arity: u8,
    /// Offset in the chunk where this function's code begins
    pub code_offset: usize,
    /// Local variable names used by this function
    pub locals: Vec<String>,
}

/// Type for native function implementations
pub type NativeFn = fn(&[Value]) -> Result<Value, String>;

/// Native (built-in) function representation
#[derive(Clone)]
pub struct NativeFunction {
    /// Name of the native function
    pub name: String,
    /// Number of parameters
    pub arity: u8,
    /// The Rust function that implements this native function
    pub function: NativeFn,
}

impl std::fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn {}>", self.name)
    }
}

/// Values that can be stored in the bytecode and manipulated by the VM
#[derive(Debug, Clone)]
pub enum Value {
    /// 32-bit signed integer
    I32(i32),
    /// 64-bit signed integer
    I64(i64),
    /// 32-bit unsigned integer
    U32(u32),
    /// 64-bit unsigned integer
    U64(u64),
    /// 64-bit floating point
    F64(f64),
    /// String value
    String(String),
    /// Function value
    Function(Function),
    /// Native function value
    NativeFunction(NativeFunction),
}

impl Value {
    /// Returns a tag byte identifying this value's type
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

    /// Deserialize a value from a reader based on its type tag
    /// 
    /// # Arguments
    /// 
    /// * `type_tag` - The type tag of the value
    /// * `reader` - The reader to read the value data from
    /// 
    /// # Returns
    /// 
    /// The deserialized value or an IO error
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

/// A chunk of bytecode representing a compiled program
#[derive(Debug, Clone)]
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
    /// # Arguments
    /// 
    /// * `byte` - The byte to write
    /// * `line` - The source code line number
    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    /// Writes an opcode to the chunk
    /// 
    /// # Arguments
    /// 
    /// * `op` - The opcode to write
    /// * `line` - The source code line number
    pub fn write_op(&mut self, op: OpCode, line: usize) {
        self.write_byte(op as u8, line);
    }

    /// Adds a constant to the chunk's constant pool
    /// 
    /// # Arguments
    /// 
    /// * `value` - The constant value to add
    /// 
    /// # Returns
    /// 
    /// The index of the constant in the constant pool
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    /// Adds an identifier to the chunk's identifier pool
    /// 
    /// # Arguments
    /// 
    /// * `name` - The identifier name to add
    /// 
    /// # Returns
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
    /// # Arguments
    /// 
    /// * `writer` - The writer to write the binary data to
    /// 
    /// # Returns
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

    /// Deserializes a chunk from binary data
    /// 
    /// # Arguments
    /// 
    /// * `reader` - The reader to read the binary data from
    /// 
    /// # Returns
    /// 
    /// The deserialized chunk or an IO error
    #[allow(dead_code)]
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

    /// Debugging function to print the chunk's bytecode
    #[cfg(feature = "print-byte_code")]
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }
    
    /// Disassembles a single instruction for debugging
    #[cfg(feature = "print-byte_code")]
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
                offset + 2
            },
            Some(OpCode::JumpIfFalse) => {
                let jump_offset = ((self.code[offset + 1] as usize) << 8) | (self.code[offset + 2] as usize);
                println!("{:<16} {:4} -> {}", "JUMP_IF_FALSE", offset, offset + 3 + jump_offset);
                offset + 3
            },
            Some(OpCode::Jump) => {
                let jump_offset = ((self.code[offset + 1] as usize) << 8) | (self.code[offset + 2] as usize);
                println!("{:<16} {:4} -> {}", "JUMP", offset, offset + 3 + jump_offset);
                offset + 3
            },
            None => {
                println!("Unknown opcode: {}", instruction);
                offset + 1
            },
        }
    }
    
    /// Helper for disassembling simple instructions
    #[cfg(feature = "print-byte_code")]
    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }
    
    /// Helper for disassembling instructions with constant operands
    #[cfg(feature = "print-byte_code")]
    fn simple_instruction_with_operand(&self, name: &str, offset: usize) -> usize {
        let constant_index = self.code[offset + 1];
        println!("{:<16} {:4} '{}'", name, constant_index, self.constants[constant_index as usize]);
        offset + 2
    }
    
    /// Helper for disassembling instructions with variable operands
    #[cfg(feature = "print-byte_code")]
    fn variable_instruction(&self, name: &str, offset: usize) -> usize {
        let var_index = self.code[offset + 1];
        println!("{:<16} {:4} '{}'", name, var_index, self.identifiers[var_index as usize]);
        offset + 2
    }
}
